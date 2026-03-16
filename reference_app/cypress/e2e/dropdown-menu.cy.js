describe('Dropdown Menu', () => {
    // ── Helpers ──────────────────────────────────────────────

    function shouldBeOpen() {
        cy.findByRole('menu').should('exist');
    }

    function shouldBeClosed() {
        cy.findByRole('menu').should('not.exist');
    }

    // Use a CSS selector for the trigger since aria-hidden is set on root when modal menu is open.
    function getTrigger() {
        return cy.get('.dropdown-trigger');
    }

    function openMenu() {
        getTrigger().click();
        shouldBeOpen();
    }

    beforeEach(() => {
        cy.visit('/dropdown-menu');
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

        it('Trigger has aria-haspopup="menu"', () => {
            getTrigger().should('have.attr', 'aria-haspopup', 'menu');
        });

        it('Trigger aria-expanded reflects open state', () => {
            getTrigger().should('have.attr', 'aria-expanded', 'false');
            openMenu();
            getTrigger().should('have.attr', 'aria-expanded', 'true');
        });

        it('Trigger aria-controls references Content id', () => {
            openMenu();
            getTrigger()
                .invoke('attr', 'aria-controls')
                .then((controlsId) => {
                    cy.findByRole('menu').should('have.attr', 'id', controlsId);
                });
        });
    });

    // ── 2. Data Attributes ──────────────────────────────────

    describe('data attributes', () => {
        it('Trigger data-state closed→open→closed', () => {
            getTrigger().should('have.attr', 'data-state', 'closed');
            openMenu();
            getTrigger().should('have.attr', 'data-state', 'open');
            cy.realPress('Escape');
            shouldBeClosed();
            getTrigger().should('have.attr', 'data-state', 'closed');
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

        it('Item has data-highlighted when focused via keyboard', () => {
            getTrigger().focus();
            cy.realPress('ArrowDown');
            shouldBeOpen();
            cy.findAllByRole('menuitem').first().should('have.attr', 'data-highlighted');
        });

        it('CheckboxItem data-state toggles checked/unchecked', () => {
            openMenu();
            cy.findByRole('menuitemcheckbox').should('have.attr', 'data-state', 'unchecked');
            cy.findByRole('menuitemcheckbox').click();
            // Menu closes after click; reopen to verify
            openMenu();
            cy.findByRole('menuitemcheckbox').should('have.attr', 'data-state', 'checked');
        });

        it('RadioItem data-state reflects selection', () => {
            openMenu();
            cy.findAllByRole('menuitemradio').eq(0).should('have.attr', 'data-state', 'checked');
            cy.findAllByRole('menuitemradio').eq(1).should('have.attr', 'data-state', 'unchecked');
            // Select second radio
            cy.findAllByRole('menuitemradio').eq(1).click();
            openMenu();
            cy.findAllByRole('menuitemradio').eq(0).should('have.attr', 'data-state', 'unchecked');
            cy.findAllByRole('menuitemradio').eq(1).should('have.attr', 'data-state', 'checked');
        });
    });

    // ── 3. Keyboard Navigation ──────────────────────────────

    describe('keyboard navigation', () => {
        it('Space opens menu from trigger', () => {
            getTrigger().focus();
            cy.realPress('Space');
            shouldBeOpen();
        });

        it('Enter opens menu from trigger', () => {
            getTrigger().focus();
            cy.realPress('Enter');
            shouldBeOpen();
        });

        it('ArrowDown opens menu from trigger', () => {
            getTrigger().focus();
            cy.realPress('ArrowDown');
            shouldBeOpen();
        });

        it('ArrowDown navigates to next item', () => {
            getTrigger().focus();
            cy.realPress('ArrowDown');
            shouldBeOpen();
            // First item is highlighted
            cy.realPress('ArrowDown');
            cy.findAllByRole('menuitem').eq(1).should('have.attr', 'data-highlighted');
        });

        it('ArrowUp navigates to previous item', () => {
            getTrigger().focus();
            cy.realPress('ArrowDown');
            shouldBeOpen();
            cy.realPress('ArrowDown');
            cy.realPress('ArrowUp');
            cy.findAllByRole('menuitem').first().should('have.attr', 'data-highlighted');
        });

        it('Enter activates focused item', () => {
            getTrigger().focus();
            cy.realPress('ArrowDown');
            shouldBeOpen();
            cy.realPress('Enter');
            shouldBeClosed();
            cy.findByTestId('last-action').should('have.text', 'Item 1');
        });

        it('Space activates focused item', () => {
            getTrigger().focus();
            cy.realPress('ArrowDown');
            shouldBeOpen();
            cy.realPress('Space');
            shouldBeClosed();
            cy.findByTestId('last-action').should('have.text', 'Item 1');
        });

        it('Escape closes menu and restores focus to trigger', () => {
            openMenu();
            cy.realPress('Escape');
            shouldBeClosed();
            getTrigger().should('be.focused');
        });

        it('Home moves focus to first item', () => {
            getTrigger().focus();
            cy.realPress('ArrowDown');
            shouldBeOpen();
            cy.realPress('ArrowDown');
            cy.realPress('ArrowDown');
            cy.realPress('Home');
            cy.findAllByRole('menuitem').first().should('have.attr', 'data-highlighted');
        });

        it('End moves focus to last menu item', () => {
            getTrigger().focus();
            cy.realPress('ArrowDown');
            shouldBeOpen();
            cy.realPress('End');
            cy.get('[role="menu"] > [role="menuitem"]').last().should('have.attr', 'data-highlighted');
        });

        it('ArrowDown skips disabled items', () => {
            cy.findByLabelText('disabled').click();
            getTrigger().focus();
            cy.realPress('ArrowDown');
            shouldBeOpen();
            // First highlighted is Item 1
            cy.realPress('ArrowDown');
            // Should skip Item 2 (disabled) and go to Item 3
            cy.focused().should('have.text', 'Item 3');
        });
    });

    // ── 4. Pointer Interaction ──────────────────────────────

    describe('pointer interaction', () => {
        it('click trigger opens menu', () => {
            getTrigger().click();
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
            cy.findByText('Item 1').realHover();
            cy.findByText('Item 1').should('have.attr', 'data-highlighted');
        });
    });

    // ── 5. Focus Management ─────────────────────────────────

    describe('focus management', () => {
        it('first item is highlighted after keyboard open', () => {
            getTrigger().focus();
            cy.realPress('ArrowDown');
            shouldBeOpen();
            cy.findAllByRole('menuitem').first().should('have.attr', 'data-highlighted');
        });

        it('focus is restored to trigger on close', () => {
            openMenu();
            cy.realPress('Escape');
            shouldBeClosed();
            getTrigger().should('be.focused');
        });
    });

    // ── 6. CheckboxItem ─────────────────────────────────────

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

        it('data-state reflects checked/unchecked', () => {
            openMenu();
            cy.findByRole('menuitemcheckbox').should('have.attr', 'data-state', 'unchecked');
            cy.findByRole('menuitemcheckbox').click();
            openMenu();
            cy.findByRole('menuitemcheckbox').should('have.attr', 'data-state', 'checked');
        });
    });

    // ── 7. RadioGroup ───────────────────────────────────────

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

    // ── 8. Submenu ──────────────────────────────────────────

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
            // Move focus into the submenu
            cy.realPress('ArrowRight');
            cy.findByText('Sub item 1').should('have.attr', 'data-highlighted');
            cy.realPress('ArrowLeft');
            cy.findByText('Sub item 1').should('not.exist');
            // Parent menu should still be open
            shouldBeOpen();
        });

        it('Escape from submenu closes entire menu', () => {
            openMenu();
            cy.findByText('Submenu →').realHover();
            cy.findByText('Sub item 1').should('exist');
            cy.realPress('ArrowRight');
            cy.findByText('Sub item 1').should('have.attr', 'data-highlighted');
            cy.realPress('Escape');
            // Escape from submenu closes entire menu tree
            shouldBeClosed();
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

    // ── 9. Disabled State ───────────────────────────────────

    describe('disabled state', () => {
        beforeEach(() => {
            cy.findByLabelText('disabled').click();
        });

        it('disabled item has data-disabled attribute', () => {
            openMenu();
            cy.findByText('Item 2').should('have.attr', 'data-disabled');
        });

        it('disabled item does not trigger onSelect via keyboard', () => {
            getTrigger().focus();
            cy.realPress('ArrowDown');
            shouldBeOpen();
            // Navigate to Item 2 (disabled) — arrows skip it, so we can't land on it
            // Instead verify that keyboard navigation skips it
            cy.realPress('ArrowDown');
            // Should be on Item 3, not Item 2
            cy.focused().should('have.text', 'Item 3');
        });
    });

    // ── 10. With Tooltip ──────────────────────────────────────

    describe('with tooltip', () => {
        beforeEach(() => {
            cy.visit('/dropdown-menu/with-tooltip');
        });

        it('click opens menu without errors', () => {
            cy.get('.dropdown-trigger').click();
            cy.findByRole('menu').should('exist');
        });

        it('menu content is positioned near trigger (not at 0,0)', () => {
            cy.get('.dropdown-trigger').click();
            cy.findByRole('menu').should('exist');
            cy.findByRole('menu').then(($menu) => {
                const rect = $menu[0].getBoundingClientRect();
                // Content should not be at top-left corner (0,0) — that indicates
                // a positioning failure (e.g., due to popper anchor not being set).
                expect(rect.top).to.be.greaterThan(10);
            });
        });

        it('keyboard open and close works without errors', () => {
            cy.get('.dropdown-trigger').focus();
            cy.realPress('ArrowDown');
            cy.findByRole('menu').should('exist');
            cy.realPress('Escape');
            cy.findByRole('menu').should('not.exist');
            cy.get('.dropdown-trigger').should('be.focused');
        });

        it('can interact with menu items after opening', () => {
            cy.get('.dropdown-trigger').click();
            cy.findByRole('menu').should('exist');
            cy.findByText('Item 1').click();
            cy.findByRole('menu').should('not.exist');
        });

        it('reopen after close works without errors', () => {
            // First open/close cycle
            cy.get('.dropdown-trigger').click();
            cy.findByRole('menu').should('exist');
            cy.realPress('Escape');
            cy.findByRole('menu').should('not.exist');
            // Second open/close cycle — this catches "closure dropped" errors
            // that occur when the tooltip's pointerup listener outlives its scope.
            cy.get('.dropdown-trigger').click();
            cy.findByRole('menu').should('exist');
            cy.findByText('Item 1').click();
            cy.findByRole('menu').should('not.exist');
        });

        it('hover to show tooltip then click to open menu works without errors', () => {
            // Hover to trigger tooltip
            cy.get('.dropdown-trigger').realHover();
            // Wait for tooltip to appear (default delay is 700ms)
            cy.wait(800);
            // Click to open dropdown — tooltip content unmounts, which previously
            // caused "closure invoked after being dropped" because the tooltip's
            // document-level listeners (pointermove, TOOLTIP_OPEN, scroll)
            // referenced Closures stored in StoredValue that got dropped during
            // scope disposal before on_cleanup could remove the listeners.
            cy.get('.dropdown-trigger').click();
            cy.findByRole('menu').should('exist');
            // Interact with an item to verify menu works normally
            cy.findByText('Item 1').click();
            cy.findByRole('menu').should('not.exist');
        });

        it('second open after hover+click cycle works without errors', () => {
            // First cycle: hover → tooltip → click → menu → close
            cy.get('.dropdown-trigger').realHover();
            cy.wait(800);
            cy.get('.dropdown-trigger').click();
            cy.findByRole('menu').should('exist');
            cy.realPress('Escape');
            cy.findByRole('menu').should('not.exist');
            // Second cycle: the DismissableLayer's document-level pointerdown/focusin
            // listeners from the first menu instance were stored in StoredValue.
            // If they weren't properly cleaned up, opening again causes a panic.
            cy.get('.dropdown-trigger').realHover();
            cy.wait(800);
            cy.get('.dropdown-trigger').click();
            cy.findByRole('menu').should('exist');
            cy.findByText('Item 2').click();
            cy.findByRole('menu').should('not.exist');
        });
    });

    // ── 11. Controlled Mode ─────────────────────────────────

    describe('controlled mode', () => {
        function getControlledTrigger() {
            return cy.findByTestId('controlled-dropdown-trigger');
        }

        function getControlledContent() {
            return cy.findByTestId('controlled-dropdown-content');
        }

        function getOpenCheckbox() {
            return cy.findByTestId('controlled-open-checkbox');
        }

        function controlledShouldBeOpen() {
            getControlledContent().should('exist');
        }

        function controlledShouldBeClosed() {
            getControlledContent().should('not.exist');
        }

        it('external checkbox opens dropdown menu', () => {
            // dropdown-menu-msc-1
            controlledShouldBeClosed();
            getOpenCheckbox().click();
            controlledShouldBeOpen();
        });

        it('external control closes dropdown menu', () => {
            // dropdown-menu-msc-1
            getOpenCheckbox().click();
            controlledShouldBeOpen();
            // Use dedicated button since modal overlay blocks checkbox interaction
            cy.findByTestId('controlled-external-close').click({force: true});
            controlledShouldBeClosed();
            cy.findByTestId('controlled-open-state').should('have.text', 'closed');
        });

        it('clicking trigger updates external state', () => {
            // dropdown-menu-msc-1
            getOpenCheckbox().should('not.be.checked');
            getControlledTrigger().click();
            controlledShouldBeOpen();
            getOpenCheckbox().should('be.checked');
        });

        it('closing via Escape updates external state', () => {
            // dropdown-menu-msc-1
            getControlledTrigger().click();
            controlledShouldBeOpen();
            getOpenCheckbox().should('be.checked');
            cy.realPress('Escape');
            controlledShouldBeClosed();
            getOpenCheckbox().should('not.be.checked');
        });
    });

    // ── Internal Styles ─────────────────────────────────────

    describe('event handler composition', () => {
        it('consumer onClick on trigger fires alongside internal handler', () => {
            // Read the initial click count
            cy.findByTestId('trigger-click-count')
                .invoke('text')
                .then((initialCount) => {
                    const startCount = parseInt(initialCount, 10);

                    // Click the trigger — both consumer's onClick and internal toggle should fire
                    cy.findByRole('button', {name: 'open'}).click();

                    // Consumer handler fired (counter incremented)
                    cy.findByTestId('trigger-click-count').should('have.text', String(startCount + 1));

                    // Internal handler fired (menu opened)
                    cy.findByRole('menu').should('exist');
                });
        });
    });

    // ── Attribute Forwarding & Style Merging ────────────────

    describe('attribute forwarding and style merging', () => {
        it('Trigger forwards className and custom data attributes', () => {
            cy.findByRole('button', {name: 'open'})
                .should('have.class', 'dropdown-trigger')
                .and('have.attr', 'data-custom', 'dropdown-trigger-custom');
        });

        it('Content forwards className via class prop', () => {
            openMenu();
            cy.findByRole('menu')
                .should('have.class', 'dropdown-content');
        });

        it('content has outline: none from internal styles', () => {
            openMenu();
            cy.findByRole('menu')
                .should('have.css', 'outline-style', 'none');
        });
    });

    // ── Attribute Forwarding (styles) ─────────────────────

    describe('attribute forwarding (styles)', () => {
        it('internal CSS variables are set', () => {
            openMenu();
            cy.findByRole('menu').then(($el) => {
                const style = getComputedStyle($el[0]);
                const availW = style.getPropertyValue('--radix-dropdown-menu-content-available-width');
                const availH = style.getPropertyValue('--radix-dropdown-menu-content-available-height');
                expect(availW.trim()).to.not.be.empty;
                expect(availH.trim()).to.not.be.empty;
            });
        });

        it('user attr:style overrides popper-derived CSS vars (Leptos limitation)', () => {
            // In React, popper-derived CSS vars are non-overridable ({…props.style, …internal}).
            // In Leptos, content_style uses attr:style which the user's attr:style can override.
            // This is a known divergence — users should avoid overriding these output-only vars.
            openMenu();
            cy.findByRole('menu').then(($el) => {
                const style = getComputedStyle($el[0]);

                const transformOrigin = style.getPropertyValue(
                    '--radix-dropdown-menu-content-transform-origin'
                );
                expect(transformOrigin.trim()).to.equal('0px 0px');

                // Non-overridden internal vars still resolve
                const availW = style.getPropertyValue(
                    '--radix-dropdown-menu-content-available-width'
                );
                expect(availW.trim()).to.not.be.empty;
            });
        });
    });
});
