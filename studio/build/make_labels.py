#!/usr/bin/env python3
"""Russian names for the assistant's command search.

Reads the Russian catalogs AI-After Effects is built with and writes studio/ru-labels.json:

    {"text": {"Import...": "Импорт...", ...}, "id": {"layer.newText": "Текст", ...}}

- crates/ui-egui/src/i18n.rs, `const RUSSIAN`: the menu tree, rows `(command, "English", "Русский")`;
- crates/ui-egui/src/i18n/ui.rs, `const RUSSIAN`: panel strings, rows `("English", "Русский")`.

The assistant panel uses it to show and search commands (their labels and menu-path segments) in
Russian. Run by studio/build/package.sh.
"""

import json
import re
import sys
from collections import Counter
from pathlib import Path

STR = r'"((?:[^"\\]|\\.)*)"'
ROW3 = re.compile(r'\(\s*' + STR + r'\s*,\s*' + STR + r'\s*,\s*' + STR + r'\s*,?\s*\)', re.S)
ROW2 = re.compile(r'\(\s*' + STR + r'\s*,\s*' + STR + r'\s*,?\s*\)', re.S)


def unescape(s: str) -> str:
    return s.encode("utf-8").decode("unicode_escape").encode("latin-1").decode("utf-8") if "\\" in s else s


def block(body: str, name: str) -> str:
    start = body.find(f"const {name}:")
    if start < 0:
        return ""
    end = body.find("\n];", start)
    return body[start : end if end > 0 else len(body)]


def main() -> int:
    root = Path(__file__).resolve().parents[2]
    dst = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("ru-labels.json")
    text: dict[str, str] = {}
    ids: dict[str, str] = {}
    menus = root / "crates" / "ui-egui" / "src" / "i18n.rs"
    panels = root / "crates" / "ui-egui" / "src" / "i18n" / "ui.rs"
    if panels.is_file():
        for m in ROW2.finditer(block(panels.read_text(encoding="utf-8"), "RUSSIAN")):
            try:
                en, ru = unescape(m.group(1)), unescape(m.group(2))
            except UnicodeError:
                continue
            if ru.strip():
                text[en] = ru
    if menus.is_file():
        rows = []
        for m in ROW3.finditer(block(menus.read_text(encoding="utf-8"), "RUSSIAN")):
            try:
                rows.append((unescape(m.group(1)), unescape(m.group(2)), unescape(m.group(3))))
            except UnicodeError:
                continue
        once = Counter(cmd for cmd, _, _ in rows if cmd)
        for cmd, en, ru in rows:
            if not ru.strip():
                continue
            text.setdefault(en, ru)
            if cmd and once[cmd] == 1:
                ids[cmd] = ru
    else:
        print(f"warning: {menus} not found; command names stay English", file=sys.stderr)
    dst.parent.mkdir(parents=True, exist_ok=True)
    dst.write_text(json.dumps({"text": text, "id": ids}, ensure_ascii=False, separators=(",", ":")), encoding="utf-8")
    print(f"{dst}: {len(text)} labels, {len(ids)} command names")
    return 0


if __name__ == "__main__":
    sys.exit(main())
