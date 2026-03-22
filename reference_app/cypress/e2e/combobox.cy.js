const framework = Cypress.env('FRAMEWORK');

describe('Combobox', () => {
    // ── Helpers ──────────────────────────────────────────────

    function getInput() {
        return cy.get('[data-testid="combobox-input"]');
    }

    function getTrigger() {
        return cy.get('[data-testid="combobox-trigger"]');
    }

    function shouldBeOpen() {
        getInput().should('have.attr', 'aria-expanded', 'true');
    }

    function shouldBeClosed() {
        getInput().should('have.attr', 'aria-expanded', 'false');
    }

    function openViaInput() {
        getInput().click();
        shouldBeOpen();
    }

    function openViaTrigger() {
        getTrigger().click();
        shouldBeOpen();
    }

    function getValue() {
        return cy.get('[data-testid="combobox-value"]');
    }

    // Multi-select helpers
    function getMultiInput() {
        return cy.get('[data-testid="multi-input"]');
    }

    function getMultiTrigger() {
        return cy.get('[data-testid="multi-trigger"]');
    }

    function multiShouldBeOpen() {
        getMultiInput().should('have.attr', 'aria-expanded', 'true');
    }

    function multiShouldBeClosed() {
        getMultiInput().should('have.attr', 'aria-expanded', 'false');
    }

    function getMultiValue() {
        return cy.get('[data-testid="multi-value"]');
    }

    beforeEach(() => {
        cy.visit('/combobox');
    });

    // ── 1. Accessibility Semantics ──────────────────────────

    describe('accessibility', () => {
        it('Input has role="combobox"', () => {
            getInput().should('have.attr', 'role', 'combobox');
        });

        it('Input has aria-expanded false when closed', () => {
            getInput().should('have.attr', 'aria-expanded', 'false');
        });

        it('Input has aria-expanded true when open', () => {
            openViaInput();
            getInput().should('have.attr', 'aria-expanded', 'true');
        });

        it('Input has aria-controls pointing to listbox', () => {
            openViaInput();
            getInput()
                .invoke('attr', 'aria-controls')
                .then((controlsId) => {
                    cy.get(`#${controlsId}`).should('have.attr', 'role', 'listbox');
                });
        });

        it('Popup list has role="listbox"', () => {
            openViaInput();
            cy.findByRole('listbox').should('exist');
        });

        it('Items have role="option"', () => {
            openViaInput();
            cy.findAllByRole('option').should('have.length.at.least', 1);
        });

        it('Groups have role="group"', () => {
            openViaInput();
            cy.findAllByRole('group').should('have.length', 2);
        });

        it('Groups are labelled by their label', () => {
            openViaInput();
            cy.findAllByRole('group')
                .first()
                .should('have.attr', 'aria-labelledby')
                .then((labelId) => {
                    cy.get(`#${labelId}`).should('have.text', 'Fruits');
                });
        });

        it('Selected item has aria-selected="true"', () => {
            openViaInput();
            cy.findByRole('option', {name: 'Apple'}).click();
            openViaInput();
            cy.findByRole('option', {name: 'Apple'}).should('have.attr', 'aria-selected', 'true');
        });

        it('Disabled item has aria-disabled="true"', () => {
            openViaInput();
            cy.findByRole('option', {name: 'Cherry'}).should('have.attr', 'aria-disabled', 'true');
        });

        it('Input has aria-autocomplete="list"', () => {
            getInput().should('have.attr', 'aria-autocomplete', 'list');
        });
    });

    // ── 2. Data Attributes ──────────────────────────────────

    describe('data attributes', () => {
        it('Item has data-highlighted when focused via keyboard', () => {
            openViaInput();
            cy.realPress('ArrowDown');
            cy.findAllByRole('option')
                .filter('[data-highlighted]')
                .should('have.length', 1);
        });

        it('Disabled item has data-disabled', () => {
            openViaInput();
            cy.findByRole('option', {name: 'Cherry'}).should('have.attr', 'data-disabled');
        });

        it('Selected item has data-selected (react) or data-state="checked" (leptos)', () => {
            openViaInput();
            cy.findByRole('option', {name: 'Apple'}).click();
            openViaInput();
            if (framework === 'react') {
                cy.findByRole('option', {name: 'Apple'}).should('have.attr', 'data-selected');
            } else {
                cy.findByRole('option', {name: 'Apple'}).should('have.attr', 'data-state', 'checked');
            }
        });
    });

    // ── 3. Keyboard Navigation ──────────────────────────────

    describe('keyboard navigation', () => {
        it('ArrowDown opens popup when closed', () => {
            getInput().focus();
            shouldBeClosed();
            cy.realPress('ArrowDown');
            shouldBeOpen();
        });

        it('ArrowDown navigates to next option', () => {
            openViaInput();
            cy.realPress('ArrowDown');
            // First non-disabled item should be highlighted
            cy.findAllByRole('option')
                .filter('[data-highlighted]')
                .should('have.length', 1);
            // Press again to move to next
            cy.realPress('ArrowDown');
            cy.findAllByRole('option')
                .filter('[data-highlighted]')
                .should('have.length', 1);
        });

        it('ArrowUp navigates to previous option', () => {
            openViaInput();
            cy.realPress('ArrowDown');
            cy.realPress('ArrowDown');
            cy.realPress('ArrowUp');
            cy.findAllByRole('option')
                .filter('[data-highlighted]')
                .should('have.length', 1);
        });

        it('Enter selects highlighted option and closes', () => {
            openViaInput();
            cy.realPress('ArrowDown');
            cy.realPress('Enter');
            shouldBeClosed();
            getValue().should('not.have.text', '(none)');
        });

        it('Escape closes popup', () => {
            openViaInput();
            cy.realPress('Escape');
            shouldBeClosed();
        });

        it('Enter does not select disabled item', () => {
            openViaInput();
            // Navigate to Cherry (disabled) - index 3 in fruits
            cy.realPress('ArrowDown'); // Apple (0)
            cy.realPress('ArrowDown'); // Avocado (1)
            cy.realPress('ArrowDown'); // Banana (2)
            cy.realPress('ArrowDown'); // Cherry (3, disabled)
            cy.realPress('Enter');
            // Cherry should not be selected
            getValue().should('have.text', '(none)');
        });

        it('Typing in input filters items', () => {
            getInput().focus();
            cy.realType('app');
            shouldBeOpen();
            cy.findAllByRole('option').should('have.length', 1);
            cy.findByRole('option', {name: 'Apple'}).should('exist');
        });
    });

    // ── 4. Pointer Interaction ──────────────────────────────

    describe('pointer interaction', () => {
        it('Click trigger toggles popup', () => {
            shouldBeClosed();
            getTrigger().click();
            shouldBeOpen();
            getTrigger().click();
            shouldBeClosed();
        });

        it('Click input opens popup', () => {
            shouldBeClosed();
            getInput().click();
            shouldBeOpen();
        });

        it('Click item selects and closes', () => {
            openViaInput();
            cy.findByRole('option', {name: 'Apple'}).click();
            shouldBeClosed();
            getValue().should('have.text', 'Apple');
        });

        it('Click outside closes popup', () => {
            openViaInput();
            cy.get('[data-testid="outside-button"]').click({force: true});
            shouldBeClosed();
        });

        it('Disabled item cannot be selected via click', () => {
            openViaInput();
            cy.findByRole('option', {name: 'Cherry'}).click({force: true});
            getValue().should('have.text', '(none)');
        });
    });

    // ── 5. Focus Management ─────────────────────────────────

    describe('focus management', () => {
        it('Input retains focus when popup opens', () => {
            getInput().click();
            shouldBeOpen();
            getInput().should('be.focused');
        });

        it('Focus returns to input after item selection', () => {
            openViaInput();
            cy.findByRole('option', {name: 'Apple'}).click();
            getInput().should('be.focused');
        });

        it('Focus returns to input on Escape', () => {
            openViaInput();
            cy.realPress('Escape');
            getInput().should('be.focused');
        });
    });

    // ── 6. Single-Select Variants ───────────────────────────

    describe('single-select variants', () => {
        it('Selecting updates displayed value', () => {
            getValue().should('have.text', '(none)');
            openViaInput();
            cy.findByRole('option', {name: 'Banana'}).click();
            getValue().should('have.text', 'Banana');
        });

        it('Input text updates to selected value', () => {
            openViaInput();
            cy.findByRole('option', {name: 'Banana'}).click();
            getInput().should('have.value', 'Banana');
        });

        it('Disabled combobox prevents interaction', () => {
            cy.findByLabelText('disabled').click();
            getInput().should('be.disabled');
        });

        it('Clear button clears selection', () => {
            openViaInput();
            cy.findByRole('option', {name: 'Apple'}).click();
            getValue().should('have.text', 'Apple');
            cy.get('[data-testid="combobox-clear"]').click();
            getValue().should('have.text', '(none)');
        });

        it('Empty state shown when filter matches nothing', () => {
            getInput().focus();
            cy.realType('zzzzz');
            shouldBeOpen();
            cy.findAllByRole('option').should('have.length', 0);
            cy.get('[data-testid="combobox-empty"]').should('be.visible');
        });

        it('Default value combobox starts with value', () => {
            cy.get('[data-testid="default-input"]').should('have.value', 'Banana');
        });
    });

    // ── 7. Multi-Select ─────────────────────────────────────

    describe('multi-select', () => {
        it('Can select multiple items', () => {
            getMultiInput().click();
            multiShouldBeOpen();
            cy.findByRole('option', {name: 'Apple'}).click();
            // Popup stays open in multi-select
            multiShouldBeOpen();
            cy.findByRole('option', {name: 'Banana'}).click();
            getMultiValue().should('contain.text', 'Apple');
            getMultiValue().should('contain.text', 'Banana');
        });

        it('Chips display selected values', () => {
            getMultiInput().click();
            cy.findByRole('option', {name: 'Apple'}).click();
            cy.findByRole('option', {name: 'Banana'}).click();
            cy.realPress('Escape');
            cy.get('[data-testid="multi-chip"]').should('have.length', 2);
        });

        it('Chip remove button deselects item', () => {
            getMultiInput().click();
            cy.findByRole('option', {name: 'Apple'}).click();
            cy.findByRole('option', {name: 'Banana'}).click();
            cy.realPress('Escape');
            cy.get('[data-testid="multi-chip"]').should('have.length', 2);
            // Remove Apple
            cy.findByLabelText('Remove Apple').click();
            cy.get('[data-testid="multi-chip"]').should('have.length', 1);
            getMultiValue().should('not.contain.text', 'Apple');
            getMultiValue().should('contain.text', 'Banana');
        });

        it('Clicking selected item deselects it in multi-select', () => {
            getMultiInput().click();
            cy.findByRole('option', {name: 'Apple'}).click();
            getMultiValue().should('contain.text', 'Apple');
            cy.findByRole('option', {name: 'Apple'}).click();
            getMultiValue().should('have.text', '(none)');
        });

        it('Multi-select listbox has aria-multiselectable', () => {
            getMultiInput().click();
            multiShouldBeOpen();
            cy.findByRole('listbox').should('have.attr', 'aria-multiselectable', 'true');
        });
    });
});
