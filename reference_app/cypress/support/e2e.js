import '@testing-library/cypress/add-commands';
import 'cypress-real-events/support';
import 'cypress-axe';

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

/**
 * Returns a CSS custom property name, adjusting prefix based on framework.
 * React Radix uses `--radix-<name>`, Leptos drops the brand prefix: `--<name>`.
 *
 * Usage:
 *   getComputedStyle(el).getPropertyValue(cssVar('accordion-content-height'))
 */
globalThis.cssVar = (name) => {
    return Cypress.env('FRAMEWORK') === 'react' ? `--radix-${name}` : `--${name}`;
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

// ── Accessibility (axe-core via cypress-axe) ────────────────────────────────
//
// Inject axe-core once per page load. cy.checkA11y() is provided by
// cypress-axe and runs the axe accessibility audit against the current DOM.
//
// Usage in tests:
//
//   describe('accessibility', () => {
//       it('has no axe violations in default state', () => {
//           cy.checkComponentA11y();
//       });
//
//       it('has no axe violations when open', () => {
//           cy.findByRole('button', {name: 'open'}).click();
//           cy.checkComponentA11y();
//       });
//   });
//
// cy.checkComponentA11y() is a thin wrapper that injects axe-core, runs the
// audit, and logs individual violations for easier debugging. It handles
// injection automatically so tests don't need to call cy.injectAxe() — this
// is important because cy.visit() wipes the injected script.
//
// Pass an optional CSS selector to scope the check, or an axe options object.

/**
 * Run an axe accessibility audit with detailed violation logging.
 * Automatically injects axe-core into the current page before running.
 *
 * @param {string} [context] - Optional CSS selector to scope the audit
 * @param {object} [options] - Optional axe-core run options (rules, runOnly, etc.)
 */
Cypress.Commands.add('checkComponentA11y', (context, options) => {
    cy.injectAxe();

    const opts = {
        runOnly: {type: 'tag', values: ['wcag2a', 'wcag2aa', 'best-practice']},
        rules: {
            // Page-level structure rules — these flag the test harness, not the
            // primitives under test. The reference_app pages are minimal fixtures
            // that intentionally omit full page scaffolding (h1, landmarks, etc.).
            'page-has-heading-one': {enabled: false},
            'landmark-one-main': {enabled: false},
            'region': {enabled: false},
        },
        ...options,
    };

    cy.checkA11y(
        context || null,
        opts,
        (violations) => {
            const summary = violations
                .map((v) => {
                    const nodes = v.nodes.map((n) => `    ${n.html}`).join('\n');
                    return `[${v.impact}] ${v.id}: ${v.help}\n${nodes}`;
                })
                .join('\n\n');
            Cypress.log({
                name: 'a11y',
                message: `${violations.length} violation(s)`,
                consoleProps: () => ({violations}),
            });
            throw new Error(`${violations.length} accessibility violation(s):\n\n${summary}`);
        },
    );
});
