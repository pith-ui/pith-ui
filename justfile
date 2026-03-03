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
