#!/usr/bin/env python3
"""Rank ported components by E2E testing priority.

Priority is based on dependant count: components that more other components
depend on should be tested first, because a bug in a foundational component
breaks everything above it.

Scores:
  - 1st-order dependants  ×3
  - 2nd-order dependants  ×2
  - 3rd+-order dependants ×1

Only includes components that are (a) ported and (b) have interactive behavior
worth testing via Cypress (filters out pure hooks/utilities).
"""

from __future__ import annotations

import re
import sys
from collections import defaultdict
from pathlib import Path

NOTES_DIR = Path(__file__).resolve().parent.parent / "notes"

# Components with meaningful interactive behavior for E2E testing.
# Utilities/hooks are tested indirectly through these.
E2E_CANDIDATES = {
    "leptos-accordion",
    "leptos-alert-dialog",
    "leptos-checkbox",
    "leptos-collapsible",
    "leptos-context-menu",
    "leptos-dialog",
    "leptos-dropdown-menu",
    "leptos-form",
    "leptos-hover-card",
    "leptos-menu",
    "leptos-menubar",
    "leptos-navigation-menu",
    "leptos-popover",
    "leptos-progress",
    "leptos-radio-group",
    "leptos-scroll-area",
    "leptos-select",
    "leptos-slider",
    "leptos-switch",
    "leptos-tabs",
    "leptos-toast",
    "leptos-toggle",
    "leptos-toggle-group",
    "leptos-toolbar",
    "leptos-tooltip",
}


def parse_frontmatter(path: Path) -> dict | None:
    text = path.read_text()
    m = re.match(r"^---\n(.*?)\n---", text, re.DOTALL)
    if not m:
        return None
    fm = m.group(1)

    pm = re.search(r"^ported:\s*(true|false)", fm, re.MULTILINE)
    if not pm:
        return None
    ported = pm.group(1) == "true"

    um = re.search(r"^unstable:\s*(true|false)", fm, re.MULTILINE)
    unstable = um.group(1) == "true" if um else False

    deps: list[str] = []
    dm = re.search(r"^dependencies:\s*\[\]", fm, re.MULTILINE)
    if not dm:
        deps = re.findall(r'- "\[\[([^]|]+?)(?:\|[^]]*?)?\]\]"', fm)

    return {"ported": ported, "unstable": unstable, "deps": deps}


def transitive_dependants(
    node: str,
    reverse_graph: dict[str, set[str]],
    depth: int = 0,
    visited: set[str] | None = None,
) -> list[tuple[str, int]]:
    """Return list of (dependant, depth) pairs reachable from node."""
    if visited is None:
        visited = set()
    result: list[tuple[str, int]] = []
    for dep in reverse_graph.get(node, set()):
        if dep not in visited:
            visited.add(dep)
            result.append((dep, depth + 1))
            result.extend(
                transitive_dependants(dep, reverse_graph, depth + 1, visited)
            )
    return result


def main() -> None:
    nodes: dict[str, dict] = {}
    for path in sorted(NOTES_DIR.glob("*.md")):
        info = parse_frontmatter(path)
        if info is None:
            continue
        nodes[path.stem] = info

    # Build reverse dependency graph (dependant -> set of things that depend on it)
    reverse_graph: defaultdict[str, set[str]] = defaultdict(set)
    for name, info in nodes.items():
        for dep in info["deps"]:
            if dep in nodes:
                reverse_graph[dep].add(name)

    # Compute scores for E2E candidates
    scores: list[tuple[str, int, int, int, int, bool]] = []
    for name in sorted(nodes):
        if name not in E2E_CANDIDATES:
            continue
        info = nodes[name]

        deps = transitive_dependants(name, reverse_graph)
        first = sum(1 for _, d in deps if d == 1)
        second = sum(1 for _, d in deps if d == 2)
        third_plus = sum(1 for _, d in deps if d >= 3)

        score = first * 3 + second * 2 + third_plus * 1

        # Also count transitive dependencies (how much code this E2E validates)
        def count_transitive_deps(n: str, seen: set[str] | None = None) -> int:
            if seen is None:
                seen = set()
            for d in nodes.get(n, {}).get("deps", []):
                if d not in seen and d in nodes:
                    seen.add(d)
                    count_transitive_deps(d, seen)
            return len(seen)

        dep_count = count_transitive_deps(name)

        scores.append((name, score, first, second, third_plus, dep_count, info["ported"]))

    # Sort by: ported first, then score desc, then dep_count desc
    scores.sort(key=lambda x: (-x[6], -x[1], -x[5]))

    # Print table
    print(
        f"{'#':<4} {'Component':<35} {'Score':<7} {'1st':<5} {'2nd':<5} "
        f"{'3rd+':<6} {'Deps':<6} {'Ported':<8} {'Status'}"
    )
    print("-" * 100)
    for i, (name, score, first, second, third_plus, dep_count, ported) in enumerate(
        scores, 1
    ):
        ported_str = "yes" if ported else "no"
        status = ""
        if not ported:
            status = "not ported"
        elif nodes[name].get("unstable"):
            status = "unstable"
        print(
            f"  {i:<3} {name:<34} {score:<7} {first:<5} {second:<5} "
            f"{third_plus:<6} {dep_count:<6} {ported_str:<8} {status}"
        )


if __name__ == "__main__":
    main()
