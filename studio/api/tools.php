<?php
// studio/api/tools.php — что помощник AI Анимации знает о себе и какие действия ему разрешены.
//
// Инструменты выполняет БРАУЗЕР (studio.js): только он видит запущенный
// редактор. Сервер лишь объявляет модели, какие инструменты есть, и решает,
// какие из них доступны в текущем режиме.

/** Инструменты, которые ничего не меняют в проекте. Доступны в обоих режимах. */
function studio_read_tools(): array
{
    return [
        [
            'type' => 'function',
            'function' => [
                'name' => 'find_commands',
                'description' => 'Найти команды редактора по смыслу (по-русски или по-английски). '
                    . 'Возвращает id команды, русское название, путь в меню, горячую клавишу и описание параметров. '
                    . 'Вызывай перед run_command, если не знаешь точный id: id нельзя угадывать.',
                'parameters' => [
                    'type' => 'object',
                    'properties' => [
                        'query' => ['type' => 'string', 'description' => 'Что нужно сделать, 1–4 слова: «текстовый слой», «ключевой кадр», «тень», «эффект размытие», «маска», «composition».'],
                    ],
                    'required' => ['query'],
                ],
            ],
        ],
        [
            'type' => 'function',
            'function' => [
                'name' => 'get_state',
                'description' => 'Текущее состояние проекта: имя, сохранён ли, элементы проекта (композиции, видео, картинки, звук; '
                    . 'missing — файл не найден), активная композиция (размер, частота кадров, длительность, рабочая область) '
                    . 'и её слои (id, номер, имя, тип, вход и выход в секундах, родитель, режим наложения, переключатели), '
                    . 'текущее время, выделенные слои.',
                'parameters' => ['type' => 'object', 'properties' => new stdClass()],
            ],
        ],
        [
            'type' => 'function',
            'function' => [
                'name' => 'layer_details',
                'description' => 'Один слой подробно: дерево его свойств с путями (path) для prop.set и prop.addKey — '
                    . 'трансформация (transform/position, transform/scale, transform/rotation, transform/opacity, transform/anchor), '
                    . 'текст, эффекты, маски, стили слоя; значения в текущий момент и ключевые кадры.',
                'parameters' => [
                    'type' => 'object',
                    'properties' => [
                        'layer' => ['type' => 'string', 'description' => 'id или имя слоя из get_state'],
                        'depth' => ['type' => 'integer', 'description' => 'Глубина дерева, по умолчанию 4'],
                    ],
                    'required' => ['layer'],
                ],
            ],
        ],
        [
            'type' => 'function',
            'function' => [
                'name' => 'check_project',
                'description' => 'Проверка проекта перед рендером: слои, которые целиком или частично за пределами кадра композиции, '
                    . 'слои, которые начинаются или кончаются за пределами длительности композиции, отсутствующие файлы, пустые композиции.',
                'parameters' => ['type' => 'object', 'properties' => new stdClass()],
            ],
        ],
        [
            'type' => 'function',
            'function' => [
                'name' => 'look_at_frame',
                'description' => 'Отрисовать кадр активной композиции и посмотреть на него: проверить титр, расположение, цвета. '
                    . 'Без time — текущий момент.',
                'parameters' => [
                    'type' => 'object',
                    'properties' => [
                        'time' => ['type' => 'number', 'description' => 'Секунда композиции'],
                    ],
                ],
            ],
        ],
    ];
}

