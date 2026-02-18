describe('Checkbox', () => {
    // ── Helpers ──────────────────────────────────────────────

    function shouldBeChecked() {
        cy.findByRole('checkbox', {name: 'accept terms'}).should('have.attr', 'data-state', 'checked');
        cy.findByRole('checkbox', {name: 'accept terms'}).should('have.attr', 'aria-checked', 'true');
    }

    function shouldBeUnchecked() {
        cy.findByRole('checkbox', {name: 'accept terms'}).should('have.attr', 'data-state', 'unchecked');
        cy.findByRole('checkbox', {name: 'accept terms'}).should('have.attr', 'aria-checked', 'false');
    }

    function shouldBeIndeterminate() {
        cy.findByRole('checkbox', {name: 'accept terms'}).should('have.attr', 'data-state', 'indeterminate');
        cy.findByRole('checkbox', {name: 'accept terms'}).should('have.attr', 'aria-checked', 'mixed');
    }

    beforeEach(() => {
        cy.visit('/checkbox');
    });

    // ── 1. Accessibility Semantics ──────────────────────────

    describe('accessibility', () => {
        it('renders as a button with role="checkbox"', () => {
            cy.findByRole('checkbox', {name: 'accept terms'}).should('exist');
            cy.findByRole('checkbox', {name: 'accept terms'}).should('have.prop', 'tagName', 'BUTTON');
        });

        it('has aria-checked="false" initially', () => {
            cy.findByRole('checkbox', {name: 'accept terms'}).should('have.attr', 'aria-checked', 'false');
        });

        it('aria-checked toggles to "true" on click', () => {
            cy.findByRole('checkbox', {name: 'accept terms'}).click();
            cy.findByRole('checkbox', {name: 'accept terms'}).should('have.attr', 'aria-checked', 'true');
        });

        it('supports indeterminate state (aria-checked="mixed")', () => {
            cy.findByLabelText('indeterminate').click();
            cy.findByRole('checkbox', {name: 'accept terms'}).should('have.attr', 'aria-checked', 'mixed');
        });
    });

    // ── 2. Data Attributes ──────────────────────────────────

    describe('data attributes', () => {
        it('data-state is "unchecked" initially', () => {
            shouldBeUnchecked();
        });

        it('data-state toggles to "checked" on click', () => {
            cy.findByRole('checkbox', {name: 'accept terms'}).click();
            shouldBeChecked();
        });

        it('data-state toggles back to "unchecked" on second click', () => {
            cy.findByRole('checkbox', {name: 'accept terms'}).click();
            shouldBeChecked();
            cy.findByRole('checkbox', {name: 'accept terms'}).click();
            shouldBeUnchecked();
        });

        it('data-state is "indeterminate" when set', () => {
            cy.findByLabelText('indeterminate').click();
            shouldBeIndeterminate();
        });

        it('clicking indeterminate checkbox transitions to "checked"', () => {
            cy.findByLabelText('indeterminate').click();
            shouldBeIndeterminate();
            cy.findByRole('checkbox', {name: 'accept terms'}).click();
            shouldBeChecked();
        });

        it('data-disabled present when disabled', () => {
            cy.findByLabelText('disabled').click();
            cy.findByRole('checkbox', {name: 'accept terms'}).should('have.attr', 'data-disabled');
        });

        it('data-disabled absent when not disabled', () => {
            cy.findByRole('checkbox', {name: 'accept terms'}).should('not.have.attr', 'data-disabled');
        });
    });

    // ── 3. Keyboard Navigation ──────────────────────────────

    describe('keyboard navigation', () => {
        it('Space toggles checked state', () => {
            cy.findByRole('checkbox', {name: 'accept terms'}).focus();
            cy.realPress('Space');
            shouldBeChecked();
            cy.realPress('Space');
            shouldBeUnchecked();
        });

        it('Enter does NOT toggle (WAI-ARIA checkbox spec)', () => {
            cy.findByRole('checkbox', {name: 'accept terms'}).focus();
            cy.realPress('Enter');
            shouldBeUnchecked();
        });
    });

    // ── 4. Pointer Interaction ──────────────────────────────

    describe('pointer interaction', () => {
        it('click toggles checked/unchecked', () => {
            cy.findByRole('checkbox', {name: 'accept terms'}).click();
            shouldBeChecked();
            cy.findByRole('checkbox', {name: 'accept terms'}).click();
            shouldBeUnchecked();
        });
    });

    // ── 5. Disabled Variant ─────────────────────────────────

    describe('disabled variant', () => {
        beforeEach(() => {
            cy.findByLabelText('disabled').click();
        });

        it('click does not toggle when disabled', () => {
            cy.findByRole('checkbox', {name: 'accept terms'}).click({force: true});
            shouldBeUnchecked();
        });

        it('button has disabled attribute', () => {
            cy.findByRole('checkbox', {name: 'accept terms'}).should('be.disabled');
        });

        it('data-disabled is present', () => {
            cy.findByRole('checkbox', {name: 'accept terms'}).should('have.attr', 'data-disabled');
        });
    });

    // ── 6. Indicator ────────────────────────────────────────

    describe('indicator', () => {
        it('indicator is not visible when unchecked', () => {
            cy.findByRole('checkbox', {name: 'accept terms'}).find('span').should('not.exist');
        });

        it('indicator is visible when checked', () => {
            cy.findByRole('checkbox', {name: 'accept terms'}).click();
            cy.findByRole('checkbox', {name: 'accept terms'}).find('span').should('exist');
            cy.findByRole('checkbox', {name: 'accept terms'}).find('span').should('have.attr', 'data-state', 'checked');
        });

        it('indicator is visible when indeterminate', () => {
            cy.findByLabelText('indeterminate').click();
            cy.findByRole('checkbox', {name: 'accept terms'}).find('span').should('exist');
            cy.findByRole('checkbox', {name: 'accept terms'})
                .find('span')
                .should('have.attr', 'data-state', 'indeterminate');
        });

        it('indicator has data-disabled when checkbox is disabled and checked', () => {
            cy.findByRole('checkbox', {name: 'accept terms'}).click();
            shouldBeChecked();
            cy.findByLabelText('disabled').click();
            cy.findByRole('checkbox', {name: 'accept terms'}).find('span').should('have.attr', 'data-disabled');
        });
    });
});
