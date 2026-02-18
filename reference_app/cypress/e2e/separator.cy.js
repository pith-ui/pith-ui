describe('Separator', () => {
    // ── Helpers ──────────────────────────────────────────────

    function getHorizontal() {
        return cy.findByTestId('horizontal-separator');
    }

    function getVertical() {
        return cy.findByTestId('vertical-separator');
    }

    function getDecorative() {
        return cy.findByTestId('decorative-separator');
    }

    beforeEach(() => {
        cy.visit('/separator');
    });

    // ── 1. Accessibility Semantics ──────────────────────────

    describe('accessibility', () => {
        it('default separator has role="separator"', () => {
            getHorizontal().should('have.attr', 'role', 'separator');
        });

        it('default horizontal separator does not have aria-orientation', () => {
            // aria-orientation defaults to "horizontal" per WAI-ARIA spec,
            // so it is omitted when orientation is horizontal
            getHorizontal().should('not.have.attr', 'aria-orientation');
        });

        it('vertical separator has aria-orientation="vertical"', () => {
            getVertical().should('have.attr', 'aria-orientation', 'vertical');
        });

        it('vertical separator has role="separator"', () => {
            getVertical().should('have.attr', 'role', 'separator');
        });

        it('decorative separator has role="none"', () => {
            getDecorative().should('have.attr', 'role', 'none');
        });

        it('decorative separator does not have aria-orientation', () => {
            getDecorative().should('not.have.attr', 'aria-orientation');
        });
    });

    // ── 2. Data Attributes ──────────────────────────────────

    describe('data attributes', () => {
        it('default separator has data-orientation="horizontal"', () => {
            getHorizontal().should('have.attr', 'data-orientation', 'horizontal');
        });

        it('vertical separator has data-orientation="vertical"', () => {
            getVertical().should('have.attr', 'data-orientation', 'vertical');
        });

        it('decorative separator has data-orientation="horizontal"', () => {
            getDecorative().should('have.attr', 'data-orientation', 'horizontal');
        });
    });
});