/** Инструменты, которые меняют проект. Только в режиме «Сделай». */
function studio_act_tools(): array
{
    return [
        [
            'type' => 'function',
            'function' => [
                'name' => 'run_command',
                'description' => 'Выполнить команду редактора с параметрами. Диалогов нет: всё задаётся параметрами. '
                    . 'Многие команды без layers действуют на выделенные слои. Действие отменяется (Ctrl+Z).',
                'parameters' => [
                    'type' => 'object',
                    'properties' => [
                        'id' => ['type' => 'string', 'description' => 'id команды, например comp.new, layer.newText, layer.newShape, layer.newSolid, prop.set, layer.setText, effect.apply, layer.style.dropShadow, layer.timing, layer.select'],
                        'params' => ['type' => 'object', 'description' => 'Параметры команды (JSON-объект). Можно не передавать — тогда значения по умолчанию.'],
                    ],
                    'required' => ['id'],
                ],
            ],
        ],
        [
            'type' => 'function',
            'function' => [
                'name' => 'animate',
                'description' => 'Анимировать свойство слоя ключевыми кадрами: на каждое время — своё значение. '
                    . 'Время — секунды композиции. Значения: позиция [x, y] (пиксели композиции, 0,0 — левый верхний угол), '
                    . 'масштаб [x, y] (%), поворот (градусы), непрозрачность (0–100). ease — плавные вход и выход (Easy Ease), по умолчанию да.',
                'parameters' => [
                    'type' => 'object',
                    'properties' => [
                        'layer' => ['type' => 'string', 'description' => 'id или имя слоя'],
                        'property' => ['type' => 'string', 'description' => 'Путь свойства: transform/position, transform/scale, transform/rotation, transform/opacity, transform/anchor или путь из layer_details'],
                        'keys' => [
                            'type' => 'array',
                            'description' => 'Ключевые кадры: [{"time": 0, "value": 0}, {"time": 0.6, "value": 100}]',
                            'items' => ['type' => 'object', 'properties' => ['time' => ['type' => 'number'], 'value' => new stdClass()]],
                        ],
                        'ease' => ['type' => 'boolean', 'description' => 'Плавно (Easy Ease), по умолчанию true'],
                    ],
                    'required' => ['layer', 'property', 'keys'],
                ],
            ],
        ],
        [
            'type' => 'function',
            'function' => [
                'name' => 'undo',
                'description' => 'Отменить последние действия (как Ctrl+Z).',
                'parameters' => [
                    'type' => 'object',
                    'properties' => [
                        'steps' => ['type' => 'integer', 'description' => 'Сколько шагов отменить, по умолчанию 1', 'minimum' => 1, 'maximum' => 20],
                    ],
                ],
            ],
        ],
    ];
}

