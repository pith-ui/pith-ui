describe('AccessibleIcon', () => {
    beforeEach(() => {
        cy.visit('/accessible-icon');
    });

    // ── 1. Accessibility Semantics ──────────────────────────
    // accessible-icon-bp-2: verify aria-hidden and focusable attributes

    describe('accessibility', () => {
        it('child element has aria-hidden="true"', () => {
            // accessible-icon-bp-2
            cy.findByTestId('icon-svg').should('have.attr', 'aria-hidden', 'true');
        });

        it('child element has focusable="false"', () => {
            // accessible-icon-bp-2
            cy.findByTestId('icon-svg').should('have.attr', 'focusable', 'false');
        });

        it('non-SVG child also has aria-hidden="true"', () => {
            // accessible-icon-bp-2
            cy.findByTestId('icon-span').should('have.attr', 'aria-hidden', 'true');
        });

        it('non-SVG child also has focusable="false"', () => {
            // accessible-icon-bp-2
            cy.findByTestId('icon-span').should('have.attr', 'focusable', 'false');
        });
    });

    // ── 2. Visually Hidden Label ──────────────────────────────
    // accessible-icon-dcnv-1: verify the behavioral contract of the visually hidden label

    describe('visually hidden label', () => {
        it('renders a visually hidden element with the label text for the first icon', () => {
            // accessible-icon-dcnv-1
            cy.contains('Close dialog').should('exist');
        });

        it('renders a visually hidden element with the label text for the second icon', () => {
            // accessible-icon-dcnv-1
            cy.contains('Settings').should('exist');
        });

        it('visually hidden label is not visible on screen', () => {
            // accessible-icon-dcnv-1
            // The VisuallyHidden component uses styles to hide the element visually
            // but keep it accessible to screen readers
            cy.contains('Close dialog')
                .should('have.css', 'position', 'absolute')
                .and('have.css', 'overflow', 'hidden');
        });
    });

    // ── 3. Initial HTML (SSR) ─────────────────────────────────
    // accessible-icon-dsr-1: verify correct initial HTML attributes

    describe('initial HTML', () => {
        it('icon has correct ARIA attributes on first paint', () => {
            // accessible-icon-dsr-1
            cy.findByTestId('icon-svg')
                .should('have.attr', 'aria-hidden', 'true')
                .and('have.attr', 'focusable', 'false');
        });

        it('label text is present in DOM on first paint', () => {
            // accessible-icon-dsr-1
            cy.contains('Close dialog').should('exist');
            cy.contains('Settings').should('exist');
        });
    });

    // ── Axe Accessibility Audit ─────────────────────────────

    describe('axe audit', () => {
        it('no violations in default state', () => {
            cy.checkComponentA11y();
        });
    });
});
