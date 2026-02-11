#!/usr/bin/env python3
"""Topologically sort the dependency tree from notes/ frontmatter."""

from __future__ import annotations

import re
import sys
from collections import defaultdict
from pathlib import Path

NOTES_DIR = Path(__file__).resolve().parent.parent / "notes"


def parse_frontmatter(path: Path) -> dict | None:
    """Extract dependencies, ported, tested_story, and has_story from YAML frontmatter."""
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

    # tested_story
    tsm = re.search(r"^tested_story:\s*(true|false)", fm, re.MULTILINE)
    tested_story = tsm.group(1) == "true" if tsm else False

    # has_story — react_story is non-empty
    rsm = re.search(r'^react_story:\s*"(.*?)"', fm, re.MULTILINE)
    has_story = bool(rsm and rsm.group(1).strip())

    # dependencies — either `[]` or a YAML list of wikilinks
    deps: list[str] = []
    dm = re.search(r"^dependencies:\s*\[\]", fm, re.MULTILINE)
    if not dm:
        deps = re.findall(r'- "\[\[([^]|]+?)(?:\|[^]]*?)?\]\]"', fm)

    return {
        "ported": ported,
        "tested_story": tested_story,
        "has_story": has_story,
        "deps": deps,
    }


def is_dep_complete(info: dict) -> bool:
    """A dependency is complete when ported AND (story tested OR no story)."""
    if not info["ported"]:
        return False
    if info["has_story"] and not info["tested_story"]:
        return False
    return True


def topo_sort(
    nodes: dict[str, dict],
) -> list[tuple[str, dict]]:
    """Kahn's algorithm. Ties broken alphabetically for stable output."""
    in_degree: dict[str, int] = {n: 0 for n in nodes}
    dependents: dict[str, list[str]] = defaultdict(list)

    for name, info in nodes.items():
        for dep in info["deps"]:
            if dep in nodes:
                in_degree[name] += 1
                dependents[dep].append(name)

    queue = sorted(n for n, d in in_degree.items() if d == 0)
    result: list[tuple[str, dict]] = []

    while queue:
        node = queue.pop(0)
        result.append((node, nodes[node]))
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
            result.append((n, nodes[n]))

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

    # Determine which components are ready to port:
    # not yet ported AND all deps are complete
    def deps_complete(info: dict) -> bool:
        return all(
            is_dep_complete(nodes[dep])
            for dep in info["deps"]
            if dep in nodes
        )

    # Print table
    print(f"{'Component':<40} {'Ported':<8} {'Story Tested':<14} {'Ready'}")
    print("-" * 75)
    for name, info in order:
        ported_str = "yes" if info["ported"] else "no"
        if not info["has_story"]:
            story_str = "n/a"
        elif info["tested_story"]:
            story_str = "yes"
        else:
            story_str = "no"

        ready = ""
        if not info["ported"] and deps_complete(info):
            ready = "<-- next"
        elif info["ported"] and info["has_story"] and not info["tested_story"]:
            ready = "<-- needs story test"

        print(f"  {name:<38} {ported_str:<8} {story_str:<14} {ready}")


if __name__ == "__main__":
    main()