/** Системная инструкция. $mode: 'advice' (Совет) или 'act' (Сделай). */
function studio_system_prompt(string $mode): string
{
    $available = $mode === 'act' ? array_merge(studio_read_tools(), studio_act_tools()) : studio_read_tools();
    $catalog = studio_tool_catalog($available);

    $common = <<<TXT
Ты — встроенный помощник AI Анимации, программы моушн-дизайна и композитинга:
меню Файл, Правка, Композиция, Слой, Эффект, Анимация, Вид, Окно; панели Проект, Композиция, Таймлайн, Элементы
управления эффектами, Эффекты и шаблоны, Символ, Абзац, Очередь рендеринга; инструменты Выделение, Рука, Масштаб,
Поворот, Камера, Перо, Фигуры, Текст. Проект состоит из элементов: композиций, видео, картинок, звука, сплошных цветов.
В композиции — слои (текст, фигуры, сплошной цвет, нулевой объект, корректирующий, видео и картинки, камера, свет,
вложенные композиции); у слоя есть вход и выход по времени, трансформация (опорная точка, положение, масштаб, поворот,
непрозрачность), эффекты, маски, стили слоя; свойства анимируются ключевыми кадрами и выражениями.
Проекты — .ecproj (родной формат), открываются также .ecprojx и проекты After Effects (.aep, .aepx); картинки, видео, звук и Lottie (.json)
импортируются. Файлы пользователя хранятся на его сервере («Мои проекты»). Готовое видео, GIF и секвенции кадров
выводятся через Композиция › Добавить в очередь рендеринга и скачиваются. Стандарт по умолчанию — 1920×1080, 25 кадр/с.
Интерфейс на русском. Пользователь работает в редакторе прямо сейчас. К его сообщению приложено состояние проекта,
а иногда кадр композиции.

Как отвечать:
- Всегда по-русски, коротко и по делу: 1–6 предложений или короткий нумерованный список шагов.
- Пункты меню пиши так: «Слой › Создать › Текст». Горячие клавиши — в скобках: (Ctrl+Alt+Shift+T). Сочетания — те, что показаны в меню программы.
- Названия команд бери из find_commands, а не по памяти. Не выдумывай функций, которых нет.
- Единицы: время — секунды (в командах; на экране тайм-код), положение — пиксели композиции от левого верхнего угла
  (центр кадра 1920×1080 — [960, 540]), масштаб — проценты [100, 100], поворот — градусы, непрозрачность — 0…100,
  цвет — "#rrggbb" или [r, g, b] от 0 до 1.
- Сохранение: Ctrl+S сохраняет проект в «Мои проекты» на сервере (значок папки в панели помощника).

Твои инструменты (других нет — ни computer_use, ни кликов по координатам):
{$catalog}
Вызывай их штатно. Если штатный вызов недоступен, напиши вызов текстом, по одному блоку на вызов:
<tool_call><function=ИМЯ><parameter=КЛЮЧ>ЗНАЧЕНИЕ</parameter></function></tool_call>
Значения-объекты и списки пиши в JSON. После блока ничего не пиши — дождись результата.
TXT;

    if ($mode === 'act') {
        return $common . "\n\n" . <<<TXT
Режим «Сделай»: ты сам правишь проект с помощью инструментов.
Это главное правило: не объясняй пользователю, как сделать вручную, — сделай сам. Если раньше в разговоре
ты был в режиме «Совет» и писал, что не можешь ничего менять, — это больше не так: сейчас режим «Сделай».
Каждое сообщение пользователя начинается с пометки текущего режима — верь ей.
Порядок работы:
1. Пойми задачу. Если она неоднозначна и ошибка дорого стоит — задай один уточняющий вопрос.
   Если не хватает мелочей (текст титра, цвет, имя) — возьми разумные и скажи, что их легко поменять.
2. Посмотри проект: состояние приложено к сообщению; слой подробно — layer_details; кадр — look_at_frame.
3. Если активной композиции нет — создай её: comp.new {name, width: 1920, height: 1080, frameRate: 25, duration (с), background?}.
4. Действуй командами (run_command) и animate:
   - текст: layer.newText {text, position: [x, y], font?, style?, size, fill: "#rrggbb", justify: "center", tracking?};
     изменить текст или его вид: layer.setText {layer, text?, size?, fill?, font?, style?, tracking?};
   - фигуры: layer.newShape {kind: rect|rounded|ellipse|star|polygon, name, size: [w, h], position: [x, y], fill: "#rrggbb", stroke?, strokeWidth?};
     сплошной цвет: layer.newSolid {name, color, width, height}; нулевой объект: layer.newNull {name};
   - свойства: prop.set {layer, path, value} (path — transform/position, transform/scale, transform/rotation, transform/opacity…);
     трансформация сразу у нескольких слоёв: layer.setTransform {layers, prop: position|scale|rotation|opacity|anchor, value};
   - анимация: animate {layer, property, keys: [{time, value}…], ease} — ключевые кадры с плавностью; готовые анимации:
     anim.applyBuiltinPreset {preset: fadeIn|fadeOut|wipeInFromLeft|lensPopIn|floatUpDown|pulse|…, layers} (список — anim.builtinPresets);
     анимация текста по буквам: layer.applyTextPreset {layer, preset: typewriter|fadeUpCharacters|bounceInWords|trackingIn|scramble|blurIn|jitter|dropInLines};
   - время слоя: layer.timing {layers, in?, out?, start? (секунды композиции)}; длительность композиции: comp.settings {duration};
   - стили слоя: layer.style.dropShadow {layers} (тень), layer.style.outerGlow, layer.style.stroke; их параметры — prop.set по путям из layer_details;
   - эффекты: effect.apply {effect: "Gaussian Blur" | "Drop Shadow" | "Fill" | …, layers}; параметры — prop.set по путям из layer_details;
   - порядок и связь: layer.select {layers}, layer.setParent {layers, parent}, layer.precompose {layers, name};
   - элементы проекта в композицию: layer.addItem {item: id|имя, time?, position?}.
   Слой указывай id или именем из get_state. Если не знаешь id команды или её параметры — find_commands.
5. После заметных изменений проверь результат (get_state, look_at_frame в нужный момент): текст не выходит за кадр,
   анимация начинается и кончается вовремя. Коротко скажи, что сделал и как отменить (Ctrl+Z).
Правила безопасности:
- Не создавай новый проект, не открывай и не закрывай файлы, не удаляй слои и композиции, если пользователь прямо об этом не просил.
- Доводи задачу до конца сам, не останавливайся спросить «продолжать ли?»: число шагов ограничивает панель, а не ты.
  Если панель остановит тебя, пользователь нажмёт «Продолжить» — продолжай с того места, где остановился.
- После каждой серии действий приходит сообщение «[Состояние после этих шагов — автоматически]» (иногда с кадром композиции).
  Сверяйся с ним: что уже сделано, что осталось. Не повторяй то, что уже есть. Если инструмент вернул warning
  о повторе — остановись, посмотри на результат (get_state или look_at_frame) и делай следующую часть задачи.
- Если просят переделать или сделать лучше — сначала посмотри, что уже есть (get_state, layer_details, look_at_frame), реши, что убрать,
  а что поправить, и только потом действуй. Не начинай с нуля, если можно улучшить то, что есть.
- Удаление и подобное панель может подтвердить у пользователя; если пользователь отказал — не повторяй это действие,
  а спроси, как поступить.
- Если команда вернула ошибку — не повторяй вслепую: прочитай ошибку и params_doc, поправь параметры или объясни, что мешает.
TXT;
    }

    return $common . "\n\n" . <<<TXT
Режим «Совет»: ты только объясняешь и подсказываешь, ничего не меняешь сам.
Можно искать команды (find_commands), смотреть проект (get_state, layer_details, check_project) и кадр (look_at_frame).
Объясняй, куда нажать и что выставить, с конкретными числами. Можешь разобрать композицию, найти ошибки и подсказать, как сделать анимацию.
Если пользователь просит изменить проект за него — скажи, что для этого нужно переключиться в режим «Сделай» (кнопка вверху панели).
TXT;
}

