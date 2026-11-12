<?php
// studio/api/files.php — «Мои проекты» AI-After Effects на вашем сервере. У каждого пользователя своя папка.
//
// Хранятся проекты (.ecproj — родной формат, .ecprojx — он же в XML, .aep/.aepx — проекты After Effects
// для открытия) и файлы, которые проекты используют: картинки, видео, звук, анимации Lottie (.json, .lottie).
// Отрендеренные ролики, GIF и секвенции кадров не хранятся — они скачиваются в браузере.
//
//   GET  files.php                          → {ok, user, files: [{name, size, modified, kind}], limit}
//   GET  files.php?action=load&name=…       → сам файл, application/octet-stream
//   POST files.php?action=save&name=…[&overwrite=1]   тело — байты файла → {ok, name, size, modified, kind}
//   POST files.php?action=rename  {name, to}   → {ok}
//   POST files.php?action=delete  {name}       → {ok}  (файл уходит в корзину .trash/ папки пользователя)
//   GET  files.php?action=fonts             → {ok, fonts: [{name, size}]} — шрифты организации
//   GET  files.php?action=font&name=…       → сам шрифт
//
// Большие файлы (видео) — до 2 ГБ. Чтобы сервер их принимал, нужны client_max_body_size 2048m; в nginx
// для api/files.php и post_max_size / upload_max_filesize не меньше 2G в PHP (см. README.ru.md).
//
// Шрифты организации: папка fonts рядом с папками пользователей (storage/ai-aftereffects/fonts). Положите
// туда шрифты, на которые у организации есть лицензия: AI-After Effects загрузит их у всех пользователей.
//
// Где лежат файлы: AI_AFTEREFFECTS_DATA_DIR из .env, иначе storage/ai-aftereffects рядом с корнем сайта
// (/app/storage/ai-aftereffects при корне /app/src — вне корня, nginx её не отдаёт; создаётся сама),
// потом data/ai-aftereffects, а если никуда нельзя писать — ai-aftereffects-data рядом с папкой приложения
// (закройте её в nginx: см. nginx.conf.example).
// Имена файлов на диске случайные, список — в index.json папки пользователя.

declare(strict_types=1);

header('Cache-Control: no-store, max-age=0');
header('X-Content-Type-Options: nosniff');

function files_fail(int $http, string $message, array $extra = []): void
{
    http_response_code($http);
    header('Content-Type: application/json; charset=utf-8');
    echo json_encode(['ok' => false, 'error' => $message] + $extra, JSON_UNESCAPED_UNICODE | JSON_UNESCAPED_SLASHES);
    exit;
}

function files_ok(array $data): void
{
    header('Content-Type: application/json; charset=utf-8');
    echo json_encode(['ok' => true] + $data, JSON_UNESCAPED_UNICODE | JSON_UNESCAPED_SLASHES);
    exit;
}

ini_set('display_errors', '0');
set_exception_handler(static function (Throwable $e): void {
    error_log('ai-aftereffects files: ' . $e->getMessage());
    files_fail(500, 'Внутренняя ошибка хранилища файлов. Подробности — в журнале PHP.');
});

require_once __DIR__ . '/llm.php';
$STUDIO = require __DIR__ . '/settings.php';

// ---------------------------------------------------------------- кто это
$user = 'local';
if ($STUDIO['auth_file'] !== '' && is_file($STUDIO['auth_file'])) {
    require_once $STUDIO['auth_file'];
    if (!function_exists('isLoggedIn') || !isLoggedIn()) {
        files_fail(401, 'Войдите в систему, чтобы открыть свои файлы.', ['login' => true]);
    }
    $u = function_exists('getCurrentUser') ? getCurrentUser() : null;
    $id = is_array($u) ? ($u['id'] ?? $u['login'] ?? null) : null;
    if ($id === null || $id === '') {
        files_fail(500, 'auth.php не сообщил, кто вошёл (getCurrentUser).');
    }
    $user = (string)$id;
} elseif (!$STUDIO['allow_anonymous']) {
    files_fail(500, 'Не найден auth.php системы. Укажите путь в AI_AFTEREFFECTS_AUTH_FILE (см. README.ru.md).');
}
if (session_status() === PHP_SESSION_ACTIVE) {
    session_write_close();
}

