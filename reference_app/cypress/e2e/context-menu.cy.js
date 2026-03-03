describe('Context Menu', () => {
    // ── Helpers ──────────────────────────────────────────────

    function shouldBeOpen() {
        cy.findByRole('menu').should('exist');
    }

    function shouldBeClosed() {
        cy.findByRole('menu').should('not.exist');
    }

    function openMenu() {
        cy.findByTestId('context-trigger').rightclick();
        shouldBeOpen();
    }

    beforeEach(() => {
        cy.visit('/context-menu');
    });

    // ── 1. Accessibility Semantics ──────────────────────────

    describe('accessibility', () => {
        it('Content has role="menu"', () => {
            openMenu();
            cy.findByRole('menu').should('exist');
        });

        it('Items have role="menuitem"', () => {
            openMenu();
            cy.findAllByRole('menuitem').should('have.length.at.least', 1);
        });

        it('CheckboxItem has role="menuitemcheckbox"', () => {
            openMenu();
            cy.findByRole('menuitemcheckbox').should('exist');
        });

        it('RadioItems have role="menuitemradio"', () => {
            openMenu();
            cy.findAllByRole('menuitemradio').should('have.length', 2);
        });
    });

    // ── 2. Data Attributes ──────────────────────────────────

    describe('data attributes', () => {
        it('Trigger data-state reflects open state', () => {
            cy.findByTestId('context-trigger').should('have.attr', 'data-state', 'closed');
            openMenu();
            cy.findByTestId('context-trigger').should('have.attr', 'data-state', 'open');
        });

        it('Content has data-state="open"', () => {
            openMenu();
            cy.findByRole('menu').should('have.attr', 'data-state', 'open');
        });

        it('Content has data-side attribute', () => {
            openMenu();
            cy.findByRole('menu').should('have.attr', 'data-side');
        });

        it('Content has data-align attribute', () => {
            openMenu();
            cy.findByRole('menu').should('have.attr', 'data-align');
        });

        it('Item has data-highlighted when hovered', () => {
            openMenu();
            cy.findByText('Item 1').realHover();
            cy.findByText('Item 1').should('have.attr', 'data-highlighted');
        });

        it('CheckboxItem data-state toggles checked/unchecked', () => {
            openMenu();
            cy.findByRole('menuitemcheckbox').should('have.attr', 'data-state', 'unchecked');
            cy.findByRole('menuitemcheckbox').click();
            openMenu();
            cy.findByRole('menuitemcheckbox').should('have.attr', 'data-state', 'checked');
        });

        it('RadioItem data-state reflects selection', () => {
            openMenu();
            cy.findAllByRole('menuitemradio').eq(0).should('have.attr', 'data-state', 'checked');
            cy.findAllByRole('menuitemradio').eq(1).should('have.attr', 'data-state', 'unchecked');
            cy.findAllByRole('menuitemradio').eq(1).click();
            openMenu();
            cy.findAllByRole('menuitemradio').eq(0).should('have.attr', 'data-state', 'unchecked');
            cy.findAllByRole('menuitemradio').eq(1).should('have.attr', 'data-state', 'checked');
        });
    });

    // ── 3. Keyboard Navigation ──────────────────────────────

    describe('keyboard navigation', () => {
        it('ArrowDown on freshly opened menu focuses first item', () => {
            openMenu();
            cy.realPress('ArrowDown');
            cy.findByText('Item 1').should('have.attr', 'data-highlighted');
        });

        it('ArrowUp on freshly opened menu focuses last item', () => {
            openMenu();
            cy.realPress('ArrowUp');
            cy.get('[role="menu"] > [role="menuitem"]').last().should('have.attr', 'data-highlighted');
        });

        it('ArrowDown navigates to next item', () => {
            openMenu();
            cy.findByText('Item 1').realHover();
            cy.findByText('Item 1').should('have.attr', 'data-highlighted');
            cy.realPress('ArrowDown');
            cy.findByText('Item 2').should('have.attr', 'data-highlighted');
        });

        it('ArrowUp navigates to previous item', () => {
            openMenu();
            cy.findByText('Item 2').realHover();
            cy.findByText('Item 2').should('have.attr', 'data-highlighted');
            cy.realPress('ArrowUp');
            cy.findByText('Item 1').should('have.attr', 'data-highlighted');
        });

        it('Enter activates focused item', () => {
            openMenu();
            cy.findByText('Item 1').realHover();
            cy.findByText('Item 1').should('have.attr', 'data-highlighted');
            cy.realPress('Enter');
            shouldBeClosed();
            cy.findByTestId('last-action').should('have.text', 'Item 1');
        });

        it('Space activates focused item', () => {
            openMenu();
            cy.findByText('Item 1').realHover();
            cy.findByText('Item 1').should('have.attr', 'data-highlighted');
            cy.realPress('Space');
            shouldBeClosed();
            cy.findByTestId('last-action').should('have.text', 'Item 1');
        });

        it('Escape closes menu', () => {
            openMenu();
            cy.realPress('Escape');
            shouldBeClosed();
        });

        it('Home moves focus to first item', () => {
            openMenu();
            cy.findByText('Item 3').realHover();
            cy.findByText('Item 3').should('have.attr', 'data-highlighted');
            cy.realPress('Home');
            cy.findByText('Item 1').should('have.attr', 'data-highlighted');
        });

        it('End moves focus to last menuitem', () => {
            openMenu();
            cy.findByText('Item 1').realHover();
            cy.findByText('Item 1').should('have.attr', 'data-highlighted');
            cy.realPress('End');
            cy.get('[role="menu"] > [role="menuitem"]').last().should('have.attr', 'data-highlighted');
        });

        it('ArrowDown skips disabled items', () => {
            cy.findByLabelText('disabled').click();
            openMenu();
            cy.findByText('Item 1').realHover();
            cy.findByText('Item 1').should('have.attr', 'data-highlighted');
            cy.realPress('ArrowDown');
            // Should skip Item 2 (disabled) and go to Item 3
            cy.findByText('Item 3').should('have.attr', 'data-highlighted');
        });
    });

    // ── 4. Pointer Interaction ──────────────────────────────

    describe('pointer interaction', () => {
        it('right-click opens menu', () => {
            cy.findByTestId('context-trigger').rightclick();
            shouldBeOpen();
        });

        it('click item activates and closes', () => {
            openMenu();
            cy.findByText('Item 1').click();
            shouldBeClosed();
            cy.findByTestId('last-action').should('have.text', 'Item 1');
        });

        it('click outside closes menu', () => {
            openMenu();
            cy.get('body').realClick({position: {x: 1, y: 1}});
            shouldBeClosed();
        });

        it('hover highlights items', () => {
            openMenu();
            cy.findByText('Item 3').realHover();
            cy.findByText('Item 3').should('have.attr', 'data-highlighted');
        });
    });

    // ── 5. CheckboxItem ─────────────────────────────────────

    describe('checkbox item', () => {
        it('click toggles checked state', () => {
            cy.findByTestId('checkbox-state').should('have.text', 'false');
            openMenu();
            cy.findByRole('menuitemcheckbox').click();
            cy.findByTestId('checkbox-state').should('have.text', 'true');
            openMenu();
            cy.findByRole('menuitemcheckbox').click();
            cy.findByTestId('checkbox-state').should('have.text', 'false');
        });
    });

    // ── 6. RadioGroup ───────────────────────────────────────

    describe('radio group', () => {
        it('click selects radio item', () => {
            cy.findByTestId('radio-value').should('have.text', 'radio1');
            openMenu();
            cy.findAllByRole('menuitemradio').eq(1).click();
            cy.findByTestId('radio-value').should('have.text', 'radio2');
        });

        it('only one radio can be checked at a time', () => {
            openMenu();
            cy.findAllByRole('menuitemradio').eq(0).should('have.attr', 'data-state', 'checked');
            cy.findAllByRole('menuitemradio').eq(1).should('have.attr', 'data-state', 'unchecked');
            cy.findAllByRole('menuitemradio').eq(1).click();
            openMenu();
            cy.findAllByRole('menuitemradio').eq(0).should('have.attr', 'data-state', 'unchecked');
            cy.findAllByRole('menuitemradio').eq(1).should('have.attr', 'data-state', 'checked');
        });
    });

    // ── 7. Submenu ──────────────────────────────────────────

    describe('submenu', () => {
        it('hover on SubTrigger opens submenu', () => {
            openMenu();
            cy.findByText('Submenu →').realHover();
            cy.findByText('Sub item 1').should('exist');
        });

        it('ArrowRight opens submenu', () => {
            openMenu();
            cy.findByText('Submenu →').realHover();
            cy.findByText('Submenu →').should('have.attr', 'data-highlighted');
            cy.realPress('ArrowRight');
            cy.findByText('Sub item 1').should('exist');
        });

        it('ArrowLeft closes submenu when focus is inside', () => {
            openMenu();
            cy.findByText('Submenu →').realHover();
            cy.findByText('Sub item 1').should('exist');
            cy.realPress('ArrowRight');
            cy.findByText('Sub item 1').should('have.attr', 'data-highlighted');
            cy.realPress('ArrowLeft');
            cy.findByText('Sub item 1').should('not.exist');
            shouldBeOpen();
        });

        it('clicking sub item activates and closes entire menu', () => {
            openMenu();
            cy.findByText('Submenu →').realHover();
            cy.findByText('Sub item 1').should('exist');
            cy.findByText('Sub item 1').click();
            shouldBeClosed();
            cy.findByTestId('last-action').should('have.text', 'Sub item 1');
        });
    });

    // ── 8. Disabled State ───────────────────────────────────

    describe('disabled state', () => {
        beforeEach(() => {
            cy.findByLabelText('disabled').click();
        });

        it('disabled item has data-disabled attribute', () => {
            openMenu();
            cy.findByText('Item 2').should('have.attr', 'data-disabled');
        });
    });
});
