import '@testing-library/cypress/add-commands';
import 'cypress-real-events/support';

// ── Reactive Tracking Warning Detection ─────────────────────────────────────
//
// Same mechanism as reference_app: intercept console.warn to catch Leptos
// "outside a reactive tracking context" warnings.

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