// ---------------------------------------------------------------- где лежат файлы
function files_root(): string
{
    $env = studio_env('AI_AFTEREFFECTS_DATA_DIR');
    if ($env !== '') {
        return rtrim($env, '/');
    }
    // Рядом с корнем сайта, но не внутри него: сначала папка storage (как /app/storage при корне
    // /app/src — туда же nginx пишет журналы), потом data. Берётся первая, куда PHP может писать.
    $docRoot = rtrim((string)($_SERVER['DOCUMENT_ROOT'] ?? ''), '/');
    $parent = $docRoot !== '' ? dirname($docRoot) : '';
    $candidates = [];
    if ($parent !== '' && $parent !== '/' && is_dir("$parent/storage")) {
        $candidates[] = "$parent/storage/ai-aftereffects";
    }
    if ($parent !== '' && $parent !== '/') {
        $candidates[] = "$parent/data/ai-aftereffects";
    }
    foreach ($candidates as $dir) {
        if ((is_dir($dir) && is_writable($dir)) || (!is_dir($dir) && @mkdir($dir, 0770, true))) {
            return $dir;
        }
    }
    return dirname(__DIR__, 2) . '/ai-aftereffects-data';
}

$root = files_root();
if (!is_dir($root) && !@mkdir($root, 0770, true) && !is_dir($root)) {
    files_fail(500, "Не удалось создать папку для файлов: $root. Создайте её и дайте PHP право записи (или укажите AI_AFTEREFFECTS_DATA_DIR).");
}
// Если папка всё же внутри сайта — закрываем её для Apache и не даём листать.
if (!is_file("$root/.htaccess")) {
    @file_put_contents("$root/.htaccess", "Require all denied\n");
}
if (!is_file("$root/index.html")) {
    @file_put_contents("$root/index.html", '');
}
$dir = $root . '/u-' . preg_replace('/[^A-Za-z0-9_-]/', '_', $user);
if (!is_dir($dir) && !@mkdir($dir, 0770, true) && !is_dir($dir)) {
    files_fail(500, 'Не удалось создать папку пользователя. Проверьте права PHP на ' . $root);
}

const FILES_MAX_BYTES = 2048 * 1024 * 1024;
const FILES_MAX_COUNT = 5000;
/** Проекты: .ecproj — родной формат (сохраняет всё), .ecprojx — он же в XML, .aep/.aepx — открыть. */
const FILES_PROJECTS = ['ecproj', 'ecprojx', 'aep', 'aepx'];
/** Файлы, которые проекты используют (их кладут в проект из «Моих проектов»). */
const FILES_MEDIA = [
    'png', 'jpg', 'jpeg', 'gif', 'webp', 'bmp', 'tif', 'tiff', 'psd', 'svg',
    'mp4', 'm4v', 'mov', 'webm', 'mkv', 'avi',
    'wav', 'mp3', 'm4a', 'aac', 'flac', 'ogg', 'opus', 'aif', 'aiff',
    'json', 'lottie',
];
define('FILES_EXTS', array_merge(FILES_PROJECTS, FILES_MEDIA));
define('FILES_STORED', '/^[0-9a-f]{32}\.(' . implode('|', FILES_EXTS) . ')$/');
const FONTS_EXTS = '/\.(ttf|otf|ttc|woff|woff2)$/i';
const FONTS_MAX_BYTES = 64 * 1024 * 1024;
const FONTS_MAX_COUNT = 200;

/** Проект или файл для проекта. */
function files_kind(string $name): string
{
    return in_array(files_ext($name), FILES_PROJECTS, true) ? 'project' : 'media';
}

