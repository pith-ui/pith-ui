#!/usr/bin/env python3
"""
Post-process Cypress E2E output: save per-spec results to .results/ for historical tracking.

Usage:
    <cypress command> 2>&1 | python3 scripts/save_e2e_results.py <framework>

Where <framework> is "leptos" or "react".

Reads Cypress stdout from stdin, echoes it to stdout (passthrough), and saves
per-spec result files to .results/<framework>/<spec-name>/<timestamp>.txt

Each result file contains the full output for that spec run, plus a summary header.

Exit code: mirrors the Cypress exit code (number of failing specs).
"""

import sys
import os
import re
from datetime import datetime
from pathlib import Path

RESULTS_DIR = Path(__file__).resolve().parent.parent / ".results"


def parse_specs(lines: list[str]) -> list[dict]:
    """Parse the final results table to extract per-spec summaries."""
    specs = []
    # Match lines like:
    #   │ ✔  dialog.cy.js                             00:11       47       47        -        -        - │
    #   │ ✖  attr-forwarding.cy.js                    01:51       83       75        5        3        - │
    pattern = re.compile(
        r"│\s+([✔✖])\s+(\S+\.cy\.js)\s+"
        r"(\S+)\s+"        # duration
        r"(\d+)\s+"        # tests
        r"(\d+)\s+"        # passing
        r"(\d+|-)\s+"      # failing
        r"(\d+|-)\s+"      # pending
        r"(\d+|-)\s+│"     # skipped
    )

    for line in lines:
        m = pattern.search(line)
        if m:
            failing = m.group(6)
            pending = m.group(7)
            skipped = m.group(8)
            specs.append({
                "status": "pass" if m.group(1) == "✔" else "fail",
                "spec": m.group(2),
                "duration": m.group(3),
                "tests": int(m.group(4)),
                "passing": int(m.group(5)),
                "failing": int(failing) if failing != "-" else 0,
                "pending": int(pending) if pending != "-" else 0,
                "skipped": int(skipped) if skipped != "-" else 0,
            })

    return specs


def extract_spec_output(lines: list[str], spec_name: str) -> list[str]:
    """Extract the output block for a specific spec from the full Cypress output."""
    output_lines = []
    capturing = False

    # Look for "Running:  <spec_name>" to start capture
    run_pattern = re.compile(r"Running:\s+" + re.escape(spec_name))
    # Stop at next "Running:" or "(Run Finished)" or the results table
    stop_pattern = re.compile(r"Running:\s+\S+\.cy\.js|^\s*\(Run Finished\)")

    for line in lines:
        if not capturing:
            if run_pattern.search(line):
                capturing = True
                output_lines.append(line)
        else:
            if stop_pattern.search(line):
                break
            output_lines.append(line)

    return output_lines


def save_spec_result(
    framework: str,
    spec: dict,
    spec_output: list[str],
    timestamp: str,
):
    """Save a single spec's results to .results/<framework>/<spec-name>/<timestamp>.txt"""
    spec_name = spec["spec"].replace(".cy.js", "")
    spec_dir = RESULTS_DIR / framework / spec_name
    spec_dir.mkdir(parents=True, exist_ok=True)

    result_file = spec_dir / f"{timestamp}.txt"

    header = (
        f"# E2E Result: {spec['spec']}\n"
        f"# Framework:  {framework}\n"
        f"# Timestamp:  {timestamp}\n"
        f"# Status:     {spec['status'].upper()}\n"
        f"# Tests:      {spec['tests']}\n"
        f"# Passing:    {spec['passing']}\n"
        f"# Failing:    {spec['failing']}\n"
        f"# Pending:    {spec['pending']}\n"
        f"# Skipped:    {spec['skipped']}\n"
        f"# Duration:   {spec['duration']}\n"
        f"#\n"
    )

    with open(result_file, "w") as f:
        f.write(header)
        f.write("\n")
        for line in spec_output:
            f.write(line)
            if not line.endswith("\n"):
                f.write("\n")

    return result_file


def main():
    if len(sys.argv) < 2:
        print("Usage: <command> | python3 scripts/save_e2e_results.py <framework>", file=sys.stderr)
        sys.exit(1)

    framework = sys.argv[1]
    if framework not in ("leptos", "react"):
        print(f"Unknown framework: {framework}. Use 'leptos' or 'react'.", file=sys.stderr)
        sys.exit(1)

    # Read stdin line-by-line, echo to stdout (passthrough)
    all_lines = []
    for line in sys.stdin:
        sys.stdout.write(line)
        sys.stdout.flush()
        all_lines.append(line.rstrip("\n"))

    # Parse specs from the results table
    specs = parse_specs(all_lines)

    if not specs:
        # No specs found — probably a build failure or no tests ran
        sys.exit(0)

    timestamp = datetime.now().strftime("%Y-%m-%d-%H-%M-%S")
    saved_files = []

    for spec in specs:
        spec_output = extract_spec_output(all_lines, spec["spec"])
        result_file = save_spec_result(framework, spec, spec_output, timestamp)
        saved_files.append((spec, result_file))

    # Print summary
    total_pass = sum(1 for s in specs if s["status"] == "pass")
    total_fail = sum(1 for s in specs if s["status"] == "fail")

    print(f"\n{'─' * 60}", file=sys.stderr)
    print(f"Results saved to .results/{framework}/", file=sys.stderr)
    for spec, path in saved_files:
        icon = "✔" if spec["status"] == "pass" else "✖"
        print(
            f"  {icon}  {spec['spec']:<35s} {spec['passing']}/{spec['tests']}  → {path.relative_to(RESULTS_DIR.parent)}",
            file=sys.stderr,
        )
    print(f"{'─' * 60}", file=sys.stderr)


if __name__ == "__main__":
    main()
