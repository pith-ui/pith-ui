# ── Unit & Wasm Tests ──────────────────────────────────────

# Run all unit tests for leptos primitives
test_leptos_unit:
    cargo test -p radix-leptos-primitives --all-features --locked --release

# Run all wasm-bindgen tests for leptos primitives (headless Chrome)
test_leptos_wasm:
    wasm-pack test --headless --chrome --release packages/primitives/leptos

# Run all leptos tests: unit, wasm, and e2e
test_leptos_all: test_leptos_unit test_leptos_wasm test_leptos

# Reference App — cross-framework E2E testing harness
# ── Dev servers ──────────────────────────────────────────

# Start the React dev server on :3000
[working-directory('reference_app/react')]
dev_react:
    pnpm dev

# Start the Leptos dev server on :3000
[working-directory('reference_app/leptos')]
dev_leptos:
    trunk serve --port 3000

# ── Cypress ──────────────────────────────────────────────

# Open the Cypress GUI
[working-directory('reference_app')]
cy_open:
    pnpm cy:open

# Run all Cypress tests
[working-directory('reference_app')]
cy_run:
    pnpm cy:run

# Run Cypress tests for a single component
[working-directory('reference_app')]
test component:
    pnpm cypress run --spec "cypress/e2e/{{ component }}.cy.js"

# ── Helpers ─────────────────────────────────────────────

# Kill any process listening on port 3000
[private]
free_port:
    -lsof -ti :3000 | xargs kill -9

# Kill any running trunk process
[private]
kill_trunk:
    -pkill -9 trunk

# ── Story Books ──────────────────────────────────────────

# Start up the Leptos Story Book
[working-directory('stories/leptos')]
serve_leptos_storybook: kill_trunk
    trunk serve

# Start up the React Story Book
[working-directory('reference/react-radix-primitives')]
serve_react_storybook:
    pnpm install && pnpm --filter @repo/storybook dev

# ── Start server + test (single command) ─────────────────

# Path to post-processing script (relative to repo root)
save_results := justfile_directory() / "scripts/save_e2e_results.py"

# Start React server, run all tests, then shut down
[working-directory('reference_app')]
test_react: free_port
    pnpm cy:run:react 2>&1 | python3 {{ save_results }} react

# Start Leptos server, run all tests, then shut down
[working-directory('reference_app')]
test_leptos: kill_trunk free_port
    pnpm cy:run:leptos 2>&1 | python3 {{ save_results }} leptos

# Start React server, test one component, then shut down
[working-directory('reference_app')]
test_react_component component: free_port
    pnpm start-server-and-test react:dev http://localhost:3000 'cypress run --headless --spec "cypress/e2e/{{ component }}.cy.js"' 2>&1 | python3 {{ save_results }} react

# Start Leptos server, test one component, then shut down
[working-directory('reference_app')]
test_leptos_component component: kill_trunk free_port
    pnpm start-server-and-test leptos:dev http://localhost:3000 'cypress run --headless --spec "cypress/e2e/{{ component }}.cy.js"' 2>&1 | python3 {{ save_results }} leptos

# Start React server, test multiple components sequentially, then shut down

# Usage: just test_react_components dialog popover hover-card
[working-directory('reference_app')]
test_react_components +components: free_port
    #!/usr/bin/env bash
    specs=$(echo "{{ components }}" | tr ' ' '\n' | sed 's|.*|cypress/e2e/&.cy.js|' | paste -sd, -)
    pnpm start-server-and-test react:dev http://localhost:3000 "cypress run --headless --spec \"$specs\"" 2>&1 | python3 {{ save_results }} react

# Start Leptos server, test multiple components sequentially, then shut down

# Usage: just test_leptos_components dialog popover hover-card
[working-directory('reference_app')]
test_leptos_components +components: kill_trunk free_port
    #!/usr/bin/env bash
    specs=$(echo "{{ components }}" | tr ' ' '\n' | sed 's|.*|cypress/e2e/&.cy.js|' | paste -sd, -)
    pnpm start-server-and-test leptos:dev http://localhost:3000 "cypress run --headless --spec \"$specs\"" 2>&1 | python3 {{ save_results }} leptos

# ── Reference Experiments ─────────────────────────────────

# Kill any process listening on port 3001
[private]
free_port_experiments:
    -lsof -ti :3001 | xargs kill -9

# Start the experiments dev server on :3001
[working-directory('reference_experiments/leptos')]
dev_experiments: kill_trunk free_port_experiments
    trunk serve --port 3001

# Open the Cypress GUI for experiments
[working-directory('reference_experiments')]
cy_open_experiments:
    pnpm cy:open

# Run all experiment tests
[working-directory('reference_experiments')]
test_experiments: kill_trunk free_port_experiments
    pnpm cy:run:leptos

# Run a single experiment test
[working-directory('reference_experiments')]
test_experiment experiment: kill_trunk free_port_experiments
    pnpm start-server-and-test leptos:dev http://localhost:3001 'cypress run --headless --spec "cypress/e2e/{{ experiment }}.cy.js"' 2>&1