/**
 * Каталог инструментов текстом — для системной подсказки. Некоторые шлюзы (например, vLLM без
 * парсера вызовов) не передают модели поле tools; тогда модель узнаёт о наших инструментах
 * только отсюда и пишет вызовы текстом.
 */
function studio_tool_catalog(array $tools): string
{
    $lines = [];
    foreach ($tools as $t) {
        $f = $t['function'] ?? [];
        $name = (string)($f['name'] ?? '');
        if ($name === '') {
            continue;
        }
        $params = [];
        foreach (($f['parameters']['properties'] ?? []) as $p => $spec) {
            $params[] = $p . ': ' . ($spec['type'] ?? 'string');
        }
        $lines[] = '- ' . $name . '(' . implode(', ', $params) . ') — ' . (string)($f['description'] ?? '');
    }
    return implode("\n", $lines);
}

/**
 * Вызовы инструментов, которые модель написала текстом, а не штатным полем tool_calls:
 *   <tool_call><function=ИМЯ><parameter=КЛЮЧ>ЗНАЧЕНИЕ</parameter>…</function></tool_call>  (Qwen)
 *   <tool_call>{"name": "ИМЯ", "arguments": {…}}</tool_call>                             (Hermes)
 * Возвращает [вызовы, текст без этих блоков].
 */
function studio_text_tool_calls(string $content): array
{
    $calls = [];
    $value = static function (string $raw) {
        $raw = trim($raw);
        $j = json_decode($raw, true);
        return ($j !== null || $raw === 'null') ? $j : $raw;
    };
    // Qwen XML: с обёрткой <tool_call> или без неё. Прокси с vLLM иногда «съедает» начало
    // вызова («<tool_call>\n<»), и приходит «function=run_command>…» — это тоже понимаем,
    // как и вызов без закрывающих тегов в конце ответа.
    if (preg_match_all('~(?:<tool_call>\s*)?<?function=([A-Za-z0-9_.\-]+)>(.*?)(?:</function>(?:\s*</tool_call>)?|(?=<?function=[A-Za-z0-9_.\-]+>)|\z)~s', $content, $m, PREG_SET_ORDER)) {
        foreach ($m as $one) {
            $args = [];
            if (preg_match_all('~<?parameter=([A-Za-z0-9_.\-]+)>(.*?)(?:</parameter>|(?=<?parameter=[A-Za-z0-9_.\-]+>)|(?=</function>)|\z)~s', $one[2], $pm, PREG_SET_ORDER)) {
                foreach ($pm as $p) {
                    $args[$p[1]] = $value($p[2]);
                }
            }
            $calls[] = ['function' => ['name' => $one[1], 'arguments' => $args ?: new stdClass()]];
            $content = str_replace($one[0], '', $content);
        }
    }
    // Hermes: JSON внутри <tool_call>.
    if (preg_match_all('~<tool_call>\s*(\{.*?\})\s*</tool_call>~s', $content, $m, PREG_SET_ORDER)) {
        foreach ($m as $one) {
            $j = json_decode($one[1], true);
            if (is_array($j) && isset($j['name'])) {
                $args = $j['arguments'] ?? ($j['parameters'] ?? []);
                if (is_string($args)) {
                    $args = json_decode($args, true) ?: [];
                }
                $calls[] = ['function' => ['name' => (string)$j['name'], 'arguments' => $args ?: new stdClass()]];
                $content = str_replace($one[0], '', $content);
            }
        }
    }
    // Рассуждения модели в ответ не нужны.
    $content = preg_replace('~<think>.*?</think>~s', '', $content) ?? $content;
    $content = trim(str_replace(['<tool_call>', '</tool_call>'], '', $content));
    return [$calls, $content];
}