/** Похоже ли начало файла ($head, первые 4 КБ) на файл с расширением $ext. */
function files_magic(string $ext, string $head): bool
{
    $at = static fn(int $o, string $s): bool => substr($head, $o, strlen($s)) === $s;
    $text = ltrim(preg_replace('/^\xEF\xBB\xBF/', '', $head) ?? '');
    $first = $text === '' ? '' : $text[0];
    $ftyp = $at(4, 'ftyp') || $at(4, 'moov') || $at(4, 'mdat') || $at(4, 'wide') || $at(4, 'free') || $at(4, 'skip');
    $mp3 = $at(0, 'ID3') || (strlen($head) > 1 && ord($head[0]) === 0xFF && (ord($head[1]) & 0xE0) === 0xE0);
    return match ($ext) {
        'ecproj', 'json' => $first === '{',
        'ecprojx', 'aepx' => $first === '<',
        'aep' => $at(0, 'RIFX'),
        'png' => $at(0, "\x89PNG"),
        'jpg', 'jpeg' => $at(0, "\xFF\xD8\xFF"),
        'gif' => $at(0, 'GIF8'),
        'webp' => $at(0, 'RIFF') && $at(8, 'WEBP'),
        'bmp' => $at(0, 'BM'),
        'tif', 'tiff' => $at(0, "II*\x00") || $at(0, "MM\x00*"),
        'psd' => $at(0, '8BPS'),
        'svg' => $first === '<' && stripos($head, '<svg') !== false,
        'mp4', 'm4v', 'mov', 'm4a' => $ftyp,
        'aac' => $ftyp || (strlen($head) > 1 && ord($head[0]) === 0xFF && (ord($head[1]) & 0xF6) === 0xF0),
        'webm', 'mkv' => $at(0, "\x1A\x45\xDF\xA3"),
        'avi' => $at(0, 'RIFF') && $at(8, 'AVI '),
        'wav' => ($at(0, 'RIFF') || $at(0, 'RF64')) && $at(8, 'WAVE'),
        'aif', 'aiff' => $at(0, 'FORM') && ($at(8, 'AIFF') || $at(8, 'AIFC')),
        'mp3' => $mp3,
        'flac' => $at(0, 'fLaC'),
        'ogg', 'opus' => $at(0, 'OggS'),
        'lottie' => $at(0, "PK\x03\x04"),
        default => false,
    };
}

/** Расширение документа по имени, в нижнем регистре ('' — неизвестное). */
function files_ext(string $name): string
{
    $ext = strtolower(pathinfo($name, PATHINFO_EXTENSION));
    return in_array($ext, FILES_EXTS, true) ? $ext : '';
}

/** Папка шрифтов организации (создаётся сама, с подсказкой внутри). */
function fonts_dir(string $root): string
{
    $dir = "$root/fonts";
    if (!is_dir($dir)) {
        @mkdir($dir, 0770, true);
        @file_put_contents("$dir/README.txt", "Положите сюда шрифты, на которые у организации есть лицензия (.ttf, .otf),\n"
            . "например фирменные гарнитуры. AI-After Effects загрузит их у всех пользователей,\n"
            . "и титры будут набраны теми же шрифтами.\n");
    }
    return $dir;
}

/** Имя файла, как его видит пользователь: без путей и служебных символов. */
function files_name(?string $name): string
{
    $name = trim((string)$name);
    $name = preg_replace('/[\x00-\x1F\x7F\/\\\\]+/u', ' ', $name) ?? '';
    $name = trim(preg_replace('/\s+/u', ' ', $name) ?? '');
    if ($name === '' || $name === '.' || $name === '..') {
        files_fail(400, 'Пустое имя файла.');
    }
    if (mb_strlen($name) > 120) {
        $name = mb_substr($name, 0, 120);
    }
    // Проект без известного расширения сохраняется в родном формате.
    if (files_ext($name) === '') {
        $name .= '.ecproj';
    }
    return $name;
}

