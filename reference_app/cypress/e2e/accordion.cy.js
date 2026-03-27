describe('Accordion', () => {
    // ── Helpers ──────────────────────────────────────────────

    function shouldBeOpen(itemName) {
        cy.findByRole('button', {name: itemName}).should('have.attr', 'data-state', 'open');
    }

    function shouldBeClosed(itemName) {
        cy.findByRole('button', {name: itemName}).should('have.attr', 'data-state', 'closed');
    }

    function contentShouldBeVisible(itemName) {
        cy.findByRole('button', {name: itemName})
            .invoke('attr', 'aria-controls')
            .then((contentId) => {
                cy.get(`#${contentId}`).should('exist').and('have.attr', 'data-state', 'open');
            });
    }

    function contentShouldBeHidden(itemName) {
        cy.findByRole('button', {name: itemName}).should('have.attr', 'aria-controls');
        cy.findByRole('button', {name: itemName})
            .invoke('attr', 'aria-controls')
            .then((contentId) => {
                cy.get(`#${contentId}`).should('have.attr', 'data-state', 'closed');
                cy.get(`#${contentId}`).should('not.be.visible');
            });
    }

    beforeEach(() => {
        cy.visit('/accordion');
    });

    // ── 1. Accessibility Semantics ──────────────────────────

    describe('accessibility', () => {
        it('Triggers have role="button"', () => {
            cy.findByRole('button', {name: 'Item 1'}).should('exist');
            cy.findByRole('button', {name: 'Item 3'}).should('exist');
        });

        it('Triggers are wrapped in h3 headings', () => {
            cy.findByRole('button', {name: 'Item 1'}).closest('h3').should('exist');
            cy.findByRole('button', {name: 'Item 3'}).closest('h3').should('exist');
        });

        it('Trigger has aria-expanded reflecting open state', () => {
            cy.findByRole('button', {name: 'Item 1'}).should('have.attr', 'aria-expanded', 'false');
            cy.findByRole('button', {name: 'Item 1'}).click();
            cy.findByRole('button', {name: 'Item 1'}).should('have.attr', 'aria-expanded', 'true');
        });

        it('Trigger has aria-controls pointing to content region', () => {
            cy.findByRole('button', {name: 'Item 1'}).click();
            cy.findByRole('button', {name: 'Item 1'})
                .invoke('attr', 'aria-controls')
                .then((contentId) => {
                    cy.get(`#${contentId}`).should('exist').and('have.attr', 'role', 'region');
                });
        });

        it('Content has role="region"', () => {
            cy.findByRole('button', {name: 'Item 1'}).click();
            cy.findByRole('region').should('exist');
        });

        it('Content has aria-labelledby pointing to trigger', () => {
            cy.findByRole('button', {name: 'Item 1'}).click();
            cy.findByRole('button', {name: 'Item 1'})
                .invoke('attr', 'id')
                .then((triggerId) => {
                    cy.findByRole('region').should('have.attr', 'aria-labelledby', triggerId);
                });
        });
    });

    // ── 2. Data Attributes ──────────────────────────────────

    describe('data attributes', () => {
        it('Item data-state: closed by default, open when expanded', () => {
            cy.findByTestId('item-1').should('have.attr', 'data-state', 'closed');
            cy.findByRole('button', {name: 'Item 1'}).click();
            cy.findByTestId('item-1').should('have.attr', 'data-state', 'open');
        });

        it('Trigger data-state reflects item open state', () => {
            shouldBeClosed('Item 1');
            cy.findByRole('button', {name: 'Item 1'}).click();
            shouldBeOpen('Item 1');
            cy.findByRole('button', {name: 'Item 3'}).click();
            shouldBeClosed('Item 1');
        });

        it('Content data-state reflects item open state', () => {
            cy.findByRole('button', {name: 'Item 1'}).click();
            contentShouldBeVisible('Item 1');
            cy.findByRole('button', {name: 'Item 3'}).click();
            contentShouldBeHidden('Item 1');
        });

        it('data-orientation on root, item, trigger, content', () => {
            cy.findByTestId('accordion-root').should('have.attr', 'data-orientation', 'vertical');
            cy.findByTestId('item-1').should('have.attr', 'data-orientation', 'vertical');
            cy.findByRole('button', {name: 'Item 1'}).should('have.attr', 'data-orientation', 'vertical');
            cy.findByRole('button', {name: 'Item 1'}).click();
            cy.findByRole('region').should('have.attr', 'data-orientation', 'vertical');
        });

        it('data-disabled on disabled items', () => {
            cy.findByTestId('item-2').should('have.attr', 'data-disabled');
            cy.findByRole('button', {name: 'Item 2'}).should('have.attr', 'data-disabled');
            cy.findByTestId('item-1').should('not.have.attr', 'data-disabled');
        });

        it('Header has data-state and data-orientation', () => {
            cy.findByRole('button', {name: 'Item 1'}).closest('h3').should('have.attr', 'data-state', 'closed');
            cy.findByRole('button', {name: 'Item 1'})
                .closest('h3')
                .should('have.attr', 'data-orientation', 'vertical');
            cy.findByRole('button', {name: 'Item 1'}).click();
            cy.findByRole('button', {name: 'Item 1'}).closest('h3').should('have.attr', 'data-state', 'open');
        });
    });

    // ── 3. Keyboard Navigation ──────────────────────────────

    describe('keyboard navigation', () => {
        it('Space toggles item', () => {
            cy.findByRole('button', {name: 'Item 1'}).focus();
            cy.realPress('Space');
            shouldBeOpen('Item 1');
            cy.realPress('Space');
            // In single non-collapsible mode, it stays open
            shouldBeOpen('Item 1');
        });

        it('Enter toggles item', () => {
            cy.findByRole('button', {name: 'Item 1'}).focus();
            cy.realPress('Enter');
            shouldBeOpen('Item 1');
        });

        it('ArrowDown moves focus to next trigger', () => {
            cy.findByRole('button', {name: 'Item 1'}).focus();
            cy.realPress('ArrowDown');
            // Item 2 is disabled, so it should be skipped
            cy.findByRole('button', {name: 'Item 3'}).should('be.focused');
        });

        it('ArrowUp moves focus to previous trigger', () => {
            cy.findByRole('button', {name: 'Item 3'}).focus();
            cy.realPress('ArrowUp');
            // Item 2 is disabled, so it should be skipped
            cy.findByRole('button', {name: 'Item 1'}).should('be.focused');
        });

        it('ArrowDown wraps from last to first trigger', () => {
            cy.findByRole('button', {name: 'Styled Item'}).focus();
            cy.realPress('ArrowDown');
            cy.findByRole('button', {name: 'Item 1'}).should('be.focused');
        });

        it('ArrowUp wraps from first to last trigger', () => {
            cy.findByRole('button', {name: 'Item 1'}).focus();
            cy.realPress('ArrowUp');
            cy.findByRole('button', {name: 'Styled Item'}).should('be.focused');
        });

        it('Home moves focus to first trigger', () => {
            cy.findByRole('button', {name: 'Styled Item'}).focus();
            cy.realPress('Home');
            cy.findByRole('button', {name: 'Item 1'}).should('be.focused');
        });

        it('End moves focus to last trigger', () => {
            cy.findByRole('button', {name: 'Item 1'}).focus();
            cy.realPress('End');
            cy.findByRole('button', {name: 'Styled Item'}).should('be.focused');
        });
    });

    // ── 4. Pointer Interaction ──────────────────────────────

    describe('pointer interaction', () => {
        it('click toggles item open', () => {
            shouldBeClosed('Item 1');
            cy.findByRole('button', {name: 'Item 1'}).click();
            shouldBeOpen('Item 1');
            contentShouldBeVisible('Item 1');
        });

        it('click another item closes current (single mode)', () => {
            cy.findByRole('button', {name: 'Item 1'}).click();
            shouldBeOpen('Item 1');
            cy.findByRole('button', {name: 'Item 3'}).click();
            shouldBeClosed('Item 1');
            shouldBeOpen('Item 3');
        });

        it('click disabled item does nothing', () => {
            cy.findByRole('button', {name: 'Item 2'}).click({force: true});
            shouldBeClosed('Item 2');
        });
    });

    // ── 5. Single Type ──────────────────────────────────────

    describe('single type', () => {
        it('only one item open at a time', () => {
            cy.findByRole('button', {name: 'Item 1'}).click();
            shouldBeOpen('Item 1');
            cy.findByRole('button', {name: 'Item 3'}).click();
            shouldBeClosed('Item 1');
            shouldBeOpen('Item 3');
        });

        it('collapsible=false: cannot close last open item by clicking', () => {
            // Default is collapsible=false
            cy.findByRole('button', {name: 'Item 1'}).click();
            shouldBeOpen('Item 1');
            cy.findByRole('button', {name: 'Item 1'}).click();
            // Should still be open
            shouldBeOpen('Item 1');
        });

        it('collapsible=true: can close last open item', () => {
            cy.findByLabelText('collapsible').click();
            cy.findByRole('button', {name: 'Item 1'}).click();
            shouldBeOpen('Item 1');
            cy.findByRole('button', {name: 'Item 1'}).click();
            shouldBeClosed('Item 1');
        });

        it('Space toggles open item when collapsible=true', () => {
            cy.findByLabelText('collapsible').click();
            cy.findByRole('button', {name: 'Item 1'}).focus();
            cy.realPress('Space');
            shouldBeOpen('Item 1');
            cy.realPress('Space');
            shouldBeClosed('Item 1');
        });
    });

    // ── 6. Multiple Type ────────────────────────────────────

    describe('multiple type', () => {
        beforeEach(() => {
            cy.findByLabelText('multiple').click();
        });

        it('multiple items can be open simultaneously', () => {
            cy.findByRole('button', {name: 'Item 1'}).click();
            shouldBeOpen('Item 1');
            cy.findByRole('button', {name: 'Item 3'}).click();
            shouldBeOpen('Item 1');
            shouldBeOpen('Item 3');
        });

        it('clicking open item closes it', () => {
            cy.findByRole('button', {name: 'Item 1'}).click();
            shouldBeOpen('Item 1');
            cy.findByRole('button', {name: 'Item 1'}).click();
            shouldBeClosed('Item 1');
        });

        it('keyboard toggles work independently', () => {
            cy.findByRole('button', {name: 'Item 1'}).focus();
            cy.realPress('Enter');
            shouldBeOpen('Item 1');
            cy.realPress('ArrowDown');
            // Skips disabled Item 2
            cy.findByRole('button', {name: 'Item 3'}).should('be.focused');
            cy.realPress('Enter');
            shouldBeOpen('Item 1');
            shouldBeOpen('Item 3');
        });
    });

    // ── 7. Attribute Forwarding (styles) ─────────────────────

    describe('attribute forwarding (styles)', () => {
        beforeEach(() => {
            // Switch to multiple mode so we can open styled item without closing others
            cy.findByLabelText('multiple').click();
            cy.findByRole('button', {name: 'Styled Item'}).click();
        });

        it('internal CSS variables are set for animation support', () => {
            cy.findByTestId('styled-content').then(($el) => {
                const heightVar = getComputedStyle($el[0]).getPropertyValue(
                    cssVar('accordion-content-height')
                );
                const widthVar = getComputedStyle($el[0]).getPropertyValue(
                    cssVar('accordion-content-width')
                );
                expect(heightVar.trim()).to.match(/^\d+(\.\d+)?px$/);
                expect(widthVar.trim()).to.match(/^\d+(\.\d+)?px$/);
            });
        });

        it('non-conflicting user styles merge with internal styles', () => {
            cy.findByTestId('styled-content').should(
                'have.css',
                'background-color',
                'rgb(255, 99, 71)'
            );
        });

        it('user style: directive overrides internal CSS var alias', () => {
            cy.findByTestId('styled-content').then(($el) => {
                const style = getComputedStyle($el[0]);

                // User set the accordion-content-height CSS var to "999px" which
                // overrides the internal alias (matching React's overridable pattern)
                const heightVar = style.getPropertyValue(cssVar('accordion-content-height'));
                expect(heightVar.trim()).to.equal('999px');

                // Width was NOT overridden by the user, so the internal alias resolves
                const widthVar = style.getPropertyValue(cssVar('accordion-content-width'));
                expect(widthVar.trim()).to.match(/^\d+(\.\d+)?px$/);
            });
        });
    });

    // ── 8. Disabled ─────────────────────────────────────────

    describe('disabled', () => {
        it('disabled item cannot be toggled by click', () => {
            cy.findByRole('button', {name: 'Item 2'}).click({force: true});
            shouldBeClosed('Item 2');
        });

        it('arrow keys skip disabled items', () => {
            cy.findByRole('button', {name: 'Item 1'}).focus();
            cy.realPress('ArrowDown');
            // Should skip Item 2 (disabled) and land on Item 3
            cy.findByRole('button', {name: 'Item 3'}).should('be.focused');
        });

        it('arrow keys skip disabled items going up', () => {
            cy.findByRole('button', {name: 'Item 3'}).focus();
            cy.realPress('ArrowUp');
            // Should skip Item 2 (disabled) and land on Item 1
            cy.findByRole('button', {name: 'Item 1'}).should('be.focused');
        });
    });

    // ── 9. Controlled Mode ────────────────────────────────────

    describe('controlled mode', () => {
        function ctrlShouldBeOpen(itemName) {
            cy.findByRole('button', {name: itemName}).should('have.attr', 'data-state', 'open');
        }

        function ctrlShouldBeClosed(itemName) {
            cy.findByRole('button', {name: itemName}).should('have.attr', 'data-state', 'closed');
        }

        it('external control opens an accordion item', () => {
            // accordion-msc-1
            ctrlShouldBeClosed('Ctrl Item 1');
            cy.findByTestId('controlled-open-item-1').click();
            ctrlShouldBeOpen('Ctrl Item 1');
            cy.findByTestId('controlled-value').should('have.text', 'ctrl-item-1');
        });

        it('external control closes an accordion item', () => {
            // accordion-msc-1
            cy.findByTestId('controlled-open-item-1').click();
            ctrlShouldBeOpen('Ctrl Item 1');
            cy.findByTestId('controlled-close-all').click();
            ctrlShouldBeClosed('Ctrl Item 1');
            cy.findByTestId('controlled-value').should('have.text', '');
        });

        it('clicking accordion trigger updates external state', () => {
            // accordion-msc-1
            cy.findByTestId('controlled-value').should('have.text', '');
            cy.findByRole('button', {name: 'Ctrl Item 1'}).click();
            cy.findByTestId('controlled-value').should('have.text', 'ctrl-item-1');
        });

        it('controlled value prop determines which item is open', () => {
            // accordion-msc-1
            cy.findByTestId('controlled-open-item-1').click();
            ctrlShouldBeOpen('Ctrl Item 1');
            ctrlShouldBeClosed('Ctrl Item 2');

            cy.findByTestId('controlled-open-item-2').click();
            ctrlShouldBeClosed('Ctrl Item 1');
            ctrlShouldBeOpen('Ctrl Item 2');
            cy.findByTestId('controlled-value').should('have.text', 'ctrl-item-2');
        });
    });

    // ── Attribute Forwarding ────────────────────────────────

    describe('attribute forwarding', () => {
        it('Item forwards className/class and custom data attributes', () => {
            cy.findByTestId('item-1')
                .should('have.class', 'accordion-item')
                .and('have.attr', 'data-custom', 'accordion-item-custom');
        });

        it('Header forwards className/class and custom data attributes', () => {
            cy.findByTestId('item-1')
                .find('[data-custom="accordion-header-custom"]')
                .should('exist')
                .and('have.class', 'accordion-header');
        });

        it('Trigger forwards className/class and custom data attributes', () => {
            cy.findByRole('button', {name: 'Item 1'})
                .should('have.class', 'accordion-trigger')
                .and('have.attr', 'data-custom', 'accordion-trigger-custom');
        });

        it('Content forwards className/class and custom data attributes when open', () => {
            cy.findByRole('button', {name: 'Item 1'}).click();
            cy.findByText('Content 1')
                .closest('[role="region"]')
                .should('have.class', 'accordion-content')
                .and('have.attr', 'data-custom', 'accordion-content-custom');
        });
    });
});
