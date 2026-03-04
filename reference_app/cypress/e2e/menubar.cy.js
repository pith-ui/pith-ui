describe('Menubar', () => {
    // ── Helpers ──────────────────────────────────────────────

    function getMenubar() {
        return cy.findByRole('menubar');
    }

    function getTrigger(name) {
        return cy.findByRole('menuitem', {name});
    }

    function shouldHaveMenuOpen() {
        cy.findByRole('menu').should('exist');
    }

    function shouldHaveMenuClosed() {
        cy.findByRole('menu').should('not.exist');
    }

    function openMenu(name) {
        getTrigger(name).click();
        shouldHaveMenuOpen();
    }

    beforeEach(() => {
        cy.visit('/menubar');
    });

    // ── 1. Accessibility Semantics ──────────────────────────

    describe('accessibility', () => {
        it('Root has role="menubar"', () => {
            getMenubar().should('exist');
        });

        it('Triggers have role="menuitem"', () => {
            getTrigger('File').should('exist');
            getTrigger('Edit').should('exist');
            getTrigger('View').should('exist');
        });

        it('Triggers have aria-haspopup="menu"', () => {
            getTrigger('File').should('have.attr', 'aria-haspopup', 'menu');
            getTrigger('Edit').should('have.attr', 'aria-haspopup', 'menu');
            getTrigger('View').should('have.attr', 'aria-haspopup', 'menu');
        });

        it('Trigger aria-expanded reflects open state', () => {
            getTrigger('File').should('have.attr', 'aria-expanded', 'false');
            openMenu('File');
            getTrigger('File').should('have.attr', 'aria-expanded', 'true');
        });

        it('Trigger aria-controls references Content id', () => {
            openMenu('File');
            getTrigger('File')
                .invoke('attr', 'aria-controls')
                .then((controlsId) => {
                    cy.findByRole('menu').should('have.attr', 'id', controlsId);
                });
        });

        it('Content has role="menu"', () => {
            openMenu('File');
            cy.findByRole('menu').should('exist');
        });

        it('Content has aria-labelledby pointing to trigger id', () => {
            openMenu('File');
            getTrigger('File')
                .invoke('attr', 'id')
                .then((triggerId) => {
                    cy.findByRole('menu').should('have.attr', 'aria-labelledby', triggerId);
                });
        });

        it('Items have role="menuitem"', () => {
            openMenu('File');
            // Items inside menu content (not the triggers in menubar)
            cy.findByRole('menu').findAllByRole('menuitem').should('have.length.at.least', 1);
        });

        it('CheckboxItems have role="menuitemcheckbox" with aria-checked', () => {
            openMenu('View');
            cy.findAllByRole('menuitemcheckbox').should('have.length', 2);
            cy.findAllByRole('menuitemcheckbox').first().should('have.attr', 'aria-checked');
        });

        it('RadioItems have role="menuitemradio" with aria-checked', () => {
            openMenu('View');
            cy.findAllByRole('menuitemradio').should('have.length', 2);
            cy.findAllByRole('menuitemradio').first().should('have.attr', 'aria-checked');
        });
    });

    // ── 2. Data Attributes ──────────────────────────────────

    describe('data attributes', () => {
        it('Trigger data-state closed → open → closed', () => {
            getTrigger('File').should('have.attr', 'data-state', 'closed');
            openMenu('File');
            getTrigger('File').should('have.attr', 'data-state', 'open');
            cy.realPress('Escape');
            shouldHaveMenuClosed();
            getTrigger('File').should('have.attr', 'data-state', 'closed');
        });

        it('Content has data-state="open"', () => {
            openMenu('File');
            cy.findByRole('menu').should('have.attr', 'data-state', 'open');
        });

        it('Content has data-side attribute', () => {
            openMenu('File');
            cy.findByRole('menu').should('have.attr', 'data-side');
        });

        it('Content has data-align attribute', () => {
            openMenu('File');
            cy.findByRole('menu').should('have.attr', 'data-align');
        });

        it('Item has data-highlighted when focused via keyboard', () => {
            getTrigger('File').focus();
            cy.realPress('ArrowDown');
            shouldHaveMenuOpen();
            cy.findByRole('menu').findAllByRole('menuitem').first().should('have.attr', 'data-highlighted');
        });

        it('CheckboxItem data-state toggles checked/unchecked', () => {
            openMenu('View');
            // "Always Show Bookmarks Bar" starts checked (bookmarks=true)
            cy.findByRole('menuitemcheckbox', {name: /Bookmarks/}).should(
                'have.attr',
                'data-state',
                'checked'
            );
            // "Always Show Full URLs" starts unchecked (urls=false)
            cy.findByRole('menuitemcheckbox', {name: /URLs/}).should(
                'have.attr',
                'data-state',
                'unchecked'
            );
            // Toggle URLs checkbox
            cy.findByRole('menuitemcheckbox', {name: /URLs/}).click();
            // Menu closes after click; reopen to verify
            openMenu('View');
            cy.findByRole('menuitemcheckbox', {name: /URLs/}).should('have.attr', 'data-state', 'checked');
        });

        it('RadioItem data-state reflects selection', () => {
            openMenu('View');
            // "Normal" is default selected — use regex since indicator text changes accessible name
            cy.findByRole('menuitemradio', {name: /Normal/}).should('have.attr', 'data-state', 'checked');
            cy.findByRole('menuitemradio', {name: /Compact/}).should('have.attr', 'data-state', 'unchecked');
            // Select Compact
            cy.findByRole('menuitemradio', {name: /Compact/}).click();
            openMenu('View');
            cy.findByRole('menuitemradio', {name: /Compact/}).should('have.attr', 'data-state', 'checked');
            cy.findByRole('menuitemradio', {name: /Normal/}).should('have.attr', 'data-state', 'unchecked');
        });
    });

    // ── 3. Keyboard Navigation ──────────────────────────────

    describe('keyboard navigation', () => {
        it('Space opens menu from trigger', () => {
            getTrigger('File').focus();
            cy.realPress('Space');
            shouldHaveMenuOpen();
        });

        it('Enter opens menu from trigger', () => {
            getTrigger('File').focus();
            cy.realPress('Enter');
            shouldHaveMenuOpen();
        });

        it('ArrowDown opens menu from trigger', () => {
            getTrigger('File').focus();
            cy.realPress('ArrowDown');
            shouldHaveMenuOpen();
        });

        it('ArrowDown navigates to next item in menu', () => {
            getTrigger('File').focus();
            cy.realPress('ArrowDown');
            shouldHaveMenuOpen();
            // First item is highlighted
            cy.findByRole('menu').findAllByRole('menuitem').first().should('have.attr', 'data-highlighted');
            cy.realPress('ArrowDown');
            cy.findByRole('menu').findAllByRole('menuitem').eq(1).should('have.attr', 'data-highlighted');
        });

        it('ArrowUp navigates to previous item in menu', () => {
            getTrigger('File').focus();
            cy.realPress('ArrowDown');
            shouldHaveMenuOpen();
            cy.realPress('ArrowDown');
            cy.realPress('ArrowUp');
            cy.findByRole('menu').findAllByRole('menuitem').first().should('have.attr', 'data-highlighted');
        });

        it('Enter activates focused item', () => {
            getTrigger('File').focus();
            cy.realPress('ArrowDown');
            shouldHaveMenuOpen();
            cy.realPress('Enter');
            shouldHaveMenuClosed();
            cy.findByTestId('last-action').should('have.text', 'New Tab');
        });

        it('Space activates focused item', () => {
            getTrigger('File').focus();
            cy.realPress('ArrowDown');
            shouldHaveMenuOpen();
            cy.realPress('Space');
            shouldHaveMenuClosed();
            cy.findByTestId('last-action').should('have.text', 'New Tab');
        });

        it('Escape closes menu and restores focus to trigger', () => {
            openMenu('File');
            cy.realPress('Escape');
            shouldHaveMenuClosed();
            getTrigger('File').should('be.focused');
        });

        it('Home moves focus to first item', () => {
            getTrigger('File').focus();
            cy.realPress('ArrowDown');
            shouldHaveMenuOpen();
            cy.realPress('ArrowDown');
            cy.realPress('ArrowDown');
            cy.realPress('Home');
            cy.findByRole('menu').findAllByRole('menuitem').first().should('have.attr', 'data-highlighted');
        });

        it('End moves focus to last item', () => {
            getTrigger('File').focus();
            cy.realPress('ArrowDown');
            shouldHaveMenuOpen();
            cy.realPress('End');
            cy.findByRole('menu').findAllByRole('menuitem').last().should('have.attr', 'data-highlighted');
        });

        it('ArrowDown skips disabled items', () => {
            cy.findByLabelText('disabled').click();
            getTrigger('Edit').focus();
            cy.realPress('ArrowDown');
            shouldHaveMenuOpen();
            // First highlighted is "Undo" (disabled) — should skip to "Redo"
            cy.focused().should('have.text', 'Redo');
        });
    });

    // ── 4. Pointer Interaction ──────────────────────────────

    describe('pointer interaction', () => {
        it('click trigger opens menu', () => {
            getTrigger('File').click();
            shouldHaveMenuOpen();
        });

        it('click item activates and closes', () => {
            openMenu('File');
            cy.findByText('New Tab').click();
            shouldHaveMenuClosed();
            cy.findByTestId('last-action').should('have.text', 'New Tab');
        });

        it('click outside closes menu', () => {
            openMenu('File');
            cy.get('body').realClick({position: {x: 1, y: 1}});
            shouldHaveMenuClosed();
        });

        it('hover highlights items', () => {
            openMenu('File');
            cy.findByText('New Window').realHover();
            cy.findByText('New Window').should('have.attr', 'data-highlighted');
        });
    });

    // ── 5. Focus Management ─────────────────────────────────

    describe('focus management', () => {
        it('first item is highlighted after keyboard open', () => {
            getTrigger('File').focus();
            cy.realPress('ArrowDown');
            shouldHaveMenuOpen();
            cy.findByRole('menu').findAllByRole('menuitem').first().should('have.attr', 'data-highlighted');
        });

        it('focus is restored to trigger on Escape close', () => {
            openMenu('File');
            cy.realPress('Escape');
            shouldHaveMenuClosed();
            getTrigger('File').should('be.focused');
        });

        it('roving tabindex: ArrowRight/Left moves between triggers', () => {
            getTrigger('File').focus();
            cy.realPress('ArrowRight');
            getTrigger('Edit').should('be.focused');
            cy.realPress('ArrowRight');
            getTrigger('View').should('be.focused');
            cy.realPress('ArrowLeft');
            getTrigger('Edit').should('be.focused');
        });
    });

    // ── 6. Inter-menu Navigation ────────────────────────────

    describe('inter-menu navigation', () => {
        it('ArrowRight from content opens next menu (File → Edit)', () => {
            openMenu('File');
            // Press ArrowRight from within File content to switch to Edit
            cy.realPress('ArrowRight');
            // Edit menu should now be open
            getTrigger('Edit').should('have.attr', 'data-state', 'open');
            cy.findByRole('menu').should('exist');
        });

        it('ArrowLeft from content opens previous menu (Edit → File)', () => {
            openMenu('Edit');
            cy.realPress('ArrowLeft');
            getTrigger('File').should('have.attr', 'data-state', 'open');
            cy.findByRole('menu').should('exist');
        });

        it('hover trigger opens adjacent menu when one is already open', () => {
            openMenu('File');
            getTrigger('Edit').realHover();
            getTrigger('Edit').should('have.attr', 'data-state', 'open');
            cy.findByRole('menu').should('exist');
        });

        it('click trigger while another is open switches to that menu', () => {
            openMenu('File');
            // Use realClick to ensure pointerenter fires before pointerdown
            // (pointerenter triggers the hover-switch behavior)
            getTrigger('View').realClick();
            getTrigger('View').should('have.attr', 'data-state', 'open');
            getTrigger('File').should('have.attr', 'data-state', 'closed');
        });

        it('ArrowRight wraps from last to first menu', () => {
            openMenu('View');
            cy.realPress('ArrowRight');
            getTrigger('File').should('have.attr', 'data-state', 'open');
        });

        it('ArrowLeft wraps from first to last menu', () => {
            openMenu('File');
            cy.realPress('ArrowLeft');
            getTrigger('View').should('have.attr', 'data-state', 'open');
        });
    });

    // ── 7. Checkbox Items ───────────────────────────────────

    describe('checkbox items', () => {
        it('click toggles checked state (readout updates)', () => {
            cy.findByTestId('checkbox-bookmarks').should('have.text', 'true');
            openMenu('View');
            cy.findByRole('menuitemcheckbox', {name: /Bookmarks/}).click();
            cy.findByTestId('checkbox-bookmarks').should('have.text', 'false');
            openMenu('View');
            cy.findByRole('menuitemcheckbox', {name: /Bookmarks/}).click();
            cy.findByTestId('checkbox-bookmarks').should('have.text', 'true');
        });

        it('data-state reflects checked/unchecked', () => {
            cy.findByTestId('checkbox-urls').should('have.text', 'false');
            openMenu('View');
            cy.findByRole('menuitemcheckbox', {name: /URLs/}).should('have.attr', 'data-state', 'unchecked');
            cy.findByRole('menuitemcheckbox', {name: /URLs/}).click();
            cy.findByTestId('checkbox-urls').should('have.text', 'true');
            openMenu('View');
            cy.findByRole('menuitemcheckbox', {name: /URLs/}).should('have.attr', 'data-state', 'checked');
        });
    });

    // ── 8. Radio Group ──────────────────────────────────────

    describe('radio group', () => {
        it('click selects radio item (readout updates)', () => {
            cy.findByTestId('radio-size').should('have.text', 'normal');
            openMenu('View');
            // Use regex since indicator text changes accessible name when checked
            cy.findByRole('menuitemradio', {name: /Compact/}).click();
            cy.findByTestId('radio-size').should('have.text', 'compact');
        });

        it('only one radio can be checked at a time', () => {
            openMenu('View');
            cy.findByRole('menuitemradio', {name: /Normal/}).should('have.attr', 'data-state', 'checked');
            cy.findByRole('menuitemradio', {name: /Compact/}).should('have.attr', 'data-state', 'unchecked');
            cy.findByRole('menuitemradio', {name: /Compact/}).click();
            openMenu('View');
            cy.findByRole('menuitemradio', {name: /Compact/}).should('have.attr', 'data-state', 'checked');
            cy.findByRole('menuitemradio', {name: /Normal/}).should('have.attr', 'data-state', 'unchecked');
        });

        it('data-state reflects selection', () => {
            openMenu('View');
            cy.findByRole('menuitemradio', {name: /Normal/}).should('have.attr', 'data-state', 'checked');
            cy.findByRole('menuitemradio', {name: /Compact/}).should('have.attr', 'data-state', 'unchecked');
        });
    });

    // ── 9. Submenus ─────────────────────────────────────────

    describe('submenus', () => {
        it('hover on SubTrigger opens submenu', () => {
            openMenu('Edit');
            cy.findByText('Find →').realHover();
            cy.findByText('Search the web…').should('exist');
        });

        it('ArrowRight opens submenu', () => {
            openMenu('Edit');
            cy.findByText('Find →').realHover();
            cy.findByText('Find →').should('have.attr', 'data-highlighted');
            cy.realPress('ArrowRight');
            cy.findByText('Search the web…').should('exist');
        });

        it('ArrowLeft closes submenu (returns to parent)', () => {
            openMenu('Edit');
            cy.findByText('Find →').realHover();
            cy.findByText('Search the web…').should('exist');
            cy.realPress('ArrowRight');
            cy.findByText('Search the web…').should('have.attr', 'data-highlighted');
            cy.realPress('ArrowLeft');
            cy.findByText('Search the web…').should('not.exist');
            // Parent menu should still be open
            shouldHaveMenuOpen();
        });

        it('Escape from submenu closes entire menu tree', () => {
            openMenu('Edit');
            cy.findByText('Find →').realHover();
            cy.findByText('Search the web…').should('exist');
            cy.realPress('ArrowRight');
            cy.findByText('Search the web…').should('have.attr', 'data-highlighted');
            cy.realPress('Escape');
            shouldHaveMenuClosed();
        });

        it('click sub item activates and closes entire menu', () => {
            openMenu('Edit');
            cy.findByText('Find →').realHover();
            cy.findByText('Search the web…').should('exist');
            cy.findByText('Search the web…').click();
            shouldHaveMenuClosed();
            cy.findByTestId('last-action').should('have.text', 'Search the web…');
        });
    });

    // ── 10. RTL ──────────────────────────────────────────────

    describe('RTL', () => {
        beforeEach(() => {
            cy.findByLabelText('rtl').click();
        });

        it('content right edge aligns with trigger right edge in RTL', () => {
            getTrigger('File').click();
            shouldHaveMenuOpen();

            getTrigger('File').then(($trigger) => {
                const triggerRect = $trigger[0].getBoundingClientRect();

                // Measure the positioned wrapper (floating-ui target element).
                cy.get('[data-radix-popper-content-wrapper]').first().then(($wrapper) => {
                    const wrapperRect = $wrapper[0].getBoundingClientRect();
                    // In RTL with align="start", the right (start) edge of content
                    // should align with the right (start) edge of the trigger.
                    expect(wrapperRect.right).to.be.closeTo(triggerRect.right, 1,
                        `wrapper(left=${wrapperRect.left}, right=${wrapperRect.right}, w=${wrapperRect.width}, dir=${$wrapper[0].getAttribute('dir')}) ` +
                        `trigger(left=${triggerRect.left}, right=${triggerRect.right}, w=${triggerRect.width})`);
                });
            });
        });
    });

    // ── 11. Disabled State ──────────────────────────────────

    describe('disabled state', () => {
        beforeEach(() => {
            cy.findByLabelText('disabled').click();
        });

        it('disabled item has data-disabled attribute', () => {
            openMenu('Edit');
            cy.findByText('Undo').should('have.attr', 'data-disabled');
        });

        it('ArrowDown skips disabled items', () => {
            getTrigger('Edit').focus();
            cy.realPress('ArrowDown');
            shouldHaveMenuOpen();
            // Should skip Undo (disabled) and land on Redo
            cy.focused().should('have.text', 'Redo');
        });

        it('disabled item does not trigger onSelect', () => {
            openMenu('Edit');
            cy.findByText('Undo').click({force: true});
            // Menu should still be open (disabled items don't close menu)
            // and last-action should be empty
            cy.findByTestId('last-action').should('have.text', '');
        });
    });
});
