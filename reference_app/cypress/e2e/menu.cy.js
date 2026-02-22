describe('Menu', () => {
    // ── Helpers ──────────────────────────────────────────────

    function getMenu() {
        return cy.findByRole('menu');
    }

    function getItems() {
        return cy.findAllByRole('menuitem');
    }

    // Focus the menu content element so keyboard navigation starts from it.
    function focusMenu() {
        getMenu().focus();
        getMenu().should('be.focused');
    }

    beforeEach(() => {
        cy.visit('/menu');
        // Menu is always open; wait for it to render.
        getMenu().should('exist');
    });

    // ── 1. Accessibility Semantics ──────────────────────────

    describe('accessibility', () => {
        it('Content has role="menu"', () => {
            getMenu().should('exist');
        });

        it('Content has aria-orientation="vertical"', () => {
            getMenu().should('have.attr', 'aria-orientation', 'vertical');
        });

        it('Items have role="menuitem"', () => {
            getItems().should('have.length.at.least', 1);
        });

        it('Groups have role="group"', () => {
            cy.get('[role="group"]').should('have.length.at.least', 2);
        });

        it('Separators have role="separator"', () => {
            cy.get('[role="separator"]').should('have.length.at.least', 2);
        });

        it('Disabled item has aria-disabled="true"', () => {
            cy.findByText('Carrot').should('have.attr', 'aria-disabled', 'true');
        });
    });

    // ── 2. Data Attributes ──────────────────────────────────

    describe('data attributes', () => {
        it('Content has data-state="open"', () => {
            getMenu().should('have.attr', 'data-state', 'open');
        });

        it('Content has data-side attribute', () => {
            getMenu().should('have.attr', 'data-side');
        });

        it('Content has data-align attribute', () => {
            getMenu().should('have.attr', 'data-align');
        });

        it('Item gets data-highlighted when focused via keyboard', () => {
            focusMenu();
            cy.realPress('ArrowDown');
            cy.findByText('Apple').should('have.attr', 'data-highlighted');
        });

        it('Disabled item has data-disabled attribute', () => {
            cy.findByText('Carrot').should('have.attr', 'data-disabled');
        });
    });

    // ── 3. Keyboard Navigation ──────────────────────────────

    describe('keyboard navigation', () => {
        it('ArrowDown moves focus to first item from content', () => {
            focusMenu();
            cy.realPress('ArrowDown');
            cy.findByText('Apple').should('have.attr', 'data-highlighted');
        });

        it('ArrowDown navigates to next item', () => {
            focusMenu();
            cy.realPress('ArrowDown');
            cy.findByText('Apple').should('have.attr', 'data-highlighted');
            cy.realPress('ArrowDown');
            cy.findByText('Banana').should('have.attr', 'data-highlighted');
        });

        it('ArrowUp navigates to previous item', () => {
            focusMenu();
            cy.realPress('ArrowDown');
            cy.realPress('ArrowDown');
            cy.findByText('Banana').should('have.attr', 'data-highlighted');
            cy.realPress('ArrowUp');
            cy.findByText('Apple').should('have.attr', 'data-highlighted');
        });

        it('ArrowUp from content focuses last item', () => {
            focusMenu();
            cy.realPress('ArrowUp');
            // "More Options..." is the last focusable item (sub trigger)
            cy.findByText('More Options...').should('have.attr', 'data-highlighted');
        });

        it('Home moves focus to first item', () => {
            focusMenu();
            cy.realPress('ArrowDown');
            cy.realPress('ArrowDown');
            cy.realPress('ArrowDown');
            cy.realPress('Home');
            cy.findByText('Apple').should('have.attr', 'data-highlighted');
        });

        it('End moves focus to last item', () => {
            focusMenu();
            cy.realPress('ArrowDown');
            cy.realPress('End');
            cy.findByText('More Options...').should('have.attr', 'data-highlighted');
        });

        it('Enter activates focused item', () => {
            focusMenu();
            cy.realPress('ArrowDown');
            cy.findByText('Apple').should('have.attr', 'data-highlighted');
            cy.realPress('Enter');
            cy.findByTestId('last-action').should('have.text', 'Apple');
        });

        it('Space activates focused item', () => {
            focusMenu();
            cy.realPress('ArrowDown');
            cy.findByText('Apple').should('have.attr', 'data-highlighted');
            cy.realPress('Space');
            cy.findByTestId('last-action').should('have.text', 'Apple');
        });

        it('ArrowDown skips disabled items', () => {
            focusMenu();
            // Navigate to Blueberry (3rd item in Fruits group)
            cy.realPress('ArrowDown');
            cy.realPress('ArrowDown');
            cy.realPress('ArrowDown');
            cy.findByText('Blueberry').should('have.attr', 'data-highlighted');
            // Next is Broccoli (first in Vegetables)
            cy.realPress('ArrowDown');
            cy.findByText('Broccoli').should('have.attr', 'data-highlighted');
            // Next should skip Carrot (disabled) and go to Courgette
            cy.realPress('ArrowDown');
            cy.findByText('Courgette').should('have.attr', 'data-highlighted');
        });

        it('Tab is prevented within menu', () => {
            focusMenu();
            cy.realPress('ArrowDown');
            cy.findByText('Apple').should('have.attr', 'data-highlighted');
            cy.realPress('Tab');
            // Tab should not move focus outside the menu; Apple should still be highlighted
            cy.findByText('Apple').should('have.attr', 'data-highlighted');
        });
    });

    // ── 4. Typeahead ────────────────────────────────────────

    describe('typeahead', () => {
        it('typing a character focuses matching item', () => {
            focusMenu();
            cy.realPress('ArrowDown');
            cy.findByText('Apple').should('have.attr', 'data-highlighted');
            // Type "b" to jump to first "B" item (Banana)
            cy.realPress('b');
            cy.findByText('Banana').should('have.attr', 'data-highlighted');
        });

        it('typing same character cycles through matches', () => {
            focusMenu();
            cy.realPress('ArrowDown');
            // Type "b" to jump to Banana
            cy.realPress('b');
            cy.findByText('Banana').should('have.attr', 'data-highlighted');
            // Type "b" again to cycle to Blueberry
            cy.realPress('b');
            cy.findByText('Blueberry').should('have.attr', 'data-highlighted');
            // Type "b" again to cycle to Broccoli
            cy.realPress('b');
            cy.findByText('Broccoli').should('have.attr', 'data-highlighted');
        });

        it('typing multiple characters matches prefix', () => {
            focusMenu();
            cy.realPress('ArrowDown');
            // Type "bl" quickly to match "Blueberry"
            cy.realPress('b');
            cy.realPress('l');
            cy.findByText('Blueberry').should('have.attr', 'data-highlighted');
        });

        it('typeahead skips disabled items', () => {
            focusMenu();
            cy.realPress('ArrowDown');
            // Type "c" — should match Courgette, not Carrot (disabled)
            cy.realPress('c');
            cy.findByText('Courgette').should('have.attr', 'data-highlighted');
        });

        it('typeahead with explicit textValue finds items by textValue', () => {
            focusMenu();
            cy.realPress('ArrowDown');
            // Type "h" to find "Hearts" (which has textValue="Hearts")
            // The item contains emoji + text, but textValue overrides textContent
            cy.realPress('h');
            cy.findByText(/Hearts/).should('have.attr', 'data-highlighted');
        });

        it('typeahead with explicit textValue cycles through matches', () => {
            focusMenu();
            cy.realPress('ArrowDown');
            // Type "s" to find first "S" match — Small (radio)
            cy.realPress('s');
            cy.findByRole('menuitemradio', {name: 'Small'}).should('have.attr', 'data-highlighted');
            // Type "s" again to cycle to Spades (via textValue)
            cy.realPress('s');
            cy.findByText(/Spades/).should('have.attr', 'data-highlighted');
        });
    });

    // ── 5. Pointer Interaction ──────────────────────────────

    describe('pointer interaction', () => {
        it('hover highlights item', () => {
            cy.findByText('Banana').realHover();
            cy.findByText('Banana').should('have.attr', 'data-highlighted');
        });

        it('click activates item', () => {
            cy.findByText('Apple').click();
            cy.findByTestId('last-action').should('have.text', 'Apple');
        });

        it('hover moves highlight between items', () => {
            cy.findByText('Apple').realHover();
            cy.findByText('Apple').should('have.attr', 'data-highlighted');
            cy.findByText('Banana').realHover();
            cy.findByText('Banana').should('have.attr', 'data-highlighted');
            cy.findByText('Apple').should('not.have.attr', 'data-highlighted');
        });
    });

    // ── 6. Disabled State ───────────────────────────────────

    describe('disabled state', () => {
        it('disabled item has data-disabled attribute', () => {
            cy.findByText('Carrot').should('have.attr', 'data-disabled');
        });

        it('disabled item has aria-disabled="true"', () => {
            cy.findByText('Carrot').should('have.attr', 'aria-disabled', 'true');
        });

        it('keyboard navigation skips disabled items', () => {
            focusMenu();
            // Navigate through vegetables: Broccoli -> (skip Carrot) -> Courgette
            cy.realPress('ArrowDown');
            cy.realPress('ArrowDown');
            cy.realPress('ArrowDown');
            cy.realPress('ArrowDown'); // Broccoli
            cy.findByText('Broccoli').should('have.attr', 'data-highlighted');
            cy.realPress('ArrowDown'); // Should skip Carrot -> Courgette
            cy.findByText('Courgette').should('have.attr', 'data-highlighted');
        });
    });

    // ── 7. Checkbox Items ───────────────────────────────────

    describe('checkbox items', () => {
        it('checkbox items have role="menuitemcheckbox"', () => {
            cy.findAllByRole('menuitemcheckbox').should('have.length', 2);
        });

        it('unchecked item has aria-checked="false" and data-state="unchecked"', () => {
            cy.findByRole('menuitemcheckbox', {name: 'Bold'}).should('have.attr', 'aria-checked', 'false');
            cy.findByRole('menuitemcheckbox', {name: 'Bold'}).should('have.attr', 'data-state', 'unchecked');
        });

        it('checked item has aria-checked="true" and data-state="checked"', () => {
            cy.findByRole('menuitemcheckbox', {name: /^Italic/}).should('have.attr', 'aria-checked', 'true');
            cy.findByRole('menuitemcheckbox', {name: /^Italic/}).should('have.attr', 'data-state', 'checked');
        });

        it('click toggles checkbox item', () => {
            // Bold starts unchecked
            cy.findByRole('menuitemcheckbox', {name: 'Bold'}).should('have.attr', 'aria-checked', 'false');
            cy.findByRole('menuitemcheckbox', {name: 'Bold'}).click();
            cy.findByTestId('checkbox-state').should('contain.text', 'Bold');
        });

        it('click unchecks a checked checkbox item', () => {
            // Italic starts checked
            cy.findByRole('menuitemcheckbox', {name: /^Italic/}).should('have.attr', 'aria-checked', 'true');
            cy.findByRole('menuitemcheckbox', {name: /^Italic/}).click();
            cy.findByTestId('checkbox-state').should('not.contain.text', 'Italic');
        });

        it('ItemIndicator visible when checked, hidden when unchecked', () => {
            // Italic is checked — indicator should be visible
            cy.findByRole('menuitemcheckbox', {name: /^Italic/}).find('span[data-state="checked"]').should('exist');
            // Bold is unchecked — indicator span should not exist (Presence hides it)
            cy.findByRole('menuitemcheckbox', {name: 'Bold'}).find('span[data-state]').should('not.exist');
        });

        it('Enter toggles checkbox item', () => {
            focusMenu();
            // Navigate to Bold checkbox item
            cy.findByRole('menuitemcheckbox', {name: 'Bold'}).realHover();
            cy.findByRole('menuitemcheckbox', {name: 'Bold'}).should('have.attr', 'data-highlighted');
            cy.realPress('Enter');
            cy.findByTestId('checkbox-state').should('contain.text', 'Bold');
        });

        it('Space toggles checkbox item', () => {
            focusMenu();
            cy.findByRole('menuitemcheckbox', {name: 'Bold'}).realHover();
            cy.findByRole('menuitemcheckbox', {name: 'Bold'}).should('have.attr', 'data-highlighted');
            cy.realPress('Space');
            cy.findByTestId('checkbox-state').should('contain.text', 'Bold');
        });

        it('keyboard navigation includes checkbox items', () => {
            focusMenu();
            // Navigate past Redo to reach Bold
            cy.realPress('ArrowDown'); // Apple
            cy.realPress('End'); // last item
            // Now go up to find checkbox items
            cy.realPress('ArrowUp'); // Clubs (suit)
            cy.realPress('ArrowUp'); // Diamonds
            cy.realPress('ArrowUp'); // Spades
            cy.realPress('ArrowUp'); // Hearts
            cy.realPress('ArrowUp'); // Large (radio)
            cy.realPress('ArrowUp'); // Medium (radio)
            cy.realPress('ArrowUp'); // Small (radio)
            cy.realPress('ArrowUp'); // Italic (checkbox)
            cy.findByRole('menuitemcheckbox', {name: /^Italic/}).should('have.attr', 'data-highlighted');
            cy.realPress('ArrowUp'); // Bold (checkbox)
            cy.findByRole('menuitemcheckbox', {name: 'Bold'}).should('have.attr', 'data-highlighted');
        });
    });

    // ── 8. Radio Items ──────────────────────────────────────

    describe('radio items', () => {
        it('radio items have role="menuitemradio"', () => {
            cy.findAllByRole('menuitemradio').should('have.length', 3);
        });

        it('selected radio item has aria-checked="true" and data-state="checked"', () => {
            // Medium is selected by default
            cy.findByRole('menuitemradio', {name: /^Medium/}).should('have.attr', 'aria-checked', 'true');
            cy.findByRole('menuitemradio', {name: /^Medium/}).should('have.attr', 'data-state', 'checked');
        });

        it('unselected radio items have aria-checked="false" and data-state="unchecked"', () => {
            cy.findByRole('menuitemradio', {name: 'Small'}).should('have.attr', 'aria-checked', 'false');
            cy.findByRole('menuitemradio', {name: 'Small'}).should('have.attr', 'data-state', 'unchecked');
            cy.findByRole('menuitemradio', {name: 'Large'}).should('have.attr', 'aria-checked', 'false');
            cy.findByRole('menuitemradio', {name: 'Large'}).should('have.attr', 'data-state', 'unchecked');
        });

        it('clicking selects radio item and deselects others', () => {
            cy.findByRole('menuitemradio', {name: 'Small'}).click();
            cy.findByTestId('radio-value').should('have.text', 'Small');
        });

        it('ItemIndicator shows for selected radio only', () => {
            // Medium is selected — indicator should be visible
            cy.findByRole('menuitemradio', {name: /^Medium/}).find('span[data-state="checked"]').should('exist');
            // Small is not selected — indicator should not exist
            cy.findByRole('menuitemradio', {name: 'Small'}).find('span[data-state]').should('not.exist');
        });

        it('Enter selects radio item', () => {
            focusMenu();
            cy.findByRole('menuitemradio', {name: 'Large'}).realHover();
            cy.findByRole('menuitemradio', {name: 'Large'}).should('have.attr', 'data-highlighted');
            cy.realPress('Enter');
            cy.findByTestId('radio-value').should('have.text', 'Large');
        });

        it('Space selects radio item', () => {
            focusMenu();
            cy.findByRole('menuitemradio', {name: 'Small'}).realHover();
            cy.findByRole('menuitemradio', {name: 'Small'}).should('have.attr', 'data-highlighted');
            cy.realPress('Space');
            cy.findByTestId('radio-value').should('have.text', 'Small');
        });
    });

    // ── 9. Submenus ─────────────────────────────────────────

    describe('submenus', () => {
        it('sub trigger has aria-haspopup="menu"', () => {
            cy.findByText('More Options...').should('have.attr', 'aria-haspopup', 'menu');
        });

        it('sub trigger has aria-expanded="false" initially', () => {
            cy.findByText('More Options...').should('have.attr', 'aria-expanded', 'false');
        });

        it('sub trigger has data-state="closed" initially', () => {
            cy.findByText('More Options...').should('have.attr', 'data-state', 'closed');
        });

        it('hover opens submenu', () => {
            cy.findByText('More Options...').realHover();
            cy.findByText('More Options...').should('have.attr', 'data-highlighted');
            // Wait for 100ms open timer
            cy.findByText('More Options...').should('have.attr', 'aria-expanded', 'true');
            cy.findByText('More Options...').should('have.attr', 'data-state', 'open');
            cy.findByText('Option A').should('be.visible');
        });

        it('submenu items are clickable', () => {
            cy.findByText('More Options...').realHover();
            cy.findByText('Option A').should('be.visible');
            cy.findByText('Option A').click();
            cy.findByTestId('last-action').should('have.text', 'Option A');
        });

        it('ArrowRight opens submenu and focuses first item (LTR)', () => {
            focusMenu();
            cy.realPress('End'); // Navigate to More Options...
            cy.findByText('More Options...').should('have.attr', 'data-highlighted');
            cy.realPress('ArrowRight');
            cy.findByText('More Options...').should('have.attr', 'aria-expanded', 'true');
            cy.findByText('Option A').should('be.visible');
            // First item should be auto-focused
            cy.findByText('Option A').should('have.attr', 'data-highlighted');
        });

        it('ArrowLeft closes submenu (LTR)', () => {
            // Open submenu via keyboard
            focusMenu();
            cy.realPress('End'); // Navigate to More Options...
            cy.findByText('More Options...').should('have.attr', 'data-highlighted');
            cy.realPress('ArrowRight');
            cy.findByText('More Options...').should('have.attr', 'aria-expanded', 'true');
            cy.findByText('Option A').should('be.visible');
            cy.realPress('ArrowLeft');
            // Submenu should close
            cy.findByText('Option A').should('not.exist');
            // Focus should return to trigger
            cy.findByText('More Options...').should('have.attr', 'data-highlighted');
        });

        it('keyboard navigation works within submenu', () => {
            focusMenu();
            cy.realPress('End'); // Navigate to More Options...
            cy.findByText('More Options...').should('have.attr', 'data-highlighted');
            cy.realPress('ArrowRight'); // Open submenu
            cy.findByText('More Options...').should('have.attr', 'aria-expanded', 'true');
            // First item should be auto-focused
            cy.findByText('Option A').should('have.attr', 'data-highlighted');
            // ArrowDown should move to Option B
            cy.realPress('ArrowDown');
            cy.findByText('Option B').should('have.attr', 'data-highlighted');
        });

        it('hover subtrigger then hover submenu item — submenu stays open (grace area)', () => {
            // Hover the sub trigger to open the submenu
            cy.findByText('More Options...').realHover();
            // Wait for the submenu to open (100ms timer + rendering)
            cy.findByText('More Options...').should('have.attr', 'aria-expanded', 'true');
            cy.findByText('Option A').should('be.visible');
            // Move mouse to a submenu item — submenu should stay open
            cy.findByText('Option A').realHover();
            cy.findByText('Option A').should('be.visible');
            cy.findByText('Option A').should('have.attr', 'data-highlighted');
            // The submenu should still be expanded
            cy.findByText('More Options...').should('have.attr', 'aria-expanded', 'true');
        });

        it('hover subtrigger then hover different parent item — submenu closes', () => {
            // Hover the sub trigger to open the submenu
            cy.findByText('More Options...').realHover();
            cy.findByText('More Options...').should('have.attr', 'aria-expanded', 'true');
            cy.findByText('Option A').should('be.visible');
            // Move mouse to a parent menu item (away from submenu)
            cy.findByText('Apple').realHover();
            // The submenu should close
            cy.findByText('More Options...').should('have.attr', 'aria-expanded', 'false');
            cy.findByText('Option A').should('not.exist');
        });

        it('submenu content has aria-labelledby linked to trigger', () => {
            cy.findByText('More Options...').realHover();
            cy.findByText('More Options...').should('have.attr', 'aria-expanded', 'true');
            // Get the trigger ID and verify content's aria-labelledby matches
            cy.findByText('More Options...').invoke('attr', 'id').then((triggerId) => {
                cy.findByText('Option A').closest('[role="menu"]')
                    .should('have.attr', 'aria-labelledby', triggerId);
            });
        });

        it('submenu content has its own menu role', () => {
            cy.findByText('More Options...').realHover();
            cy.findByText('More Options...').should('have.attr', 'aria-expanded', 'true');
            // The submenu should have its own menu role
            cy.findByText('Option A').closest('[role="menu"]')
                .should('have.attr', 'role', 'menu');
        });
    });

    // ── 10. Animated Submenus ────────────────────────────────

    describe('animated submenus', () => {
        beforeEach(() => {
            // Enable animation
            cy.findByTestId('animated-toggle').check();
        });

        it('ArrowRight opens animated submenu and focuses first item', () => {
            focusMenu();
            cy.realPress('End');
            cy.findByText('More Options...').should('have.attr', 'data-highlighted');
            cy.realPress('ArrowRight');
            cy.findByText('More Options...').should('have.attr', 'aria-expanded', 'true');
            cy.findByText('Option A').should('be.visible');
            cy.findByText('Option A').should('have.attr', 'data-highlighted');
        });

        it('ArrowLeft closes animated submenu and returns focus to trigger', () => {
            focusMenu();
            cy.realPress('End');
            cy.realPress('ArrowRight');
            cy.findByText('Option A').should('have.attr', 'data-highlighted');
            cy.realPress('ArrowLeft');
            cy.findByText('More Options...').should('have.attr', 'data-highlighted');
            cy.findByText('More Options...').should('have.attr', 'aria-expanded', 'false');
        });

        it('rapid open-close-open cycle: first item is highlighted after reopen', () => {
            focusMenu();
            cy.realPress('End');
            cy.findByText('More Options...').should('have.attr', 'data-highlighted');

            // Open submenu
            cy.realPress('ArrowRight');
            cy.findByText('Option A').should('have.attr', 'data-highlighted');

            // Close submenu immediately
            cy.realPress('ArrowLeft');
            cy.findByText('More Options...').should('have.attr', 'data-highlighted');

            // Reopen submenu — first item must be highlighted
            cy.realPress('ArrowRight');
            cy.findByText('Option A').should('be.visible');
            cy.findByText('Option A').should('have.attr', 'data-highlighted');
        });

        it('multiple rapid open-close cycles: first item is always highlighted on reopen', () => {
            focusMenu();
            cy.realPress('End');
            cy.findByText('More Options...').should('have.attr', 'data-highlighted');

            // Cycle 1
            cy.realPress('ArrowRight');
            cy.findByText('Option A').should('have.attr', 'data-highlighted');
            cy.realPress('ArrowLeft');
            cy.findByText('More Options...').should('have.attr', 'data-highlighted');

            // Cycle 2
            cy.realPress('ArrowRight');
            cy.findByText('Option A').should('have.attr', 'data-highlighted');
            cy.realPress('ArrowLeft');
            cy.findByText('More Options...').should('have.attr', 'data-highlighted');

            // Cycle 3
            cy.realPress('ArrowRight');
            cy.findByText('Option A').should('have.attr', 'data-highlighted');
            cy.realPress('ArrowLeft');
            cy.findByText('More Options...').should('have.attr', 'data-highlighted');

            // Final open — must still work
            cy.realPress('ArrowRight');
            cy.findByText('Option A').should('be.visible');
            cy.findByText('Option A').should('have.attr', 'data-highlighted');
        });

        it('open animated submenu, navigate within, close, reopen — submenu is interactive', () => {
            focusMenu();
            cy.realPress('End');
            cy.findByText('More Options...').should('have.attr', 'data-highlighted');

            // Open and navigate to second item
            cy.realPress('ArrowRight');
            cy.findByText('Option A').should('have.attr', 'data-highlighted');
            cy.realPress('ArrowDown');
            cy.findByText('Option B').should('have.attr', 'data-highlighted');

            // Close
            cy.realPress('ArrowLeft');
            cy.findByText('More Options...').should('have.attr', 'data-highlighted');

            // Reopen — an item should be highlighted (focus is restored)
            cy.realPress('ArrowRight');
            cy.findByText('Option A').should('be.visible');
            // Navigate down to verify the submenu is interactive
            cy.realPress('ArrowDown');
            cy.findByText('Option B').should('have.attr', 'data-highlighted');
        });
    });
});
