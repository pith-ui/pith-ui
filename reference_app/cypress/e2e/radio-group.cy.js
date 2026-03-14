describe('Radio Group', () => {
    // ── Helpers ──────────────────────────────────────────────

    function shouldBeChecked(name) {
        cy.findByRole('radio', {name}).should('have.attr', 'data-state', 'checked');
        cy.findByRole('radio', {name}).should('have.attr', 'aria-checked', 'true');
    }

    function shouldBeUnchecked(name) {
        cy.findByRole('radio', {name}).should('have.attr', 'data-state', 'unchecked');
        cy.findByRole('radio', {name}).should('have.attr', 'aria-checked', 'false');
    }

    beforeEach(() => {
        cy.visit('/radio-group');
    });

    // ── 1. Accessibility Semantics ──────────────────────────

    describe('accessibility', () => {
        it('root has role="radiogroup"', () => {
            cy.findByRole('radiogroup').should('exist');
        });

        it('items have role="radio"', () => {
            cy.findAllByRole('radio').should('have.length', 3);
        });

        it('items have aria-checked="false" initially', () => {
            cy.findByRole('radio', {name: 'Cat'}).should('have.attr', 'aria-checked', 'false');
            cy.findByRole('radio', {name: 'Dog'}).should('have.attr', 'aria-checked', 'false');
            cy.findByRole('radio', {name: 'Rabbit'}).should('have.attr', 'aria-checked', 'false');
        });

        it('clicking item sets aria-checked="true" on it and "false" on others', () => {
            cy.findByRole('radio', {name: 'Cat'}).click();
            cy.findByRole('radio', {name: 'Cat'}).should('have.attr', 'aria-checked', 'true');
            cy.findByRole('radio', {name: 'Rabbit'}).should('have.attr', 'aria-checked', 'false');
        });

        it('group has accessible label via aria-label', () => {
            cy.findByRole('radiogroup').should('have.attr', 'aria-label', 'Favourite pet');
        });
    });

    // ── 2. Data Attributes ──────────────────────────────────

    describe('data attributes', () => {
        it('items have data-state="unchecked" initially', () => {
            cy.findByRole('radio', {name: 'Cat'}).should('have.attr', 'data-state', 'unchecked');
            cy.findByRole('radio', {name: 'Dog'}).should('have.attr', 'data-state', 'unchecked');
            cy.findByRole('radio', {name: 'Rabbit'}).should('have.attr', 'data-state', 'unchecked');
        });

        it('clicking item sets data-state="checked" on it and "unchecked" on others', () => {
            cy.findByRole('radio', {name: 'Cat'}).click();
            shouldBeChecked('Cat');
            shouldBeUnchecked('Rabbit');

            cy.findByRole('radio', {name: 'Rabbit'}).click();
            shouldBeUnchecked('Cat');
            shouldBeChecked('Rabbit');
        });

        it('indicator has data-state matching its item when checked', () => {
            cy.findByRole('radio', {name: 'Cat'}).click();
            shouldBeChecked('Cat');
            cy.findByRole('radio', {name: 'Cat'}).find('span').should('have.attr', 'data-state', 'checked');
        });

        it('indicator is not rendered for unchecked items (no force mount)', () => {
            // Before any selection, no indicators should be visible
            cy.findByRole('radio', {name: 'Cat'}).find('span').should('not.exist');
        });

        it('disabled item has data-disabled', () => {
            cy.findByRole('radio', {name: 'Dog'}).should('have.attr', 'data-disabled');
        });

        it('non-disabled items do not have data-disabled', () => {
            cy.findByRole('radio', {name: 'Cat'}).should('not.have.attr', 'data-disabled');
            cy.findByRole('radio', {name: 'Rabbit'}).should('not.have.attr', 'data-disabled');
        });
    });

    // ── 3. Keyboard Navigation ──────────────────────────────

    describe('keyboard navigation', () => {
        it('Tab enters group and focuses first item', () => {
            // Ensure WASM app has rendered radio items before pressing Tab
            cy.findByRole('radio', {name: 'Cat'}).should('exist');
            cy.realPress('Tab');
            cy.findByRole('radio', {name: 'Cat'}).should('be.focused');
        });

        it('Tab focuses checked item when one is selected', () => {
            cy.findByRole('radio', {name: 'Rabbit'}).click();
            shouldBeChecked('Rabbit');
            // Tab away then back
            cy.realPress('Tab');
            cy.realPress(['Shift', 'Tab']);
            cy.findByRole('radio', {name: 'Rabbit'}).should('be.focused');
        });

        it('ArrowDown moves focus and checks next item', () => {
            cy.findByRole('radio', {name: 'Cat'}).focus();
            cy.realPress('ArrowDown');
            // Dog is disabled, so should skip to Rabbit
            cy.findByRole('radio', {name: 'Rabbit'}).should('be.focused');
            shouldBeChecked('Rabbit');
        });

        it('ArrowUp moves focus and checks previous item', () => {
            cy.findByRole('radio', {name: 'Rabbit'}).click();
            cy.findByRole('radio', {name: 'Rabbit'}).focus();
            cy.realPress('ArrowUp');
            // Dog is disabled, so should skip to Cat
            cy.findByRole('radio', {name: 'Cat'}).should('be.focused');
            shouldBeChecked('Cat');
        });

        it('ArrowRight moves focus and checks next item', () => {
            cy.findByRole('radio', {name: 'Cat'}).focus();
            cy.realPress('ArrowRight');
            // Dog is disabled, skip to Rabbit
            cy.findByRole('radio', {name: 'Rabbit'}).should('be.focused');
            shouldBeChecked('Rabbit');
        });

        it('ArrowLeft moves focus and checks previous item', () => {
            cy.findByRole('radio', {name: 'Rabbit'}).click();
            cy.findByRole('radio', {name: 'Rabbit'}).focus();
            cy.realPress('ArrowLeft');
            // Dog is disabled, skip to Cat
            cy.findByRole('radio', {name: 'Cat'}).should('be.focused');
            shouldBeChecked('Cat');
        });

        it('Space selects focused item', () => {
            cy.findByRole('radio', {name: 'Cat'}).focus();
            cy.realPress('Space');
            shouldBeChecked('Cat');
        });

        it('Arrow keys skip disabled items', () => {
            cy.findByRole('radio', {name: 'Cat'}).focus();
            cy.realPress('ArrowDown');
            // Should skip Dog (disabled) and land on Rabbit
            cy.findByRole('radio', {name: 'Rabbit'}).should('be.focused');
        });

        it('Arrow keys wrap around (down from last to first)', () => {
            cy.findByRole('radio', {name: 'Rabbit'}).click();
            cy.findByRole('radio', {name: 'Rabbit'}).focus();
            cy.realPress('ArrowDown');
            cy.findByRole('radio', {name: 'Cat'}).should('be.focused');
            shouldBeChecked('Cat');
        });

        it('Arrow keys wrap around (up from first to last)', () => {
            cy.findByRole('radio', {name: 'Cat'}).focus();
            cy.realPress('ArrowUp');
            cy.findByRole('radio', {name: 'Rabbit'}).should('be.focused');
            shouldBeChecked('Rabbit');
        });

        it('Enter does not select item (WAI-ARIA spec)', () => {
            cy.findByRole('radio', {name: 'Cat'}).focus();
            cy.realPress('Enter');
            shouldBeUnchecked('Cat');
        });
    });

    // ── 4. Pointer Interaction ──────────────────────────────

    describe('pointer interaction', () => {
        it('click selects item', () => {
            shouldBeUnchecked('Cat');
            cy.findByRole('radio', {name: 'Cat'}).click();
            shouldBeChecked('Cat');
        });

        it('click different item changes selection', () => {
            cy.findByRole('radio', {name: 'Cat'}).click();
            shouldBeChecked('Cat');
            cy.findByRole('radio', {name: 'Rabbit'}).click();
            shouldBeUnchecked('Cat');
            shouldBeChecked('Rabbit');
        });

        it('click disabled item does not select', () => {
            cy.findByRole('radio', {name: 'Dog'}).click({force: true});
            shouldBeUnchecked('Dog');
        });
    });

    // ── 5. Disabled Variant ─────────────────────────────────

    describe('disabled variant', () => {
        beforeEach(() => {
            cy.findByLabelText('disabled').click();
        });

        it('root has data-disabled', () => {
            cy.findByRole('radiogroup').should('have.attr', 'data-disabled');
        });

        it('all items have data-disabled', () => {
            cy.findByRole('radio', {name: 'Cat'}).should('have.attr', 'data-disabled');
            cy.findByRole('radio', {name: 'Dog'}).should('have.attr', 'data-disabled');
            cy.findByRole('radio', {name: 'Rabbit'}).should('have.attr', 'data-disabled');
        });

        it('clicking does not select any item', () => {
            cy.findByRole('radio', {name: 'Cat'}).click({force: true});
            shouldBeUnchecked('Cat');
            cy.findByRole('radio', {name: 'Rabbit'}).click({force: true});
            shouldBeUnchecked('Rabbit');
        });

        it('all items have disabled attribute', () => {
            cy.findByRole('radio', {name: 'Cat'}).should('be.disabled');
            cy.findByRole('radio', {name: 'Dog'}).should('be.disabled');
            cy.findByRole('radio', {name: 'Rabbit'}).should('be.disabled');
        });
    });

    // ── 6. Controlled Mode ───────────────────────────────────

    describe('controlled mode', () => {
        it('clicking item updates external state', () => {
            cy.findByTestId('radio-value').should('have.text', '');
            cy.findByRole('radio', {name: 'Cat'}).click();
            cy.findByTestId('radio-value').should('have.text', 'cat');
        });

        it('external state controls selection', () => {
            shouldBeUnchecked('Rabbit');
            cy.findByTestId('set-rabbit').click();
            shouldBeChecked('Rabbit');
        });

        it('clearing external state deselects all', () => {
            cy.findByRole('radio', {name: 'Cat'}).click();
            shouldBeChecked('Cat');
            cy.findByTestId('clear-value').click();
            shouldBeUnchecked('Cat');
            shouldBeUnchecked('Rabbit');
        });

        it('on_value_change fires with correct value on keyboard', () => {
            cy.findByTestId('clear-value').click();
            cy.findByRole('radio', {name: 'Cat'}).focus();
            cy.realPress('Space');
            cy.findByTestId('radio-value').should('have.text', 'cat');
        });
    });

    // ── Attribute Forwarding ────────────────────────────────

    describe('attribute forwarding', () => {
        it('Root forwards className/class and custom data attributes', () => {
            cy.findByRole('radiogroup')
                .should('have.class', 'radio-group-root')
                .and('have.attr', 'data-custom', 'radio-group-root-custom');
        });

        it('Item forwards className/class and custom data attributes', () => {
            cy.findByRole('radio', {name: 'Cat'})
                .should('have.class', 'radio-group-item')
                .and('have.attr', 'data-custom', 'radio-group-item-custom');
        });

        it('Indicator forwards className/class and custom data attributes when checked', () => {
            cy.findByRole('radio', {name: 'Cat'}).click();
            cy.findByRole('radio', {name: 'Cat'})
                .find('[data-custom="radio-group-indicator-custom"]')
                .should('exist')
                .and('have.class', 'radio-group-indicator');
        });
    });
});