/** Список файлов пользователя под блокировкой: $fn меняет его и возвращает результат. */
function files_index(string $dir, callable $fn)
{
    $lock = fopen("$dir/index.lock", 'c');
    if ($lock === false) {
        files_fail(500, 'Не удалось открыть список файлов.');
    }
    flock($lock, LOCK_EX);
    try {
        $raw = is_file("$dir/index.json") ? file_get_contents("$dir/index.json") : '';
        $index = $raw ? json_decode($raw, true) : [];
        if (!is_array($index)) {
            $index = [];
        }
        $before = $index;
        $result = $fn($index);
        if ($index !== $before) {
            $tmp = "$dir/index.json." . bin2hex(random_bytes(4));
            file_put_contents($tmp, json_encode($index, JSON_UNESCAPED_UNICODE | JSON_PRETTY_PRINT));
            rename($tmp, "$dir/index.json");
        }
        return $result;
    } finally {
        flock($lock, LOCK_UN);
        fclose($lock);
    }
}

function files_json_body(): array
{
    $j = json_decode((string)file_get_contents('php://input'), true);
    return is_array($j) ? $j : [];
}

$method = $_SERVER['REQUEST_METHOD'] ?? 'GET';
$action = (string)($_GET['action'] ?? ($method === 'GET' ? 'list' : ''));

