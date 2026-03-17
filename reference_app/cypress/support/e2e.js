import '@testing-library/cypress/add-commands';
import 'cypress-real-events/support';

// ── Framework Detection ─────────────────────────────────────────────────────
//
// The justfile sets CYPRESS_FRAMEWORK=react|leptos when running tests.
// Cypress auto-exposes CYPRESS_* env vars via Cypress.env().
//
// Use in tests to skip cases that only apply to one framework:
//
//   it.skipForFramework('react', 'React omits aria-pressed')('has aria-pressed', () => {
//       ...
//   });
//

/**
 * Wrap `it()` to conditionally skip tests for a specific framework.
 *
 * Usage:
 *   it.skipForFramework('react', 'React does not implement aria-pressed')('has aria-pressed', () => {
 *       ...
 *   });
 *
 * When the current framework matches, the test is registered via `it.skip()`
 * so it shows as "pending" in the Cypress output — not as a failure.
 */
it.skipForFramework = (framework, reason) => {
    const current = Cypress.env('FRAMEWORK');
    if (current === framework) {
        return (title, fn) => it.skip(`${title} [skipped: ${reason}]`, fn);
    }
    return it;
};

// ── Reactive Tracking Warning Detection ─────────────────────────────────────
//
// Intercepts console.warn to catch Leptos "outside a reactive tracking context"
// warnings. These indicate signal reads that won't trigger reactive updates and
// should be fixed with .get_untracked() or by moving the read into a reactive
// closure.
//
// Usage:
//   cy.assertNoReactiveWarnings()   — assert + clear accumulated warnings
//
// Warnings are also automatically checked in afterEach, so any warnings that
// occur during a test will fail it even without explicit assertNoReactiveWarnings
// calls. Call it between actions to pinpoint exactly when a warning occurs.

Cypress.on('window:before:load', (win) => {
    win.__reactiveWarnings = [];

    const originalWarn = win.console.warn;
    win.console.warn = (...args) => {
        const msg = args.map((a) => String(a)).join(' ');
        if (msg.includes('outside a reactive tracking context')) {
            win.__reactiveWarnings.push(msg);
        }
        originalWarn.apply(win.console, args);
    };
});

Cypress.Commands.add('assertNoReactiveWarnings', () => {
    cy.window({log: false}).then((win) => {
        const warnings = win.__reactiveWarnings || [];
        if (warnings.length > 0) {
            const summary = warnings
                .map((w, i) => {
                    // Extract "At <file>:<line>:<col>" location from the verbose warning
                    const match = w.match(/At ([^,]+,)/);
                    const loc = match ? match[1].replace(/,$/, '') : w.substring(0, 120);
                    return `  ${i + 1}. ${loc}`;
                })
                .join('\n');
            win.__reactiveWarnings = [];
            throw new Error(`${warnings.length} reactive tracking warning(s) detected:\n${summary}`);
        }
    });
});

afterEach(() => {
    cy.assertNoReactiveWarnings();
});
