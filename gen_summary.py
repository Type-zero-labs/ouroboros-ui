#!/usr/bin/env python3
"""Generate docs/SUMMARY.md for mdBook from the existing docs tree.

Titles come from each file's first H1 (falls back to Title-Cased filename).
Rerun after adding a component page: python3 gen_summary.py
"""
import re
from pathlib import Path

DOCS = Path(__file__).parent / "docs"
LAYERS = ["atoms", "cells", "molecules", "organisms", "graph"]


def title_of(md: Path) -> str:
    for line in md.read_text(encoding="utf-8").splitlines():
        m = re.match(r"#\s+(.*)", line)
        if m:
            t = m.group(1).strip()
            t = re.sub(r"[`*]", "", t)
            t = re.sub(r"\s+—.*$", "", t)  # drop subtitle after em-dash
            return t
    return md.stem.replace("_", " ").title()


def line(depth: int, md: Path) -> str:
    rel = md.relative_to(DOCS).as_posix()
    return f"{'  ' * depth}- [{title_of(md)}](./{rel})"


out = ["# Summary", ""]
out.append("[Introduction](./README.md)")
out.append("")
out.append("# Foundations")
out.append("")
for name in ["usage.md", "architecture.md", "tokens.md", "theming.md",
             "typography.md", "layout.md", "governance.md", "guards.md"]:
    p = DOCS / name
    if p.exists():
        out.append(line(0, p))
out.append("")
out.append("# Components")
out.append("")
out.append(line(0, DOCS / "components" / "README.md"))
for layer in LAYERS:
    d = DOCS / "components" / layer
    if not d.is_dir():
        continue
    readme = d / "README.md"
    if readme.exists():
        out.append(line(1, readme))
        depth = 2
    else:
        out.append(f"  - [{layer.title()}]()")
        depth = 2
    for md in sorted(d.glob("*.md")):
        if md.name == "README.md":
            continue
        out.append(line(depth, md))

(DOCS / "SUMMARY.md").write_text("\n".join(out) + "\n", encoding="utf-8")
print(f"SUMMARY.md: {sum(1 for l in out if l.startswith(('-', '  ')))} entries")
