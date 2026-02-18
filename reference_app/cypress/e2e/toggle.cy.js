describe('Toggle', () => {
    // ── Helpers ──────────────────────────────────────────────

    function shouldBeOn() {
        cy.findByRole('button', {name: 'toggle'}).should('have.attr', 'data-state', 'on');
    }

    function shouldBeOff() {
        cy.findByRole('button', {name: 'toggle'}).should('have.attr', 'data-state', 'off');
    }

    beforeEach(() => {
        cy.visit('/toggle');
    });

    // ── 1. Accessibility Semantics ──────────────────────────

    describe('accessibility', () => {
        it('renders as a button', () => {
            cy.findByRole('button', {name: 'toggle'}).should('exist');
        });

        it('has aria-pressed="false" initially', () => {
            cy.findByRole('button', {name: 'toggle'}).should('have.attr', 'aria-pressed', 'false');
        });

        it('aria-pressed toggles to "true" and back', () => {
            cy.findByRole('button', {name: 'toggle'}).click();
            cy.findByRole('button', {name: 'toggle'}).should('have.attr', 'aria-pressed', 'true');
            cy.findByRole('button', {name: 'toggle'}).click();
            cy.findByRole('button', {name: 'toggle'}).should('have.attr', 'aria-pressed', 'false');
        });
    });

    // ── 2. Data Attributes ──────────────────────────────────

    describe('data attributes', () => {
        it('data-state is "off" initially', () => {
            shouldBeOff();
        });

        it('data-state toggles to "on" and back', () => {
            cy.findByRole('button', {name: 'toggle'}).click();
            shouldBeOn();
            cy.findByRole('button', {name: 'toggle'}).click();
            shouldBeOff();
        });

        it('data-disabled present when disabled', () => {
            cy.findByLabelText('disabled').click();
            cy.findByRole('button', {name: 'toggle'}).should('have.attr', 'data-disabled');
        });

        it('data-disabled absent when not disabled', () => {
            cy.findByRole('button', {name: 'toggle'}).should('not.have.attr', 'data-disabled');
        });
    });

    // ── 3. Keyboard Navigation ──────────────────────────────

    describe('keyboard navigation', () => {
        it('Space toggles on and off', () => {
            cy.findByRole('button', {name: 'toggle'}).focus();
            cy.realPress('Space');
            shouldBeOn();
            cy.realPress('Space');
            shouldBeOff();
        });

        it('Enter toggles on and off', () => {
            cy.findByRole('button', {name: 'toggle'}).focus();
            cy.realPress('Enter');
            shouldBeOn();
            cy.realPress('Enter');
            shouldBeOff();
        });
    });

    // ── 4. Pointer Interaction ──────────────────────────────

    describe('pointer interaction', () => {
        it('click toggles on and off', () => {
            cy.findByRole('button', {name: 'toggle'}).click();
            shouldBeOn();
            cy.findByRole('button', {name: 'toggle'}).click();
            shouldBeOff();
        });
    });

    // ── 5. Disabled Variant ─────────────────────────────────

    describe('disabled variant', () => {
        beforeEach(() => {
            cy.findByLabelText('disabled').click();
        });

        it('click does not toggle when disabled', () => {
            cy.findByRole('button', {name: 'toggle'}).click({force: true});
            shouldBeOff();
        });

        it('button has disabled attribute', () => {
            cy.findByRole('button', {name: 'toggle'}).should('be.disabled');
        });

        it('data-disabled is present', () => {
            cy.findByRole('button', {name: 'toggle'}).should('have.attr', 'data-disabled');
        });
    });

    // ── 6. Controlled Variant ───────────────────────────────

    describe('controlled variant', () => {
        it('external pressed checkbox controls toggle state', () => {
            shouldBeOff();
            cy.findByLabelText('pressed').click();
            shouldBeOn();
            cy.findByRole('button', {name: 'toggle'}).should('have.attr', 'aria-pressed', 'true');
            cy.findByLabelText('pressed').click();
            shouldBeOff();
            cy.findByRole('button', {name: 'toggle'}).should('have.attr', 'aria-pressed', 'false');
        });

        it('clicking toggle updates external pressed checkbox', () => {
            shouldBeOff();
            cy.findByLabelText('pressed').should('not.be.checked');
            cy.findByRole('button', {name: 'toggle'}).click();
            shouldBeOn();
            cy.findByLabelText('pressed').should('be.checked');
            cy.findByRole('button', {name: 'toggle'}).click();
            shouldBeOff();
            cy.findByLabelText('pressed').should('not.be.checked');
        });
    });
});