switch ($action) {
    case 'list':
        $files = files_index($dir, static function (array &$index): array {
            $out = [];
            foreach ($index as $name => $f) {
                $out[] = ['name' => (string)$name, 'size' => (int)($f['size'] ?? 0), 'modified' => (int)($f['modified'] ?? 0), 'kind' => files_kind((string)$name)];
            }
            usort($out, static fn($a, $b) => $b['modified'] <=> $a['modified']);
            return $out;
        });
        files_ok(['user' => $user, 'files' => $files, 'limit' => FILES_MAX_BYTES]);

    case 'load':
        $name = files_name($_GET['name'] ?? '');
        $file = files_index($dir, static fn(array &$index) => $index[$name]['file'] ?? null);
        if (!is_string($file) || !preg_match(FILES_STORED, $file) || !is_file("$dir/$file")) {
            files_fail(404, "Файла «{$name}» нет.");
        }
        header('Content-Type: application/octet-stream');
        header('Content-Length: ' . filesize("$dir/$file"));
        readfile("$dir/$file");
        exit;

    case 'save':
        if ($method !== 'POST') {
            files_fail(405, 'Нужен POST.');
        }
        $name = files_name($_GET['name'] ?? '');
        $overwrite = ($_GET['overwrite'] ?? '') === '1';
        $declared = (int)($_SERVER['CONTENT_LENGTH'] ?? 0);
        if ($declared > FILES_MAX_BYTES) {
            files_fail(413, 'Файл больше 2 ГБ.');
        }
        $tmp = "$dir/upload." . bin2hex(random_bytes(6));
        $in = fopen('php://input', 'rb');
        $out = fopen($tmp, 'wb');
        $size = ($in && $out) ? stream_copy_to_stream($in, $out, FILES_MAX_BYTES + 1) : false;
        if ($in) fclose($in);
        if ($out) fclose($out);
        if ($size === false || $size === 0) {
            @unlink($tmp);
            files_fail(400, $declared > 0
                ? 'Сервер не передал PHP содержимое файла. Скорее всего, файл больше post_max_size в настройках PHP — увеличьте post_max_size и upload_max_filesize.'
                : 'Пустой файл.');
        }
        if ($size > FILES_MAX_BYTES) {
            @unlink($tmp);
            files_fail(413, 'Файл больше 2 ГБ.');
        }
        // Содержимое должно соответствовать расширению (подпись в начале файла).
        $ext = files_ext($name);
        $head = (string)file_get_contents($tmp, false, null, 0, 4096);
        if (!files_magic($ext, $head)) {
            @unlink($tmp);
            files_fail(400, 'Содержимое не похоже на файл .' . $ext . '.');
        }
        $saved = files_index($dir, static function (array &$index) use ($dir, $name, $overwrite, $tmp, $size) {
            if (isset($index[$name]) && !$overwrite) {
                return ['exists' => true];
            }
            if (!isset($index[$name]) && count($index) >= FILES_MAX_COUNT) {
                return ['full' => true];
            }
            $old = (string)($index[$name]['file'] ?? '');
            $file = preg_match(FILES_STORED, $old) ? $old : (bin2hex(random_bytes(16)) . '.' . files_ext($name));
            if (!rename($tmp, "$dir/$file")) {
                return ['error' => true];
            }
            $index[$name] = ['file' => $file, 'size' => $size, 'modified' => time()];
            return ['modified' => $index[$name]['modified']];
        });
        if (!empty($saved['exists']) || !empty($saved['full']) || !empty($saved['error'])) {
            @unlink($tmp);
        }
        if (!empty($saved['exists'])) {
            files_fail(409, "Файл «{$name}» уже есть.", ['exists' => true]);
        }
        if (!empty($saved['full'])) {
            files_fail(507, 'Слишком много файлов. Удалите ненужные.');
        }
        if (!empty($saved['error'])) {
            files_fail(500, 'Не удалось записать файл на сервер. Проверьте права PHP на ' . $dir);
        }
        files_ok(['name' => $name, 'size' => $size, 'modified' => $saved['modified'], 'kind' => files_kind($name)]);

    case 'rename':
        if ($method !== 'POST') {
            files_fail(405, 'Нужен POST.');
        }
        $body = files_json_body();
        $name = files_name($body['name'] ?? '');
        $to = files_name($body['to'] ?? '');
        $r = files_index($dir, static function (array &$index) use ($name, $to) {
            if (!isset($index[$name])) return 'missing';
            if ($name === $to) return 'ok';
            if (isset($index[$to])) return 'exists';
            $index[$to] = $index[$name];
            unset($index[$name]);
            return 'ok';
        });
        if ($r === 'missing') files_fail(404, "Файла «{$name}» нет.");
        if ($r === 'exists') files_fail(409, "Файл «{$to}» уже есть.", ['exists' => true]);
        files_ok(['name' => $to]);

    case 'delete':
        if ($method !== 'POST') {
            files_fail(405, 'Нужен POST.');
        }
        $name = files_name(files_json_body()['name'] ?? '');
        $r = files_index($dir, static function (array &$index) use ($dir, $name) {
            if (!isset($index[$name])) return false;
            $file = (string)($index[$name]['file'] ?? '');
            if (preg_match(FILES_STORED, $file) && is_file("$dir/$file")) {
                @mkdir("$dir/.trash", 0770);
                @rename("$dir/$file", "$dir/.trash/" . date('Ymd-His') . '-' . $file);
            }
            unset($index[$name]);
            return true;
        });
        if (!$r) files_fail(404, "Файла «{$name}» нет.");
        files_ok([]);

    case 'fonts':
        $fdir = fonts_dir($root);
        $fonts = [];
        foreach (scandir($fdir) ?: [] as $f) {
            if (preg_match(FONTS_EXTS, $f) && is_file("$fdir/$f") && filesize("$fdir/$f") <= FONTS_MAX_BYTES) {
                $fonts[] = ['name' => $f, 'size' => filesize("$fdir/$f")];
            }
            if (count($fonts) >= FONTS_MAX_COUNT) {
                break;
            }
        }
        files_ok(['fonts' => $fonts]);

    case 'font':
        $f = basename((string)($_GET['name'] ?? ''));
        $path = fonts_dir($root) . '/' . $f;
        if ($f === '' || $f[0] === '.' || !preg_match(FONTS_EXTS, $f) || !is_file($path) || filesize($path) > FONTS_MAX_BYTES) {
            files_fail(404, 'Нет такого шрифта.');
        }
        header('Content-Type: application/octet-stream');
        header('Content-Length: ' . filesize($path));
        header('Cache-Control: private, max-age=86400');
        readfile($path);
        exit;

    default:
        files_fail(400, 'Неизвестное действие.');
}
