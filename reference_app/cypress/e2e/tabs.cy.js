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
            cy.findByTestId('uncontrolled-tabs-section').findAllByRole('tab').should('have.length', 3);
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

    // ── Internal Styles ─────────────────────────────────────

    describe('internal styles', () => {
        it('initially active tab content has animation-duration: 0s to prevent mount animation', () => {
            // TabsContent suppresses mount animation by setting animation-duration: 0s
            cy.findByText('Content 1').should('have.css', 'animation-duration', '0s');
        });

        it('newly activated tab content does not have animation-duration: 0s', () => {
            // Switch to Tab 3 — its content should animate normally
            cy.findByRole('tab', {name: 'Tab 3'}).click();
            cy.findByText('Content 3').then(($el) => {
                const inlineStyle = $el[0].style.cssText;
                expect(inlineStyle).to.not.contain('animation-duration');
            });
        });
    });

    // ── 8. Force Mount ─────────────────────────────────────

    describe('force mount', () => {
        it('both force-mounted panels are in the DOM', () => {
            cy.findByTestId('fm-content-1').should('exist');
            cy.findByTestId('fm-content-2').should('exist');
        });

        it('active force-mounted panel has data-state="active"', () => {
            cy.findByTestId('fm-content-1').should('have.attr', 'data-state', 'active');
        });

        it('inactive force-mounted panel has data-state="inactive"', () => {
            cy.findByTestId('fm-content-2').should('have.attr', 'data-state', 'inactive');
        });

        it('data-state toggles when switching force-mounted tabs', () => {
            cy.findByTestId('fm-content-1').should('have.attr', 'data-state', 'active');
            cy.findByTestId('fm-content-2').should('have.attr', 'data-state', 'inactive');

            cy.findByTestId('fm-trigger-2').click();

            cy.findByTestId('fm-content-2').should('have.attr', 'data-state', 'active');
            cy.findByTestId('fm-content-1').should('have.attr', 'data-state', 'inactive');
        });

        it('inactive force-mounted panel has role="tabpanel"', () => {
            // Even inactive force-mounted panels maintain their tabpanel role
            cy.findByTestId('fm-content-2').should('have.attr', 'role', 'tabpanel');
        });
    });

    // ── 9. Controlled Mode ──────────────────────────────────

    describe('controlled mode', () => {
        it('external control changes active tab', () => {
            // tabs-msc-1
            // Initially Tab 1 is active
            cy.findByTestId('controlled-tab-trigger-1').should('have.attr', 'data-state', 'active');
            cy.findByTestId('controlled-tab-content-1').should('exist');

            // Click external button to switch to Tab 3
            cy.findByTestId('controlled-select-tab3').click();
            cy.findByTestId('controlled-tab-trigger-3').should('have.attr', 'data-state', 'active');
            cy.findByTestId('controlled-tab-trigger-1').should('have.attr', 'data-state', 'inactive');
            cy.findByTestId('controlled-tab-content-3').should('contain.text', 'Controlled Content 3');

            // Click external button to switch to Tab 2
            cy.findByTestId('controlled-select-tab2').click();
            cy.findByTestId('controlled-tab-trigger-2').should('have.attr', 'data-state', 'active');
            cy.findByTestId('controlled-tab-trigger-3').should('have.attr', 'data-state', 'inactive');
            cy.findByTestId('controlled-tab-content-2').should('contain.text', 'Controlled Content 2');
        });

        it('clicking tab trigger updates external state', () => {
            // tabs-msc-1
            // Initially the external value display shows ctab1
            cy.findByTestId('controlled-value-display').should('have.text', 'ctab1');

            // Click controlled Tab 3 trigger
            cy.findByTestId('controlled-tab-trigger-3').click();
            cy.findByTestId('controlled-value-display').should('have.text', 'ctab3');

            // Click controlled Tab 2 trigger
            cy.findByTestId('controlled-tab-trigger-2').click();
            cy.findByTestId('controlled-value-display').should('have.text', 'ctab2');
        });

        it('consumer-driven value prop controls which tab is active', () => {
            // tabs-msc-1
            // Use external button to set Tab 2
            cy.findByTestId('controlled-select-tab2').click();
            cy.findByTestId('controlled-tab-trigger-2').should('have.attr', 'aria-selected', 'true');
            cy.findByTestId('controlled-tab-trigger-1').should('have.attr', 'aria-selected', 'false');
            cy.findByTestId('controlled-tab-trigger-3').should('have.attr', 'aria-selected', 'false');
            cy.findByTestId('controlled-tab-content-2').should('exist');

            // Switch back to Tab 1 via external button
            cy.findByTestId('controlled-select-tab1').click();
            cy.findByTestId('controlled-tab-trigger-1').should('have.attr', 'aria-selected', 'true');
            cy.findByTestId('controlled-tab-trigger-2').should('have.attr', 'aria-selected', 'false');
            cy.findByTestId('controlled-tab-content-1').should('exist');
        });
    });

    // ── 10. Attribute Forwarding ────────────────────────────

    describe('attribute forwarding', () => {
        it('Root forwards className/class and custom data attributes', () => {
            cy.findByTestId('uncontrolled-tabs-section')
                .find('[data-custom="tabs-root-custom"]')
                .should('exist')
                .and('have.class', 'tabs-root')
                .and('have.attr', 'data-orientation', 'horizontal');
        });

        it('List forwards className/class and custom data attributes', () => {
            cy.findByRole('tablist')
                .should('have.class', 'tabs-list')
                .and('have.attr', 'data-custom', 'tabs-list-custom')
                .and('have.attr', 'role', 'tablist');
        });

        it('Trigger forwards className/class and custom data attributes', () => {
            cy.findByRole('tab', {name: 'Tab 1'})
                .should('have.class', 'tabs-trigger')
                .and('have.attr', 'data-custom', 'tabs-trigger-custom')
                .and('have.attr', 'role', 'tab');
        });

        it('Content forwards className/class and custom data attributes', () => {
            cy.findByRole('tabpanel')
                .should('have.class', 'tabs-content')
                .and('have.attr', 'data-custom', 'tabs-content-custom')
                .and('have.attr', 'role', 'tabpanel');
        });
    });

    // ── Axe Accessibility Audit ─────────────────────────────

    describe('axe audit', () => {
        // Exclude aria-hidden-focus — the test fixture wraps extra tab sections
        // in aria-hidden="true" to prevent findByRole collisions, but those
        // sections still contain focusable tab triggers.
        const a11yOpts = {rules: {'aria-hidden-focus': {enabled: false}}};

        it('no violations with default tab selected', () => {
            cy.checkComponentA11y(null, a11yOpts);
        });

        it('no violations after switching tabs', () => {
            cy.findByRole('tab', {name: 'Tab 3'}).click();
            tab3ShouldBeActive();
            cy.checkComponentA11y(null, a11yOpts);
        });
    });
});
