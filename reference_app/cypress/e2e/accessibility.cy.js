// ── Accessibility Audit ──────────────────────────────────────────────────────
//
// Runs axe-core against every component page in its default (closed/idle) state.
// This catches structural WCAG violations — missing labels, invalid ARIA, color
// contrast, heading order, etc. — across all primitives in a single spec.
//
// For state-specific a11y checks (open dialogs, expanded accordions, active
// menus), add cy.checkComponentA11y() calls to the individual component test
// files inside a `describe('accessibility', ...)` block.

const components = [
    'accessible-icon',
    'accordion',
    'alert-dialog',
    'aspect-ratio',
    'avatar',
    'calendar',
    'checkbox',
    'collapsible',
    'combobox',
    'context-menu',
    'dialog',
    'dropdown-menu',
    'form',
    'hover-card',
    'label',
    'menu',
    'menubar',
    'navigation-menu',
    'one-time-password-field',
    'password-toggle-field',
    'popover',
    'popper',
    'progress',
    'radio-group',
    'scroll-area',
    'select',
    'separator',
    'slider',
    'switch',
    'tabs',
    'time-field',
    'toast',
    'toggle',
    'toggle-group',
    'toolbar',
    'tooltip',
];

describe('Accessibility audit (default state)', () => {
    for (const component of components) {
        it(`${component} has no axe violations`, () => {
            cy.visit(`/${component}`);
            cy.checkComponentA11y();
        });
    }
});
