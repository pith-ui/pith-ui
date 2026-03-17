#!/usr/bin/env python3
"""Query E2E test result history from .results/runs.json."""

import argparse
import json
import sys
from pathlib import Path

RESULTS_DIR = Path(__file__).resolve().parent.parent / ".results"
RUNS_FILE = RESULTS_DIR / "runs.json"


def load_runs():
    if not RUNS_FILE.exists():
        print(f"No results file found at {RUNS_FILE}", file=sys.stderr)
        sys.exit(1)
    with open(RUNS_FILE) as f:
        return json.load(f)


def cmd_history(args):
    """Show run history for a spec, grouped by framework."""
    runs = load_runs()
    spec = args.spec

    # Collect all entries for this spec across all timestamps
    entries = []
    for timestamp, run_data in sorted(runs.items()):
        specs = run_data.get("specs", {})
        if spec in specs:
            entry = specs[spec].copy()
            entry["timestamp"] = timestamp
            entries.append(entry)

    if not entries:
        print(f"No results found for spec '{spec}'.")
        print(f"\nAvailable specs: {', '.join(sorted(all_specs(runs)))}")
        return

    # Group by framework
    by_framework = {}
    for entry in entries:
        fw = entry["framework"]
        by_framework.setdefault(fw, []).append(entry)

    for fw in sorted(by_framework):
        rows = by_framework[fw]
        print(f"\n  {fw}")
        print(f"  {'─' * 64}")
        print(f"  {'Timestamp':<22} {'Status':<6} {'Pass':>5} {'Fail':>5} {'Pend':>5} {'Total':>5}  {'Duration'}")
        print(f"  {'─' * 64}")
        for r in rows:
            status = r["status"]
            marker = "✔" if status == "PASS" else "✖"
            print(
                f"  {r['timestamp']:<22} {marker} {status:<4} "
                f"{r['passing']:>5} {r['failing']:>5} {r['pending']:>5} {r['tests']:>5}  {r['duration']}"
            )
    print()


def all_specs(runs):
    specs = set()
    for run_data in runs.values():
        specs.update(run_data.get("specs", {}).keys())
    return specs


def cmd_list(args):
    """List all specs that have results."""
    runs = load_runs()
    specs = sorted(all_specs(runs))

    if not specs:
        print("No specs found.")
        return

    # For each spec, show latest result per framework
    print(f"\n  {'Spec':<30} {'React':<18} {'Leptos':<18}")
    print(f"  {'─' * 66}")

    for spec in specs:
        latest = {}
        for timestamp in sorted(runs.keys()):
            run_data = runs[timestamp]
            if spec in run_data.get("specs", {}):
                entry = run_data["specs"][spec]
                latest[entry["framework"]] = entry

        react_str = _status_str(latest.get("react"))
        leptos_str = _status_str(latest.get("leptos"))
        print(f"  {spec:<30} {react_str:<18} {leptos_str:<18}")
    print()


def _status_str(entry):
    if not entry:
        return "—"
    marker = "✔" if entry["status"] == "PASS" else "✖"
    return f"{marker} {entry['passing']}/{entry['tests']}"


def main():
    parser = argparse.ArgumentParser(
        description="Query E2E test result history from .results/runs.json"
    )
    sub = parser.add_subparsers(dest="command")

    p_history = sub.add_parser("history", help="Show run history for a spec")
    p_history.add_argument("spec", help="Spec name (e.g. dialog, attr-forwarding)")

    sub.add_parser("list", help="List all specs with their latest results")

    args = parser.parse_args()

    if args.command == "history":
        cmd_history(args)
    elif args.command == "list":
        cmd_list(args)
    else:
        parser.print_help()


if __name__ == "__main__":
    main()
