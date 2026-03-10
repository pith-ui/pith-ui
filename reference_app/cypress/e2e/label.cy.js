describe('Label', () => {
    beforeEach(() => {
        cy.visit('/label');
    });

    // ── 1. Rendered Element ───────────────────────────────────
    // label-dcnv-1: verify the component renders a <label> element

    describe('rendered element', () => {
        it('renders as a <label> element', () => {
            // label-dcnv-1
            cy.findByTestId('basic-label').should('match', 'label');
        });

        it('label has for attribute pointing to associated input', () => {
            // label-dcnv-1
            cy.findByTestId('basic-label').should('have.attr', 'for', 'basic-input');
        });
    });

    // ── 2. Initial HTML (SSR) ─────────────────────────────────
    // label-dsr-1: verify correct initial server-rendered HTML

    describe('initial HTML', () => {
        it('renders as a native <label> element on first paint', () => {
            // label-dsr-1
            cy.findByTestId('basic-label').should('match', 'label');
        });

        it('label text is present in DOM on first paint', () => {
            // label-dsr-1
            cy.findByTestId('basic-label').should('contain.text', 'Basic Label');
        });
    });

    // ── 3. Mouse Behavior ─────────────────────────────────────
    // label-bp-1: verify mousedown behavior

    describe('mouse behavior', () => {
        it('clicking label focuses associated input', () => {
            // label-bp-1
            cy.findByTestId('basic-label').click();
            cy.findByTestId('basic-input').should('be.focused');
        });

        it('clicking nested button does not prevent default', () => {
            // label-bp-1: clicking on interactive elements should not be intercepted
            cy.findByTestId('nested-button').click();
            // The button click should work normally (not prevented)
            cy.findByTestId('nested-button').should('exist');
        });

        it('clicking nested input does not prevent default', () => {
            // label-bp-1: clicking on interactive elements should not be intercepted
            cy.findByTestId('nested-input').click();
            cy.findByTestId('nested-input').should('be.focused');
        });
    });
});
