# ── Unit & Wasm Tests ──────────────────────────────────────

# Run all unit tests for leptos primitives
test_leptos_unit:
    cargo test -p 'radix-leptos-*' --all-features --locked --release

# Run all wasm-bindgen tests for leptos primitives (headless Chrome)
test_leptos_wasm:
    #!/usr/bin/env bash
    set -euo pipefail
    for toml in packages/primitives/leptos/*/Cargo.toml; do
        if grep -q 'wasm-bindgen-test' "$toml"; then
            dir=$(dirname "$toml")
            pkg=$(basename "$dir")
            echo "==> wasm-pack test $pkg"
            wasm-pack test --headless --chrome --release "$dir"
        fi
    done

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

# ── Start server + test (single command) ─────────────────

# Start React servexr, run all tests, then shut down
[working-directory('reference_app')]
test_react: free_port
    pnpm cy:run:react

# Start Leptos server, run all tests, then shut down
[working-directory('reference_app')]
test_leptos: kill_trunk free_port
    pnpm cy:run:leptos

# Start React server, test one component, then shut down
[working-directory('reference_app')]
test_react_component component: free_port
    pnpm start-server-and-test react:dev http://localhost:3000 'cypress run --headless --spec "cypress/e2e/{{ component }}.cy.js"' 2>&1

# Start Leptos server, test one component, then shut down
[working-directory('reference_app')]
test_leptos_component component: kill_trunk free_port
    pnpm start-server-and-test leptos:dev http://localhost:3000 'cypress run --headless --spec "cypress/e2e/{{ component }}.cy.js"' 2>&1

# Start React server, test multiple components sequentially, then shut down
# Usage: just test_react_components dialog popover hover-card
[working-directory('reference_app')]
test_react_components +components: free_port
    #!/usr/bin/env bash
    specs=$(echo "{{ components }}" | tr ' ' '\n' | sed 's|.*|cypress/e2e/&.cy.js|' | paste -sd, -)
    pnpm start-server-and-test react:dev http://localhost:3000 "cypress run --headless --spec \"$specs\"" 2>&1

# Start Leptos server, test multiple components sequentially, then shut down
# Usage: just test_leptos_components dialog popover hover-card
[working-directory('reference_app')]
test_leptos_components +components: kill_trunk free_port
    #!/usr/bin/env bash
    specs=$(echo "{{ components }}" | tr ' ' '\n' | sed 's|.*|cypress/e2e/&.cy.js|' | paste -sd, -)
    pnpm start-server-and-test leptos:dev http://localhost:3000 "cypress run --headless --spec \"$specs\"" 2>&1
