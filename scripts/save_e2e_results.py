#!/usr/bin/env python3
"""
Post-process Cypress E2E output: save per-spec results to .results/ for historical tracking.

Usage:
    <cypress command> 2>&1 | python3 scripts/save_e2e_results.py <framework>

Where <framework> is "leptos" or "react".

Reads Cypress stdout from stdin, echoes it to stdout (passthrough), and saves
per-spec result files to:

    .results/<framework>/<timestamp>/<spec-name>/result.txt
    .results/<framework>/<timestamp>/<spec-name>/*.png  (failure screenshots)

Each result.txt contains a structured header plus the full Cypress output for that spec.
"""

import json
import sys
import re
import shutil
from datetime import datetime
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
RESULTS_DIR = REPO_ROOT / ".results"
SCREENSHOTS_DIR = REPO_ROOT / "reference_app" / "cypress" / "screenshots"


def parse_specs(lines: list[str]) -> list[dict]:
    """Parse the final results table to extract per-spec summaries."""
    specs = []
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

    run_pattern = re.compile(r"Running:\s+" + re.escape(spec_name))
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


def copy_screenshots(spec_name: str, dest_dir: Path):
    """Copy failure screenshots for a spec into the result directory."""
    screenshot_source = SCREENSHOTS_DIR / spec_name
    if not screenshot_source.is_dir():
        return []

    copied = []
    for png in screenshot_source.glob("*.png"):
        dest = dest_dir / png.name
        shutil.copy2(png, dest)
        copied.append(dest)

    return copied


def save_spec_result(
    framework: str,
    spec: dict,
    spec_output: list[str],
    run_dir: Path,
) -> Path:
    """Save a single spec's results to <run_dir>/<spec-name>/result.txt"""
    spec_name = spec["spec"].replace(".cy.js", "")
    spec_dir = run_dir / spec_name
    spec_dir.mkdir(parents=True, exist_ok=True)

    result_file = spec_dir / "result.txt"

    header = (
        f"# E2E Result: {spec['spec']}\n"
        f"# Framework:  {framework}\n"
        f"# Timestamp:  {run_dir.name}\n"
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

    # Copy failure screenshots
    screenshots = copy_screenshots(spec["spec"], spec_dir)

    return result_file, screenshots


def update_runs_json(framework: str, timestamp: str, specs: list[dict], run_dir: Path):
    """Append this run's summary to .results/runs.json."""
    runs_file = RESULTS_DIR / "runs.json"

    if runs_file.exists():
        with open(runs_file) as f:
            runs = json.load(f)
    else:
        runs = {}

    spec_entries = {}
    for spec in specs:
        spec_name = spec["spec"].replace(".cy.js", "")
        result_path = str((run_dir / spec_name / "result.txt").relative_to(REPO_ROOT))
        spec_entries[spec_name] = {
            "framework": framework,
            "status": spec["status"].upper(),
            "tests": spec["tests"],
            "passing": spec["passing"],
            "failing": spec["failing"],
            "pending": spec["pending"],
            "skipped": spec["skipped"],
            "duration": spec["duration"],
            "result_path": result_path,
        }

    runs[timestamp] = {"specs": spec_entries}

    with open(runs_file, "w") as f:
        json.dump(runs, f, indent=4)
        f.write("\n")


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
        sys.exit(0)

    timestamp = datetime.now().strftime("%Y-%m-%d_%H-%M-%S")
    run_dir = RESULTS_DIR / framework / timestamp
    run_dir.mkdir(parents=True, exist_ok=True)

    saved = []

    for spec in specs:
        spec_output = extract_spec_output(all_lines, spec["spec"])
        result_file, screenshots = save_spec_result(framework, spec, spec_output, run_dir)
        saved.append((spec, result_file, screenshots))

    # Update runs.json summary
    update_runs_json(framework, timestamp, specs, run_dir)

    # Print summary
    print(f"\n{'─' * 60}", file=sys.stderr)
    print(f"Results saved to {run_dir.relative_to(REPO_ROOT)}/", file=sys.stderr)
    for spec, path, screenshots in saved:
        icon = "✔" if spec["status"] == "pass" else "✖"
        spec_name = spec["spec"].replace(".cy.js", "")
        ss_note = f"  +{len(screenshots)} screenshots" if screenshots else ""
        print(
            f"  {icon}  {spec_name:<35s} {spec['passing']}/{spec['tests']}{ss_note}",
            file=sys.stderr,
        )
    print(f"{'─' * 60}", file=sys.stderr)


if __name__ == "__main__":
    main()
