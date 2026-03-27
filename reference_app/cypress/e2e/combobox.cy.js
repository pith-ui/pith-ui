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
            // Remove first chip (Apple)
            cy.get('[data-testid="multi-chip-remove"]').first().click();
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

        it('Input text clears after selecting item in multi-select', () => {
            // Open multi-select, select an item, verify input clears
            getMultiInput().click();
            multiShouldBeOpen();
            cy.findByRole('option', {name: 'Apple'}).click();
            getMultiValue().should('contain.text', 'Apple');
            // Input should be cleared after selection in multi-select
            getMultiInput().should('have.value', '');
        });

        it.skipForFramework('react', 'Base UI uses built-in chip backspace with different semantics')('Backspace removes last chip when input is empty', () => {
            getMultiInput().click();
            cy.findByRole('option', {name: 'Apple'}).click();
            cy.findByRole('option', {name: 'Banana'}).click();
            cy.realPress('Escape');
            getMultiValue().should('contain.text', 'Apple');
            getMultiValue().should('contain.text', 'Banana');
            getMultiInput().focus();
            // Single backspace removes last chip directly
            cy.realPress('Backspace');
            getMultiValue().should('contain.text', 'Apple');
            getMultiValue().should('not.contain.text', 'Banana');
        });

        it('Tab away closes popup and clears input text', () => {
            getMultiInput().focus();
            cy.realType('kiwi');
            multiShouldBeOpen();
            cy.realPress('Tab');
            multiShouldBeClosed();
            getMultiInput().should('have.value', '');
        });

        it('Chip remove buttons are not in tab order', () => {
            getMultiInput().click();
            cy.findByRole('option', {name: 'Apple'}).click();
            cy.findByRole('option', {name: 'Banana'}).click();
            cy.realPress('Escape');
            getMultiInput().should('be.focused');
            // Tab should skip past chip remove buttons to the next external element
            cy.realPress('Tab');
            getMultiInput().should('not.be.focused');
            // Should NOT be on a chip remove button
            cy.focused().should('not.have.attr', 'data-testid', 'multi-chip-remove');
        });

        it.skipForFramework('react', 'React uses Base UI chip primitives with different highlight mechanism')('ArrowLeft loops through chips and back to input', () => {
            getMultiInput().click();
            cy.findByRole('option', {name: 'Apple'}).click();
            cy.findByRole('option', {name: 'Banana'}).click();
            cy.realPress('Escape');
            getMultiInput().should('be.focused');
            // ArrowLeft from input highlights last chip (Banana)
            cy.realPress('ArrowLeft');
            cy.get('[data-testid="multi-chip"]').last().should('have.attr', 'data-highlighted');
            // ArrowLeft again highlights first chip (Apple)
            cy.realPress('ArrowLeft');
            cy.get('[data-testid="multi-chip"]').first().should('have.attr', 'data-highlighted');
            // ArrowLeft from first chip loops back to input (no chip highlighted)
            cy.realPress('ArrowLeft');
            cy.get('[data-testid="multi-chip"]').filter('[data-highlighted]').should('have.length', 0);
            // ArrowLeft again loops to last chip
            cy.realPress('ArrowLeft');
            cy.get('[data-testid="multi-chip"]').last().should('have.attr', 'data-highlighted');
        });

        it.skipForFramework('react', 'React uses Base UI chip primitives with different highlight mechanism')('ArrowRight is terminal at input', () => {
            getMultiInput().click();
            cy.findByRole('option', {name: 'Apple'}).click();
            cy.findByRole('option', {name: 'Banana'}).click();
            cy.realPress('Escape');
            getMultiInput().should('be.focused');
            // Navigate to last chip
            cy.realPress('ArrowLeft');
            cy.get('[data-testid="multi-chip"]').last().should('have.attr', 'data-highlighted');
            // ArrowRight returns to input
            cy.realPress('ArrowRight');
            cy.get('[data-testid="multi-chip"]').filter('[data-highlighted]').should('have.length', 0);
            // ArrowRight at input has no further effect (stays at input, no chips highlighted)
            cy.realPress('ArrowRight');
            cy.get('[data-testid="multi-chip"]').filter('[data-highlighted]').should('have.length', 0);
        });

        it('Enter selects item and keeps popup open in multi-select', () => {
            getMultiInput().click();
            multiShouldBeOpen();
            cy.realPress('ArrowDown');
            cy.realPress('Enter');
            // Popup stays open for more selections
            multiShouldBeOpen();
            // Item should be selected
            getMultiValue().should('not.have.text', '(none)');
        });

        it('Click on item adds it and keeps popup open in multi-select', () => {
            getMultiInput().click();
            multiShouldBeOpen();
            cy.findByRole('option', {name: 'Apple'}).click();
            // Popup stays open after click selection in multi-select
            multiShouldBeOpen();
            getMultiValue().should('contain.text', 'Apple');
            // Can select another item
            cy.findByRole('option', {name: 'Banana'}).click();
            multiShouldBeOpen();
            getMultiValue().should('contain.text', 'Apple');
            getMultiValue().should('contain.text', 'Banana');
        });

        it.skipForFramework('react', 'React uses Base UI chip primitives with different highlight mechanism')('Placeholder hidden when items are selected', () => {
            // Initially placeholder is present
            getMultiInput().invoke('attr', 'placeholder').should('exist');
            getMultiInput().click();
            cy.findByRole('option', {name: 'Apple'}).click();
            cy.realPress('Escape');
            // Placeholder should be empty or absent when chips exist
            getMultiInput().then(($input) => {
                const placeholder = $input.attr('placeholder');
                expect(!placeholder || placeholder === '').to.be.true;
            });
        });

        it.skipForFramework('react', 'React uses Base UI chip primitives with different highlight mechanism')('ArrowLeft to chip clears input text and hides cursor', () => {
            getMultiInput().click();
            cy.findByRole('option', {name: 'Apple'}).click();
            cy.realPress('Escape');
            getMultiInput().should('be.focused');
            // ArrowLeft from empty input to chip should set data-chip-highlighted
            cy.realPress('ArrowLeft');
            getMultiInput().should('have.attr', 'data-chip-highlighted');
            // Return to input clears chip highlight
            cy.realPress('ArrowRight');
            getMultiInput().should('not.have.attr', 'data-chip-highlighted');
        });

        it.skipForFramework('react', 'React uses Base UI chip primitives with different highlight mechanism')('ArrowLeft to chip closes popup if open', () => {
            getMultiInput().click();
            cy.findByRole('option', {name: 'Apple'}).click();
            // Popup is still open after click selection
            multiShouldBeOpen();
            // ArrowLeft moves to chip — popup should close
            cy.realPress('ArrowLeft');
            multiShouldBeClosed();
            cy.get('[data-testid="multi-chip"]').last().should('have.attr', 'data-highlighted');
        });

        it.skipForFramework('react', 'React uses Base UI chip primitives with different highlight mechanism')('ArrowDown/Up from chip opens popup and returns focus to input', () => {
            getMultiInput().click();
            cy.findByRole('option', {name: 'Apple'}).click();
            cy.realPress('Escape');
            multiShouldBeClosed();
            // Navigate to chip
            cy.realPress('ArrowLeft');
            cy.get('[data-testid="multi-chip"]').last().should('have.attr', 'data-highlighted');
            // ArrowDown should open popup and clear chip highlight
            cy.realPress('ArrowDown');
            multiShouldBeOpen();
            cy.get('[data-testid="multi-chip"]').filter('[data-highlighted]').should('have.length', 0);
            // Close and navigate to chip again
            cy.realPress('Escape');
            cy.realPress('ArrowLeft');
            cy.get('[data-testid="multi-chip"]').last().should('have.attr', 'data-highlighted');
            // ArrowUp should also open popup and clear chip highlight
            cy.realPress('ArrowUp');
            multiShouldBeOpen();
            cy.get('[data-testid="multi-chip"]').filter('[data-highlighted]').should('have.length', 0);
        });

        it('Enter in multi-select preserves highlight on selected item', () => {
            getMultiInput().click();
            multiShouldBeOpen();
            cy.realPress('ArrowDown'); // highlight first item
            cy.realPress('ArrowDown'); // highlight second item
            // Get the highlighted item's aria id
            getMultiInput().invoke('attr', 'aria-activedescendant').then((activeId) => {
                expect(activeId).to.not.be.empty;
                cy.realPress('Enter');
                // Same item should still be highlighted after selection
                getMultiInput().should('have.attr', 'aria-activedescendant', activeId);
            });
        });

        it.skipForFramework('react', 'React uses Base UI chip primitives with different semantics')('Successive backspaces remove chips one at a time', () => {
            getMultiInput().click();
            cy.findByRole('option', {name: 'Apple'}).click();
            cy.findByRole('option', {name: 'Banana'}).click();
            cy.findByRole('option', {name: 'Grape'}).click();
            cy.realPress('Escape');
            getMultiInput().should('be.focused');
            getMultiInput().should('have.value', '');
            cy.get('[data-testid="multi-chip"]').should('have.length', 3);
            // First backspace removes Grape (last)
            cy.realPress('Backspace');
            cy.get('[data-testid="multi-chip"]').should('have.length', 2);
            getMultiValue().should('not.contain.text', 'Grape');
            // Second backspace removes Banana
            cy.realPress('Backspace');
            cy.get('[data-testid="multi-chip"]').should('have.length', 1);
            getMultiValue().should('not.contain.text', 'Banana');
            getMultiValue().should('contain.text', 'Apple');
        });
    });

    // ── 8. Auto-Highlight ──────────────────────────────────

    describe('auto-highlight', () => {
        function getAutoInput() {
            return cy.get('[data-testid="autohighlight-input"]');
        }

        function getAutoTrigger() {
            return cy.get('[data-testid="autohighlight-trigger"]');
        }

        function autoShouldBeOpen() {
            getAutoInput().should('have.attr', 'aria-expanded', 'true');
        }

        function autoShouldBeClosed() {
            getAutoInput().should('have.attr', 'aria-expanded', 'false');
        }

        function getAutoValue() {
            return cy.get('[data-testid="autohighlight-value"]');
        }

        it('Typing highlights the first matching item', () => {
            getAutoInput().focus();
            cy.realType('a');
            autoShouldBeOpen();
            // First matching item should be highlighted
            cy.findAllByRole('option')
                .filter('[data-highlighted]')
                .should('have.length', 1)
                .first()
                .should('contain.text', 'Apple');
        });

        it('Typing a different query updates the highlight to new first match', () => {
            getAutoInput().focus();
            cy.realType('ban');
            autoShouldBeOpen();
            cy.findAllByRole('option')
                .filter('[data-highlighted]')
                .should('have.length', 1)
                .first()
                .should('contain.text', 'Banana');
        });

        it('Clearing the input removes the highlight', () => {
            getAutoInput().focus();
            cy.realType('a');
            autoShouldBeOpen();
            cy.findAllByRole('option')
                .filter('[data-highlighted]')
                .should('have.length', 1);
            // Clear by selecting all and deleting
            getAutoInput().clear();
            // Highlight should be gone
            getAutoInput().should('have.value', '');
        });

        it('Enter selects the auto-highlighted item', () => {
            getAutoValue().should('have.text', '(none)');
            getAutoInput().focus();
            cy.realType('man');
            autoShouldBeOpen();
            // Mango should be highlighted
            cy.findAllByRole('option')
                .filter('[data-highlighted]')
                .should('have.length', 1)
                .first()
                .should('contain.text', 'Mango');
            cy.realPress('Enter');
            autoShouldBeClosed();
            getAutoValue().should('have.text', 'Mango');
        });

        it('Keyboard navigation overrides auto-highlight', () => {
            getAutoInput().focus();
            cy.realType('a');
            autoShouldBeOpen();
            // Auto-highlight is on first item (Apple)
            cy.findAllByRole('option')
                .filter('[data-highlighted]')
                .should('have.length', 1)
                .first()
                .should('contain.text', 'Apple');
            // ArrowDown should move to next item
            cy.realPress('ArrowDown');
            cy.findAllByRole('option')
                .filter('[data-highlighted]')
                .should('have.length', 1)
                .first()
                .should('contain.text', 'Avocado');
        });

        it('No items highlighted when query matches nothing', () => {
            getAutoInput().focus();
            cy.realType('zzzzz');
            autoShouldBeOpen();
            cy.findAllByRole('option').should('have.length', 0);
        });

        it('Without auto_highlight, typing does not highlight first item', () => {
            // Use the regular combobox (which does NOT have auto_highlight)
            getInput().focus();
            cy.realType('a');
            shouldBeOpen();
            cy.findAllByRole('option')
                .filter('[data-highlighted]')
                .should('have.length', 0);
        });
    });

    // ── Axe Accessibility Audit ─────────────────────────────

    describe('axe audit', () => {
        it('no violations in closed state', () => {
            cy.checkComponentA11y();
        });

        it('no violations when open', () => {
            getInput().click();
            shouldBeOpen();
            cy.checkComponentA11y();
        });
    });
});
