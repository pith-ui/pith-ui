describe('Switch', () => {
    // ── Helpers ──────────────────────────────────────────────

    function shouldBeChecked() {
        cy.findByRole('switch', {name: 'airplane mode'}).should('have.attr', 'data-state', 'checked');
        cy.findByRole('switch', {name: 'airplane mode'}).should('have.attr', 'aria-checked', 'true');
    }

    function shouldBeUnchecked() {
        cy.findByRole('switch', {name: 'airplane mode'}).should('have.attr', 'data-state', 'unchecked');
        cy.findByRole('switch', {name: 'airplane mode'}).should('have.attr', 'aria-checked', 'false');
    }

    beforeEach(() => {
        cy.visit('/switch');
    });

    // ── 1. Accessibility Semantics ──────────────────────────

    describe('accessibility', () => {
        it('has role="switch"', () => {
            cy.findByRole('switch', {name: 'airplane mode'}).should('exist');
        });

        it('has aria-checked="false" initially', () => {
            cy.findByRole('switch', {name: 'airplane mode'}).should('have.attr', 'aria-checked', 'false');
        });

        it('aria-checked toggles to "true" on click', () => {
            cy.findByRole('switch', {name: 'airplane mode'}).click();
            cy.findByRole('switch', {name: 'airplane mode'}).should('have.attr', 'aria-checked', 'true');
        });

        it('aria-checked toggles back to "false" on second click', () => {
            cy.findByRole('switch', {name: 'airplane mode'}).click();
            cy.findByRole('switch', {name: 'airplane mode'}).click();
            cy.findByRole('switch', {name: 'airplane mode'}).should('have.attr', 'aria-checked', 'false');
        });
    });

    // ── 2. Data Attributes ──────────────────────────────────

    describe('data attributes', () => {
        it('Root: data-state is "unchecked" initially', () => {
            shouldBeUnchecked();
        });

        it('Root: data-state toggles to "checked" on click', () => {
            cy.findByRole('switch', {name: 'airplane mode'}).click();
            shouldBeChecked();
        });

        it('Root: data-state toggles back to "unchecked" on second click', () => {
            cy.findByRole('switch', {name: 'airplane mode'}).click();
            cy.findByRole('switch', {name: 'airplane mode'}).click();
            shouldBeUnchecked();
        });

        it('Root: data-disabled present when disabled', () => {
            cy.findByLabelText('disabled').click();
            cy.findByRole('switch', {name: 'airplane mode'}).should('have.attr', 'data-disabled');
        });

        it('Root: data-disabled absent when not disabled', () => {
            cy.findByRole('switch', {name: 'airplane mode'}).should('not.have.attr', 'data-disabled');
        });

        it('Thumb: data-state matches root state (unchecked)', () => {
            cy.findByRole('switch', {name: 'airplane mode'}).find('span').should('have.attr', 'data-state', 'unchecked');
        });

        it('Thumb: data-state matches root state (checked)', () => {
            cy.findByRole('switch', {name: 'airplane mode'}).click();
            cy.findByRole('switch', {name: 'airplane mode'}).find('span').should('have.attr', 'data-state', 'checked');
        });

        it('Thumb: data-disabled present when disabled', () => {
            cy.findByLabelText('disabled').click();
            cy.findByRole('switch', {name: 'airplane mode'}).find('span').should('have.attr', 'data-disabled');
        });

        it('Thumb: data-disabled absent when not disabled', () => {
            cy.findByRole('switch', {name: 'airplane mode'})
                .find('span')
                .should('not.have.attr', 'data-disabled');
        });
    });

    // ── 3. Keyboard Navigation ──────────────────────────────

    describe('keyboard navigation', () => {
        it('Space toggles checked state', () => {
            cy.findByRole('switch', {name: 'airplane mode'}).focus();
            cy.realPress('Space');
            shouldBeChecked();
            cy.realPress('Space');
            shouldBeUnchecked();
        });

        it('Enter toggles checked state', () => {
            cy.findByRole('switch', {name: 'airplane mode'}).focus();
            cy.realPress('Enter');
            shouldBeChecked();
            cy.realPress('Enter');
            shouldBeUnchecked();
        });
    });

    // ── 4. Pointer Interaction ──────────────────────────────

    describe('pointer interaction', () => {
        it('click toggles checked/unchecked', () => {
            cy.findByRole('switch', {name: 'airplane mode'}).click();
            shouldBeChecked();
            cy.findByRole('switch', {name: 'airplane mode'}).click();
            shouldBeUnchecked();
        });
    });

    // ── 5. Disabled Variant ─────────────────────────────────

    describe('disabled variant', () => {
        beforeEach(() => {
            cy.findByLabelText('disabled').click();
        });

        it('click does not toggle when disabled', () => {
            cy.findByRole('switch', {name: 'airplane mode'}).click({force: true});
            shouldBeUnchecked();
        });

        it('has disabled attribute', () => {
            cy.findByRole('switch', {name: 'airplane mode'}).should('be.disabled');
        });

        it('has data-disabled', () => {
            cy.findByRole('switch', {name: 'airplane mode'}).should('have.attr', 'data-disabled');
        });
    });

    // ── 5b. Required Variant ──────────────────────────────────

    describe('required variant', () => {
        it('has aria-required="true" when required', () => {
            cy.findByLabelText('required').click();
            cy.findByRole('switch', {name: 'airplane mode'}).should('have.attr', 'aria-required', 'true');
        });
    });

    // ── 6. Controlled Variant ───────────────────────────────

    describe('controlled variant', () => {
        it('external checked checkbox controls switch state', () => {
            shouldBeUnchecked();
            cy.findByLabelText('checked').click();
            shouldBeChecked();
            cy.findByLabelText('checked').click();
            shouldBeUnchecked();
        });

        it('clicking switch updates external checked checkbox', () => {
            shouldBeUnchecked();
            cy.findByLabelText('checked').should('not.be.checked');
            cy.findByRole('switch', {name: 'airplane mode'}).click();
            shouldBeChecked();
            cy.findByLabelText('checked').should('be.checked');
            cy.findByRole('switch', {name: 'airplane mode'}).click();
            shouldBeUnchecked();
            cy.findByLabelText('checked').should('not.be.checked');
        });
    });

    // ── Attribute Forwarding ────────────────────────────────

    describe('attribute forwarding', () => {
        it('Root forwards className/class and custom data attributes', () => {
            cy.findByRole('switch', {name: 'airplane mode'})
                .should('have.class', 'switch-root')
                .and('have.attr', 'data-custom', 'switch-root-custom');
        });

        it('Thumb forwards className/class and custom data attributes', () => {
            cy.findByRole('switch', {name: 'airplane mode'})
                .find('[data-custom="switch-thumb-custom"]')
                .should('exist')
                .and('have.class', 'switch-thumb');
        });
    });
});
