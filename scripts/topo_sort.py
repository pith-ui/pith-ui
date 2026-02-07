#!/usr/bin/env python3
"""Topologically sort the dependency tree from notes/ frontmatter."""

from __future__ import annotations

import re
import sys
from collections import defaultdict
from pathlib import Path

NOTES_DIR = Path(__file__).resolve().parent.parent / "notes"


def parse_frontmatter(path: Path) -> dict | None:
    """Extract dependencies and ported status from YAML frontmatter."""
    text = path.read_text()
    m = re.match(r"^---\n(.*?)\n---", text, re.DOTALL)
    if not m:
        return None

    fm = m.group(1)

    # ported
    pm = re.search(r"^ported:\s*(true|false)", fm, re.MULTILINE)
    if not pm:
        return None
    ported = pm.group(1) == "true"

    # dependencies — either `[]` or a YAML list of wikilinks
    deps: list[str] = []
    dm = re.search(r"^dependencies:\s*\[\]", fm, re.MULTILINE)
    if not dm:
        deps = re.findall(r'- "\[\[([^]|]+?)(?:\|[^]]*?)?\]\]"', fm)

    return {"ported": ported, "deps": deps}


def topo_sort(
    nodes: dict[str, dict],
) -> list[tuple[str, bool]]:
    """Kahn's algorithm. Ties broken alphabetically for stable output."""
    in_degree: dict[str, int] = {n: 0 for n in nodes}
    dependents: dict[str, list[str]] = defaultdict(list)

    for name, info in nodes.items():
        for dep in info["deps"]:
            if dep in nodes:
                in_degree[name] += 1
                dependents[dep].append(name)

    queue = sorted(n for n, d in in_degree.items() if d == 0)
    result: list[tuple[str, bool]] = []

    while queue:
        node = queue.pop(0)
        result.append((node, nodes[node]["ported"]))
        for dep in sorted(dependents[node]):
            in_degree[dep] -= 1
            if in_degree[dep] == 0:
                queue.append(dep)
                queue.sort()

    if len(result) != len(nodes):
        sorted_names = {r[0] for r in result}
        cycle = [n for n in nodes if n not in sorted_names]
        print(f"WARNING: cycle detected among: {cycle}", file=sys.stderr)
        for n in sorted(cycle):
            result.append((n, nodes[n]["ported"]))

    return result


def main() -> None:
    nodes: dict[str, dict] = {}
    for path in sorted(NOTES_DIR.glob("*.md")):
        info = parse_frontmatter(path)
        if info is None:
            continue
        name = path.stem
        nodes[name] = info

    order = topo_sort(nodes)

    # Print as a Python literal
    print("[")
    for name, ported in order:
        print(f'    ("{name}", {ported}),')
    print("]")


if __name__ == "__main__":
    main()
