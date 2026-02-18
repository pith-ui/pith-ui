describe('Tabs', () => {
    // ── Helpers ──────────────────────────────────────────────

    function shouldShowPanel(text) {
        cy.findByRole('tabpanel').should('contain.text', text);
    }

    function tab1ShouldBeActive() {
        cy.findByRole('tab', {name: 'Tab 1'}).should('have.attr', 'aria-selected', 'true');
        cy.findByRole('tab', {name: 'Tab 1'}).should('have.attr', 'data-state', 'active');
    }

    function tab3ShouldBeActive() {
        cy.findByRole('tab', {name: 'Tab 3'}).should('have.attr', 'aria-selected', 'true');
        cy.findByRole('tab', {name: 'Tab 3'}).should('have.attr', 'data-state', 'active');
    }

    beforeEach(() => {
        cy.visit('/tabs');
    });

    // ── 1. Accessibility Semantics ──────────────────────────

    describe('accessibility', () => {
        it('TabList has role="tablist"', () => {
            cy.findByRole('tablist').should('exist');
        });

        it('Triggers have role="tab"', () => {
            cy.findAllByRole('tab').should('have.length', 3);
        });

        it('Active content has role="tabpanel"', () => {
            cy.findByRole('tabpanel').should('exist');
        });

        it('Active tab has aria-selected="true"', () => {
            cy.findByRole('tab', {name: 'Tab 1'}).should('have.attr', 'aria-selected', 'true');
        });

        it('Inactive tabs have aria-selected="false"', () => {
            cy.findByRole('tab', {name: 'Tab 3'}).should('have.attr', 'aria-selected', 'false');
        });

        it('Tab has aria-controls pointing to its panel', () => {
            cy.findByRole('tab', {name: 'Tab 1'})
                .invoke('attr', 'aria-controls')
                .then((controlsId) => {
                    cy.findByRole('tabpanel').should('have.attr', 'id', controlsId);
                });
        });

        it('Panel has aria-labelledby pointing to its tab', () => {
            cy.findByRole('tabpanel')
                .invoke('attr', 'aria-labelledby')
                .then((labelId) => {
                    cy.findByRole('tab', {name: 'Tab 1'}).should('have.attr', 'id', labelId);
                });
        });

        it('TabList has aria-orientation', () => {
            cy.findByRole('tablist').should('have.attr', 'aria-orientation', 'horizontal');
        });
    });

    // ── 2. Data Attributes ──────────────────────────────────

    describe('data attributes', () => {
        it('Active tab has data-state="active"', () => {
            cy.findByRole('tab', {name: 'Tab 1'}).should('have.attr', 'data-state', 'active');
        });

        it('Inactive tab has data-state="inactive"', () => {
            cy.findByRole('tab', {name: 'Tab 3'}).should('have.attr', 'data-state', 'inactive');
        });

        it('Active content has data-state="active"', () => {
            cy.findByRole('tabpanel').should('have.attr', 'data-state', 'active');
        });

        it('data-state transitions when switching tabs', () => {
            cy.findByRole('tab', {name: 'Tab 1'}).should('have.attr', 'data-state', 'active');
            cy.findByRole('tab', {name: 'Tab 3'}).should('have.attr', 'data-state', 'inactive');

            cy.findByRole('tab', {name: 'Tab 3'}).click();

            cy.findByRole('tab', {name: 'Tab 3'}).should('have.attr', 'data-state', 'active');
            cy.findByRole('tab', {name: 'Tab 1'}).should('have.attr', 'data-state', 'inactive');
        });

        it('data-orientation on tablist, trigger, and content', () => {
            cy.findByRole('tablist').should('have.attr', 'data-orientation', 'horizontal');
            cy.findByRole('tab', {name: 'Tab 1'}).should('have.attr', 'data-orientation', 'horizontal');
            cy.findByRole('tabpanel').should('have.attr', 'data-orientation', 'horizontal');
        });

        it('Disabled tab has data-disabled', () => {
            cy.findByRole('tab', {name: 'Tab 2'}).should('have.attr', 'data-disabled');
        });

        it('Non-disabled tabs do not have data-disabled', () => {
            cy.findByRole('tab', {name: 'Tab 1'}).should('not.have.attr', 'data-disabled');
            cy.findByRole('tab', {name: 'Tab 3'}).should('not.have.attr', 'data-disabled');
        });
    });

    // ── 3. Keyboard Navigation ──────────────────────────────

    describe('keyboard navigation', () => {
        it('ArrowRight moves focus to next tab (horizontal)', () => {
            cy.findByRole('tab', {name: 'Tab 1'}).focus();
            cy.realPress('ArrowRight');
            // Tab 2 is disabled, so it should skip to Tab 3
            cy.findByRole('tab', {name: 'Tab 3'}).should('be.focused');
        });

        it('ArrowLeft moves focus to previous tab (horizontal)', () => {
            cy.findByRole('tab', {name: 'Tab 3'}).focus();
            cy.realPress('ArrowLeft');
            // Tab 2 is disabled, so it should skip to Tab 1
            cy.findByRole('tab', {name: 'Tab 1'}).should('be.focused');
        });

        it('ArrowRight wraps around to first tab', () => {
            cy.findByRole('tab', {name: 'Tab 3'}).focus();
            cy.realPress('ArrowRight');
            cy.findByRole('tab', {name: 'Tab 1'}).should('be.focused');
        });

        it('ArrowLeft wraps around to last tab', () => {
            cy.findByRole('tab', {name: 'Tab 1'}).focus();
            cy.realPress('ArrowLeft');
            cy.findByRole('tab', {name: 'Tab 3'}).should('be.focused');
        });

        it('Home moves focus to first tab', () => {
            cy.findByRole('tab', {name: 'Tab 3'}).focus();
            cy.realPress('Home');
            cy.findByRole('tab', {name: 'Tab 1'}).should('be.focused');
        });

        it('End moves focus to last tab', () => {
            cy.findByRole('tab', {name: 'Tab 1'}).focus();
            cy.realPress('End');
            cy.findByRole('tab', {name: 'Tab 3'}).should('be.focused');
        });

        it('Tab key moves focus from trigger into active panel content', () => {
            cy.findByRole('tab', {name: 'Tab 1'}).focus();
            cy.realPress('Tab');
            cy.findByRole('tabpanel').should('be.focused');
        });

        it('Arrow keys in automatic mode also activate tab', () => {
            cy.findByRole('tab', {name: 'Tab 1'}).focus();
            tab1ShouldBeActive();
            cy.realPress('ArrowRight');
            // Tab 2 is disabled, skip to Tab 3
            cy.findByRole('tab', {name: 'Tab 3'}).should('be.focused');
            tab3ShouldBeActive();
            shouldShowPanel('Content 3');
        });
    });

    // ── 4. Pointer Interaction ──────────────────────────────

    describe('pointer interaction', () => {
        it('Click activates tab and shows corresponding panel', () => {
            tab1ShouldBeActive();
            shouldShowPanel('Content 1');

            cy.findByRole('tab', {name: 'Tab 3'}).click();
            tab3ShouldBeActive();
            shouldShowPanel('Content 3');
        });

        it('Click on disabled tab does not activate it', () => {
            tab1ShouldBeActive();
            cy.findByRole('tab', {name: 'Tab 2'}).click({force: true});
            tab1ShouldBeActive();
            shouldShowPanel('Content 1');
        });
    });

    // ── 5. Orientation ──────────────────────────────────────

    describe('orientation', () => {
        describe('vertical', () => {
            beforeEach(() => {
                cy.findByLabelText('vertical').click();
            });

            it('data-orientation updates to vertical', () => {
                cy.findByRole('tablist').should('have.attr', 'data-orientation', 'vertical');
                cy.findByRole('tablist').should('have.attr', 'aria-orientation', 'vertical');
                cy.findByRole('tab', {name: 'Tab 1'}).should('have.attr', 'data-orientation', 'vertical');
                cy.findByRole('tabpanel').should('have.attr', 'data-orientation', 'vertical');
            });

            it('ArrowDown moves to next tab', () => {
                cy.findByRole('tab', {name: 'Tab 1'}).focus();
                cy.realPress('ArrowDown');
                // Tab 2 disabled, skip to Tab 3
                cy.findByRole('tab', {name: 'Tab 3'}).should('be.focused');
            });

            it('ArrowUp moves to previous tab', () => {
                cy.findByRole('tab', {name: 'Tab 3'}).focus();
                cy.realPress('ArrowUp');
                // Tab 2 disabled, skip to Tab 1
                cy.findByRole('tab', {name: 'Tab 1'}).should('be.focused');
            });

            it('ArrowRight does not navigate in vertical mode', () => {
                cy.findByRole('tab', {name: 'Tab 1'}).focus();
                cy.realPress('ArrowRight');
                cy.findByRole('tab', {name: 'Tab 1'}).should('be.focused');
            });

            it('ArrowLeft does not navigate in vertical mode', () => {
                cy.findByRole('tab', {name: 'Tab 1'}).focus();
                cy.realPress('ArrowLeft');
                cy.findByRole('tab', {name: 'Tab 1'}).should('be.focused');
            });

            it('Home and End still work in vertical mode', () => {
                cy.findByRole('tab', {name: 'Tab 1'}).focus();
                cy.realPress('End');
                cy.findByRole('tab', {name: 'Tab 3'}).should('be.focused');
                cy.realPress('Home');
                cy.findByRole('tab', {name: 'Tab 1'}).should('be.focused');
            });
        });

        describe('horizontal (default)', () => {
            it('ArrowDown does not navigate in horizontal mode', () => {
                cy.findByRole('tab', {name: 'Tab 1'}).focus();
                cy.realPress('ArrowDown');
                cy.findByRole('tab', {name: 'Tab 1'}).should('be.focused');
            });

            it('ArrowUp does not navigate in horizontal mode', () => {
                cy.findByRole('tab', {name: 'Tab 1'}).focus();
                cy.realPress('ArrowUp');
                cy.findByRole('tab', {name: 'Tab 1'}).should('be.focused');
            });
        });
    });

    // ── 6. Activation Mode ──────────────────────────────────

    describe('activation mode', () => {
        describe('manual', () => {
            beforeEach(() => {
                cy.findByLabelText('manual').click();
            });

            it('ArrowRight moves focus but does not activate tab', () => {
                cy.findByRole('tab', {name: 'Tab 1'}).focus();
                tab1ShouldBeActive();
                cy.realPress('ArrowRight');
                // Focus moved to Tab 3 (Tab 2 disabled)
                cy.findByRole('tab', {name: 'Tab 3'}).should('be.focused');
                // But Tab 1 should still be active
                tab1ShouldBeActive();
                shouldShowPanel('Content 1');
            });

            it('Space activates the focused tab', () => {
                cy.findByRole('tab', {name: 'Tab 1'}).focus();
                cy.realPress('ArrowRight');
                cy.findByRole('tab', {name: 'Tab 3'}).should('be.focused');
                tab1ShouldBeActive();
                cy.realPress('Space');
                tab3ShouldBeActive();
                shouldShowPanel('Content 3');
            });

            it('Enter activates the focused tab', () => {
                cy.findByRole('tab', {name: 'Tab 1'}).focus();
                cy.realPress('ArrowRight');
                cy.findByRole('tab', {name: 'Tab 3'}).should('be.focused');
                tab1ShouldBeActive();
                cy.realPress('Enter');
                tab3ShouldBeActive();
                shouldShowPanel('Content 3');
            });
        });

        describe('automatic (default)', () => {
            it('focus immediately activates tab', () => {
                cy.findByRole('tab', {name: 'Tab 1'}).focus();
                cy.realPress('ArrowRight');
                cy.findByRole('tab', {name: 'Tab 3'}).should('be.focused');
                tab3ShouldBeActive();
                shouldShowPanel('Content 3');
            });
        });
    });

    // ── 7. Disabled ─────────────────────────────────────────

    describe('disabled', () => {
        it('Disabled tab is skipped by arrow key navigation', () => {
            cy.findByRole('tab', {name: 'Tab 1'}).focus();
            cy.realPress('ArrowRight');
            // Should skip Tab 2 (disabled) and go to Tab 3
            cy.findByRole('tab', {name: 'Tab 3'}).should('be.focused');
        });

        it('Disabled tab has data-disabled attribute', () => {
            cy.findByRole('tab', {name: 'Tab 2'}).should('have.attr', 'data-disabled');
        });

        it('Disabled tab has disabled attribute', () => {
            cy.findByRole('tab', {name: 'Tab 2'}).should('have.attr', 'disabled');
        });

        it('Disabled tab has aria-selected="false"', () => {
            cy.findByRole('tab', {name: 'Tab 2'}).should('have.attr', 'aria-selected', 'false');
        });
    });
});
