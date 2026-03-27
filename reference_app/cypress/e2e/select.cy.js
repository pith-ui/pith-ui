describe('Select', () => {
    // ── Helpers ──────────────────────────────────────────────

    function shouldBeOpen() {
        cy.findByRole('listbox').should('exist');
    }

    function shouldBeClosed() {
        // Select content may stay in DOM; check trigger data-state instead
        getTrigger().should('have.attr', 'data-state', 'closed');
    }

    // Use CSS selector since trigger gets aria-hidden when select is open (modal)
    function getTrigger() {
        return cy.get('[data-testid="select-trigger"]');
    }

    function openSelect() {
        getTrigger().click();
        shouldBeOpen();
    }

    beforeEach(() => {
        cy.visit('/select');
    });

    // ── 1. Accessibility Semantics ──────────────────────────

    describe('accessibility', () => {
        it('Trigger has role="combobox"', () => {
            getTrigger().should('have.attr', 'role', 'combobox');
        });

        it('Trigger has aria-expanded', () => {
            getTrigger().should('have.attr', 'aria-expanded', 'false');
            openSelect();
            getTrigger().should('have.attr', 'aria-expanded', 'true');
        });

        it('Content has role="listbox"', () => {
            openSelect();
            cy.findByRole('listbox').should('exist');
        });

        it('Items have role="option"', () => {
            openSelect();
            cy.findAllByRole('option').should('have.length.at.least', 1);
        });

        it('Groups have role="group"', () => {
            openSelect();
            cy.findAllByRole('group').should('have.length', 2);
        });

        it('Groups are labelled by their Label', () => {
            openSelect();
            cy.findAllByRole('group')
                .first()
                .should('have.attr', 'aria-labelledby')
                .then((labelId) => {
                    cy.get(`#${labelId}`).should('have.text', 'Fruits');
                });
        });

        it('Trigger has aria-autocomplete="none"', () => {
            getTrigger().should('have.attr', 'aria-autocomplete', 'none');
        });

        it('Trigger aria-controls references Content id when open', () => {
            openSelect();
            getTrigger()
                .invoke('attr', 'aria-controls')
                .then((controlsId) => {
                    cy.findByRole('listbox').should('have.attr', 'id', controlsId);
                });
        });
    });

    // ── 2. Content Sizing ─────────────────────────────────────

    describe('content sizing', () => {
        it('content min-width matches trigger width', () => {
            getTrigger().then($trigger => {
                const triggerWidth = $trigger[0].getBoundingClientRect().width;
                openSelect();
                cy.findByRole('listbox').then($content => {
                    const contentWidth = $content[0].getBoundingClientRect().width;
                    expect(contentWidth).to.be.at.least(triggerWidth);
                });
            });
        });
    });

    // ── 3. Data Attributes ──────────────────────────────────

    describe('data attributes', () => {
        it('Trigger data-state closed→open→closed', () => {
            getTrigger().should('have.attr', 'data-state', 'closed');
            openSelect();
            getTrigger().should('have.attr', 'data-state', 'open');
            cy.realPress('Escape');
            shouldBeClosed();
            getTrigger().should('have.attr', 'data-state', 'closed');
        });

        it('Trigger has data-placeholder when no value selected', () => {
            getTrigger().should('have.attr', 'data-placeholder');
        });

        it('Trigger loses data-placeholder after selection', () => {
            openSelect();
            cy.findByRole('option', {name: 'Apple'}).click();
            shouldBeClosed();
            getTrigger().should('not.have.attr', 'data-placeholder');
        });

        it('Content has data-state="open"', () => {
            openSelect();
            cy.findByRole('listbox').should('have.attr', 'data-state', 'open');
        });

        it('Content has data-side attribute (popper mode)', () => {
            openSelect();
            cy.findByRole('listbox').should('have.attr', 'data-side');
        });

        it('Content has data-align attribute (popper mode)', () => {
            openSelect();
            cy.findByRole('listbox').should('have.attr', 'data-align');
        });

        it('Item data-state reflects checked/unchecked', () => {
            openSelect();
            // No item selected initially
            cy.findByRole('option', {name: 'Apple'}).should('have.attr', 'data-state', 'unchecked');
            cy.findByRole('option', {name: 'Apple'}).click();
            shouldBeClosed();
            openSelect();
            cy.findByRole('option', {name: 'Apple'}).should('have.attr', 'data-state', 'checked');
            cy.findByRole('option', {name: 'Banana'}).should('have.attr', 'data-state', 'unchecked');
        });

        it('Disabled item has data-disabled', () => {
            openSelect();
            cy.findByRole('option', {name: 'Cherry'}).should('have.attr', 'data-disabled');
        });

        it('Item has data-highlighted when focused', () => {
            getTrigger().focus();
            cy.realPress('ArrowDown');
            shouldBeOpen();
            cy.findAllByRole('option').not('[data-disabled]').first().should('have.attr', 'data-highlighted');
        });
    });

    // ── 3. Keyboard Navigation ──────────────────────────────

    describe('keyboard navigation', () => {
        it('Space opens select from trigger', () => {
            getTrigger().focus();
            cy.realPress('Space');
            shouldBeOpen();
        });

        it('Enter opens select from trigger', () => {
            getTrigger().focus();
            cy.realPress('Enter');
            shouldBeOpen();
        });

        it('ArrowDown opens select from trigger', () => {
            getTrigger().focus();
            cy.realPress('ArrowDown');
            shouldBeOpen();
        });

        it('ArrowUp opens select from trigger', () => {
            getTrigger().focus();
            cy.realPress('ArrowUp');
            shouldBeOpen();
        });

        it('ArrowDown navigates to next option', () => {
            getTrigger().focus();
            cy.realPress('ArrowDown');
            shouldBeOpen();
            cy.realPress('ArrowDown');
            // Should move to next non-disabled option
            cy.findByRole('option', {name: 'Avocado'}).should('have.attr', 'data-highlighted');
        });

        it('ArrowUp navigates to previous option', () => {
            getTrigger().focus();
            cy.realPress('ArrowDown');
            shouldBeOpen();
            cy.realPress('ArrowDown');
            cy.realPress('ArrowUp');
            cy.findByRole('option', {name: 'Apple'}).should('have.attr', 'data-highlighted');
        });

        it('Enter selects highlighted item', () => {
            // Open via click, then use keyboard to navigate and select
            openSelect();
            cy.findByRole('option', {name: 'Apple'}).should('have.attr', 'data-highlighted');
            cy.realPress('ArrowDown'); // Avocado
            cy.findByRole('option', {name: 'Avocado'}).should('have.attr', 'data-highlighted');
            cy.realPress('Enter');
            shouldBeClosed();
            cy.findByTestId('select-value').should('have.text', 'avocado');
        });

        it('Space selects and closes', () => {
            getTrigger().focus();
            cy.realPress('Space');
            shouldBeOpen();
            // First option highlighted
            cy.realPress('Space');
            shouldBeClosed();
            cy.findByTestId('select-value').should('have.text', 'apple');
        });

        it('Escape closes without selecting', () => {
            openSelect();
            cy.realPress('Escape');
            shouldBeClosed();
            cy.findByTestId('select-value').should('have.text', '(none)');
        });

        it('Home moves to first option', () => {
            getTrigger().focus();
            cy.realPress('ArrowDown');
            shouldBeOpen();
            cy.realPress('ArrowDown');
            cy.realPress('ArrowDown');
            cy.realPress('Home');
            cy.findByRole('option', {name: 'Apple'}).should('have.attr', 'data-highlighted');
        });

        it('End moves to last option', () => {
            getTrigger().focus();
            cy.realPress('ArrowDown');
            shouldBeOpen();
            cy.realPress('End');
            cy.findByRole('option', {name: 'Potato'}).should('have.attr', 'data-highlighted');
        });

        it('Tab does not close select (prevented)', () => {
            openSelect();
            cy.realPress('Tab');
            shouldBeOpen();
        });

        it('ArrowDown skips disabled options', () => {
            getTrigger().focus();
            cy.realPress('ArrowDown');
            shouldBeOpen();
            // Starts on Apple
            cy.realPress('ArrowDown');
            // Avocado
            cy.realPress('ArrowDown');
            // Banana
            cy.realPress('ArrowDown');
            // Should skip Cherry (disabled) and go to Carrot
            cy.findByRole('option', {name: 'Carrot'}).should('have.attr', 'data-highlighted');
        });
    });

    // ── 4. Pointer Interaction ──────────────────────────────

    describe('pointer interaction', () => {
        it('click trigger opens select', () => {
            getTrigger().click();
            shouldBeOpen();
        });

        it('click option selects and closes', () => {
            openSelect();
            cy.findByRole('option', {name: 'Banana'}).click();
            shouldBeClosed();
            cy.findByTestId('select-value').should('have.text', 'banana');
        });

        it('click outside closes select', () => {
            openSelect();
            cy.get('body').realClick({position: {x: 1, y: 1}});
            shouldBeClosed();
        });
    });

    // ── 5. Focus Management ─────────────────────────────────

    describe('focus management', () => {
        it('focus returns to trigger on close', () => {
            openSelect();
            cy.realPress('Escape');
            shouldBeClosed();
            getTrigger().should('be.focused');
        });

        it('selected item is highlighted when reopening', () => {
            // Select Banana
            openSelect();
            cy.findByRole('option', {name: 'Banana'}).click();
            shouldBeClosed();
            // Reopen — Banana should be highlighted
            openSelect();
            cy.findByRole('option', {name: 'Banana'}).should('have.attr', 'data-highlighted');
        });
    });

    // ── 6. Value Display ────────────────────────────────────

    describe('value display', () => {
        it('shows placeholder initially', () => {
            getTrigger().should('contain.text', 'Select a fruit...');
        });

        it('shows selected item text in trigger', () => {
            openSelect();
            cy.findByRole('option', {name: 'Carrot'}).click();
            shouldBeClosed();
            getTrigger().should('contain.text', 'Carrot');
        });

        it('readout updates on selection', () => {
            cy.findByTestId('select-value').should('have.text', '(none)');
            openSelect();
            cy.findByRole('option', {name: 'Apple'}).click();
            cy.findByTestId('select-value').should('have.text', 'apple');
        });
    });

    // ── 7. Disabled State ───────────────────────────────────

    describe('disabled state', () => {
        it('disabled trigger cannot be opened', () => {
            cy.findByLabelText('disabled').click();
            getTrigger().should('have.attr', 'data-disabled');
            getTrigger().click({force: true});
            shouldBeClosed();
        });

        it('disabled item has data-disabled and cannot be selected', () => {
            openSelect();
            cy.findByRole('option', {name: 'Cherry'}).should('have.attr', 'data-disabled');
            cy.findByRole('option', {name: 'Cherry'}).click();
            // Select should stay open — disabled item doesn't select
            shouldBeOpen();
        });
    });

    // ── 8. Groups ───────────────────────────────────────────

    describe('groups', () => {
        it('navigation continues across groups', () => {
            getTrigger().focus();
            cy.realPress('ArrowDown');
            shouldBeOpen();
            // Navigate through: Apple → Avocado → Banana → (skip Cherry) → Carrot → Potato
            cy.realPress('ArrowDown'); // Avocado
            cy.realPress('ArrowDown'); // Banana
            cy.realPress('ArrowDown'); // Carrot (skips disabled Cherry, crosses group boundary)
            cy.findByRole('option', {name: 'Carrot'}).should('have.attr', 'data-highlighted');
            cy.realPress('ArrowDown'); // Potato
            cy.findByRole('option', {name: 'Potato'}).should('have.attr', 'data-highlighted');
        });
    });

    // ── 9. Typeahead ────────────────────────────────────────

    describe('typeahead', () => {
        it('typing a character highlights matching item when open', () => {
            openSelect();
            // Type "b" — should highlight Banana (first "b" item)
            cy.realPress('b');
            cy.findByRole('option', {name: 'Banana'}).should('have.attr', 'data-highlighted');
        });

        it('typing skips disabled items', () => {
            openSelect();
            // Type "c" — should skip disabled Cherry and highlight Carrot
            cy.realPress('c');
            cy.findByRole('option', {name: 'Carrot'}).should('have.attr', 'data-highlighted');
        });

        it('repeated character cycles through matching items', () => {
            openSelect();
            // Type "a" — should highlight Apple (first "a" item)
            cy.realPress('a');
            cy.findByRole('option', {name: 'Apple'}).should('have.attr', 'data-highlighted');
            // Type "a" again — should cycle to Avocado
            cy.realPress('a');
            cy.findByRole('option', {name: 'Avocado'}).should('have.attr', 'data-highlighted');
        });

        it('multi-character search narrows results', () => {
            openSelect();
            // Type "ba" quickly — should highlight Banana
            cy.realPress('b');
            cy.realPress('a');
            cy.findByRole('option', {name: 'Banana'}).should('have.attr', 'data-highlighted');
        });

        it('typing "p" highlights Potato', () => {
            openSelect();
            cy.realPress('p');
            cy.findByRole('option', {name: 'Potato'}).should('have.attr', 'data-highlighted');
        });
    });

    // ── 10. Default Value ───────────────────────────────────

    describe('default value', () => {
        it('trigger has no data-placeholder when default value is set', () => {
            cy.get('[data-testid="default-trigger"]').should('not.have.attr', 'data-placeholder');
        });

        it('default value item is checked when opened', () => {
            cy.get('[data-testid="default-trigger"]').click();
            cy.findByRole('listbox').should('exist');
            cy.findByRole('option', {name: 'Banana'}).should('have.attr', 'data-state', 'checked');
        });

        it('default value item is highlighted when opened', () => {
            cy.get('[data-testid="default-trigger"]').click();
            cy.findByRole('listbox').should('exist');
            cy.findByRole('option', {name: 'Banana'}).should('have.attr', 'data-highlighted');
        });

        it('trigger shows default value text on initial render', () => {
            // The trigger must display the selected item's text immediately on page load,
            // without requiring the user to open and close the select first.
            cy.get('[data-testid="default-trigger"]').should('contain.text', 'Banana');
        });

        it('trigger shows default value text after open/close', () => {
            cy.get('[data-testid="default-trigger"]').click();
            cy.findByRole('listbox').should('exist');
            cy.realPress('Escape');
            cy.get('[data-testid="default-trigger"]').should('have.attr', 'data-state', 'closed');
            cy.get('[data-testid="default-trigger"]').should('contain.text', 'Banana');
        });
    });

    // ── 11. Form Integration ──────────────────────────────────

    describe('form integration', () => {
        function getFormTrigger() {
            return cy.get('[data-testid="form-trigger"]');
        }

        it('form trigger shows default value text on initial render', () => {
            getFormTrigger().should('contain.text', 'France');
        });

        it('form select has default value after open/close', () => {
            getFormTrigger().click();
            cy.findByRole('listbox').should('exist');
            cy.realPress('Escape');
            getFormTrigger().should('have.attr', 'data-state', 'closed');
            getFormTrigger().should('contain.text', 'France');
        });

        it('form data updates live on select change', () => {
            // Initially should be empty or show default
            getFormTrigger().click();
            cy.findByRole('listbox').should('exist');
            cy.findByRole('option', {name: 'United Kingdom'}).click();
            getFormTrigger().should('have.attr', 'data-state', 'closed');
            // Form data should update immediately without submit
            cy.findByTestId('form-data').should('contain.text', 'uk');
        });

        it('form data includes country field after submit', () => {
            cy.findByRole('button', {name: 'Submit'}).click();
            cy.findByTestId('form-data').should('contain.text', 'country');
            cy.findByTestId('form-data').should('contain.text', 'fr');
        });

        it('form data reflects changed selection after submit', () => {
            getFormTrigger().click();
            cy.findByRole('listbox').should('exist');
            cy.findByRole('option', {name: 'Spain'}).click();
            getFormTrigger().should('have.attr', 'data-state', 'closed');
            cy.findByRole('button', {name: 'Submit'}).click();
            cy.findByTestId('form-data').should('contain.text', 'es');
        });
    });

    // ── 12. Item-Aligned Positioning ────────────────────────

    describe('item-aligned positioning', () => {
        function getAlignedTrigger() {
            return cy.get('[data-testid="aligned-trigger"]');
        }

        function openAligned() {
            getAlignedTrigger().click();
            cy.findByRole('listbox').should('exist');
        }

        it('aligned trigger shows default value text on initial render', () => {
            getAlignedTrigger().should('contain.text', 'Banana');
        });

        it('content does not have data-side (not popper mode)', () => {
            openAligned();
            cy.findByRole('listbox').should('not.have.attr', 'data-side');
        });

        it('content does not have data-align (not popper mode)', () => {
            openAligned();
            cy.findByRole('listbox').should('not.have.attr', 'data-align');
        });

        it('selected item is vertically aligned with trigger', () => {
            getAlignedTrigger().then(($trigger) => {
                const triggerRect = $trigger[0].getBoundingClientRect();
                const triggerCenterY = triggerRect.top + triggerRect.height / 2;

                openAligned();

                // Find the checked/selected item (Banana, the default value)
                cy.findByRole('option', {name: 'Banana'}).then(($item) => {
                    const itemRect = $item[0].getBoundingClientRect();
                    const itemCenterY = itemRect.top + itemRect.height / 2;
                    // Allow some tolerance (within 20px) for alignment
                    expect(itemCenterY).to.be.closeTo(triggerCenterY, 20);
                });
            });
        });

        it('content is sized by its items, not available viewport space', () => {
            openAligned();
            cy.findByRole('listbox').then(($content) => {
                const contentHeight = $content[0].getBoundingClientRect().height;
                // 5 items at ~30px each + padding ≈ under 250px
                // It should NOT fill the viewport (~720px in Cypress)
                expect(contentHeight).to.be.lessThan(250);
            });
        });

        it('content visible area (with background) matches content height', () => {
            openAligned();
            cy.get('[data-testid="aligned-content"]').then(($content) => {
                const el = $content[0];
                const contentHeight = el.getBoundingClientRect().height;
                // The styled content element should wrap its items tightly
                // not stretch to fill the positioning wrapper
                expect(contentHeight).to.be.lessThan(250);
                // Also check the parent wrapper is not visually styled
                const wrapper = el.parentElement;
                if (wrapper) {
                    const wrapperStyle = window.getComputedStyle(wrapper);
                    const wrapperBg = wrapperStyle.backgroundColor;
                    // Wrapper should be transparent (no visible background)
                    expect(['transparent', 'rgba(0, 0, 0, 0)']).to.include(wrapperBg);
                }
            });
        });

        it('no ancestor of listbox between wrapper and listbox has visible styling', () => {
            openAligned();
            cy.findByRole('listbox').then(($listbox) => {
                const listbox = $listbox[0];
                // Walk up the DOM tree from listbox to the position:fixed wrapper
                // Every element between them should be transparent with no visible borders/shadows
                let el = listbox.parentElement;
                const issues = [];
                while (el) {
                    const style = window.getComputedStyle(el);
                    const tag = el.tagName.toLowerCase();
                    const classes = el.className;
                    const height = el.getBoundingClientRect().height;
                    const bg = style.backgroundColor;
                    const border = style.borderTopWidth;
                    const shadow = style.boxShadow;
                    const overflow = style.overflow;
                    const position = style.position;

                    // Log for debugging
                    cy.log(`Ancestor: <${tag} class="${classes}"> h=${Math.round(height)} bg=${bg} border=${border} shadow=${shadow} overflow=${overflow} position=${position}`);

                    // Check for visible styling that would make this element appear as a "box"
                    const hasBg = bg !== 'transparent' && bg !== 'rgba(0, 0, 0, 0)';
                    const hasBorder = parseFloat(border) > 0;
                    const hasShadow = shadow !== 'none';
                    const hasOverflow = overflow === 'hidden' || overflow === 'auto' || overflow === 'scroll';

                    if ((hasBg || hasBorder || hasShadow) && height > 250) {
                        issues.push(`<${tag} class="${classes}"> has visible styling (bg=${bg}, border=${border}, shadow=${shadow}) and is ${Math.round(height)}px tall`);
                    }
                    if (hasOverflow && height > 250) {
                        issues.push(`<${tag} class="${classes}"> has overflow:${overflow} and is ${Math.round(height)}px tall`);
                    }

                    if (position === 'fixed') break; // Stop at the wrapper
                    el = el.parentElement;
                }
                expect(issues).to.deep.equal([]);
            });
        });

        it('the styled content element is tightly sized around its items', () => {
            openAligned();
            // Find the element with visible background (the styled content box)
            // Walk from body to find any position:fixed descendant with a background
            cy.findByRole('listbox').then(($listbox) => {
                const listbox = $listbox[0];
                const listboxHeight = listbox.getBoundingClientRect().height;
                // The listbox (content div) should be tightly sized
                expect(listboxHeight).to.be.lessThan(250);

                // Check the wrapper (parent with position:fixed) is not styled
                let el = listbox.parentElement;
                while (el) {
                    const style = window.getComputedStyle(el);
                    if (style.position === 'fixed') {
                        const wrapperBg = style.backgroundColor;
                        const wrapperBorder = parseFloat(style.borderTopWidth);
                        const wrapperShadow = style.boxShadow;
                        // Wrapper should have no visible styling
                        const hasVisibleStyle = (wrapperBg !== 'transparent' && wrapperBg !== 'rgba(0, 0, 0, 0)')
                            || wrapperBorder > 0
                            || wrapperShadow !== 'none';
                        expect(hasVisibleStyle, 'wrapper should not have visible styling').to.be.false;
                        break;
                    }
                    el = el.parentElement;
                }
            });
        });

        it('content width is at least trigger width', () => {
            getAlignedTrigger().then(($trigger) => {
                const triggerWidth = $trigger[0].getBoundingClientRect().width;
                openAligned();
                cy.findByRole('listbox').then(($content) => {
                    const contentWidth = $content[0].getBoundingClientRect().width;
                    expect(contentWidth).to.be.at.least(triggerWidth);
                });
            });
        });

        it('keyboard navigation works in item-aligned mode', () => {
            getAlignedTrigger().focus();
            cy.realPress('ArrowDown');
            cy.findByRole('listbox').should('exist');
            // Default is Banana, ArrowDown should move to Cherry
            cy.realPress('ArrowDown');
            cy.findByRole('option', {name: 'Cherry'}).should('have.attr', 'data-highlighted');
        });

        it('selection works in item-aligned mode', () => {
            openAligned();
            cy.findByRole('option', {name: 'Apple'}).click();
            getAlignedTrigger().should('have.attr', 'data-state', 'closed');
            getAlignedTrigger().should('contain.text', 'Apple');
        });
    });

    // ── 13. Context Menu Suppression ─────────────────────────

    describe('context menu suppression', () => {
        it('right-click on content does not open context menu', () => {
            openSelect();
            // Trigger contextmenu event on the listbox and verify it is prevented
            cy.findByRole('listbox').trigger('contextmenu');
            // Select should still be open (contextmenu was suppressed)
            shouldBeOpen();
        });
    });

    // ── 14. Window Events ────────────────────────────────────

    describe('window events', () => {
        it('select closes on window resize', () => {
            // select-bp-3
            openSelect();
            cy.window().trigger('resize');
            shouldBeClosed();
        });

        it('select closes on window blur', () => {
            // select-bp-3
            openSelect();
            cy.window().trigger('blur');
            shouldBeClosed();
        });
    });

    // ── 15. Controlled Value (Dual Selects) ─────────────────

    describe('controlled value', () => {
        it('readout shows initial controlled value', () => {
            cy.findByTestId('controlled-value').should('have.text', 'uk');
        });

        it('controlled triggers show initial value text on render', () => {
            // Both triggers share the same controlled value ('uk') and should
            // display the item text immediately without opening the select.
            cy.get('[data-testid="controlled-trigger-a"]').should('contain.text', 'United Kingdom');
            cy.get('[data-testid="controlled-trigger-b"]').should('contain.text', 'United Kingdom');
        });

        it('changing Select A updates Select B and readout', () => {
            cy.get('[data-testid="controlled-trigger-a"]').click();
            cy.findByRole('listbox').should('exist');
            cy.findByRole('option', {name: 'France'}).click();
            cy.get('[data-testid="controlled-trigger-a"]').should('have.attr', 'data-state', 'closed');
            // Readout should update immediately
            cy.findByTestId('controlled-value').should('have.text', 'fr');
            // Trigger A should show the selected text
            cy.get('[data-testid="controlled-trigger-a"]').should('contain.text', 'France');
            // Open and close Select B to verify it has the synced value
            cy.get('[data-testid="controlled-trigger-b"]').click();
            cy.findByRole('listbox').should('exist');
            cy.findByRole('option', {name: 'France'}).should('have.attr', 'data-state', 'checked');
            cy.realPress('Escape');
            cy.get('[data-testid="controlled-trigger-b"]').should('have.attr', 'data-state', 'closed');
            cy.get('[data-testid="controlled-trigger-b"]').should('contain.text', 'France');
        });

        it('changing Select B updates Select A and readout', () => {
            cy.get('[data-testid="controlled-trigger-b"]').click();
            cy.findByRole('listbox').should('exist');
            cy.findByRole('option', {name: 'Spain'}).click();
            cy.get('[data-testid="controlled-trigger-b"]').should('have.attr', 'data-state', 'closed');
            // Readout should update immediately
            cy.findByTestId('controlled-value').should('have.text', 'es');
            // Trigger B should show the selected text
            cy.get('[data-testid="controlled-trigger-b"]').should('contain.text', 'Spain');
            // Open and close Select A to verify it has the synced value
            cy.get('[data-testid="controlled-trigger-a"]').click();
            cy.findByRole('listbox').should('exist');
            cy.findByRole('option', {name: 'Spain'}).should('have.attr', 'data-state', 'checked');
            cy.realPress('Escape');
            cy.get('[data-testid="controlled-trigger-a"]').should('have.attr', 'data-state', 'closed');
            cy.get('[data-testid="controlled-trigger-a"]').should('contain.text', 'Spain');
        });
    });

    // ── Axe Accessibility Audit ─────────────────────────────

    describe('axe audit', () => {
        it('no violations in closed state', () => {
            cy.checkComponentA11y();
        });

        it('no violations when open', () => {
            openSelect();
            cy.checkComponentA11y();
        });
    });
});
