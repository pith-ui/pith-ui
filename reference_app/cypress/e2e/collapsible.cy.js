describe('Collapsible', () => {
    // ── Helpers ──────────────────────────────────────────────

    function shouldBeOpen() {
        cy.findByText('Collapsible content.').should('be.visible');
    }

    function shouldBeClosed() {
        cy.findByText('Collapsible content.').should('not.exist');
    }

    beforeEach(() => {
        cy.visit('/collapsible');
    });

    // ── 1. Accessibility Semantics ──────────────────────────

    describe('accessibility', () => {
        it('Trigger is a button', () => {
            cy.findByRole('button', {name: 'toggle'}).should('exist');
        });

        it('Trigger has aria-expanded reflecting open state', () => {
            cy.findByRole('button', {name: 'toggle'}).should('have.attr', 'aria-expanded', 'false');
            cy.findByRole('button', {name: 'toggle'}).click();
            shouldBeOpen();
            cy.findByRole('button', {name: 'toggle'}).should('have.attr', 'aria-expanded', 'true');
        });

        it('Trigger has aria-controls referencing Content id', () => {
            cy.findByRole('button', {name: 'toggle'}).click();
            shouldBeOpen();
            cy.findByRole('button', {name: 'toggle'})
                .invoke('attr', 'aria-controls')
                .then((controlsId) => {
                    cy.findByText('Collapsible content.').parent().should('have.attr', 'id', controlsId);
                });
        });

        it('Content is hidden when closed and visible when open', () => {
            shouldBeClosed();
            cy.findByRole('button', {name: 'toggle'}).click();
            shouldBeOpen();
        });
    });

    // ── 2. Data Attributes ──────────────────────────────────

    describe('data attributes', () => {
        it('Root has data-state reflecting open state', () => {
            cy.findByTestId('collapsible-root').should('have.attr', 'data-state', 'closed');
            cy.findByRole('button', {name: 'toggle'}).click();
            shouldBeOpen();
            cy.findByTestId('collapsible-root').should('have.attr', 'data-state', 'open');
            cy.findByRole('button', {name: 'toggle'}).click();
            shouldBeClosed();
            cy.findByTestId('collapsible-root').should('have.attr', 'data-state', 'closed');
        });

        it('Trigger has data-state reflecting open state', () => {
            cy.findByRole('button', {name: 'toggle'}).should('have.attr', 'data-state', 'closed');
            cy.findByRole('button', {name: 'toggle'}).click();
            shouldBeOpen();
            cy.findByRole('button', {name: 'toggle'}).should('have.attr', 'data-state', 'open');
            cy.findByRole('button', {name: 'toggle'}).click();
            shouldBeClosed();
            cy.findByRole('button', {name: 'toggle'}).should('have.attr', 'data-state', 'closed');
        });

        it('Content has data-state reflecting open state', () => {
            cy.findByRole('button', {name: 'toggle'}).click();
            shouldBeOpen();
            cy.findByText('Collapsible content.').parent().should('have.attr', 'data-state', 'open');
        });

        it('Root has data-disabled when disabled', () => {
            cy.findByLabelText('disabled').click();
            cy.findByTestId('collapsible-root').should('have.attr', 'data-disabled');
        });

        it('Trigger has data-disabled when disabled', () => {
            cy.findByLabelText('disabled').click();
            cy.findByRole('button', {name: 'toggle'}).should('have.attr', 'data-disabled');
        });
    });

    // ── 3. Keyboard Navigation ──────────────────────────────

    describe('keyboard navigation', () => {
        it('Space toggles open and closed', () => {
            cy.findByRole('button', {name: 'toggle'}).focus();
            cy.realPress('Space');
            shouldBeOpen();
            cy.realPress('Space');
            shouldBeClosed();
        });

        it('Enter toggles open and closed', () => {
            cy.findByRole('button', {name: 'toggle'}).focus();
            cy.realPress('Enter');
            shouldBeOpen();
            cy.realPress('Enter');
            shouldBeClosed();
        });
    });

    // ── 4. Pointer Interaction ──────────────────────────────

    describe('pointer interaction', () => {
        it('click toggles open and closed', () => {
            cy.findByRole('button', {name: 'toggle'}).click();
            shouldBeOpen();
            cy.findByRole('button', {name: 'toggle'}).click();
            shouldBeClosed();
        });
    });

    // ── 5. Disabled Variant ─────────────────────────────────

    describe('disabled variant', () => {
        beforeEach(() => {
            cy.findByLabelText('disabled').click();
        });

        it('click does not toggle when disabled', () => {
            cy.findByRole('button', {name: 'toggle'}).click({force: true});
            shouldBeClosed();
        });

        it('Trigger has disabled attribute', () => {
            cy.findByRole('button', {name: 'toggle'}).should('be.disabled');
        });

        it('Trigger has data-disabled attribute', () => {
            cy.findByRole('button', {name: 'toggle'}).should('have.attr', 'data-disabled');
        });

        it('Root has data-disabled attribute', () => {
            cy.findByTestId('collapsible-root').should('have.attr', 'data-disabled');
        });
    });

    // ── 6. Internal Styles (CSS custom properties) ──────────

    describe('internal styles', () => {
        it('collapsible-content-height CSS var matches actual height', () => {
            cy.findByTestId('styled-collapsible-content').then(($el) => {
                const height = $el[0].getBoundingClientRect().height;
                const varValue = getComputedStyle($el[0]).getPropertyValue(
                    cssVar('collapsible-content-height')
                );
                expect(varValue.trim()).to.equal(`${height}px`);
            });
        });

        it('collapsible-content-width CSS var matches actual width', () => {
            cy.findByTestId('styled-collapsible-content').then(($el) => {
                const width = $el[0].getBoundingClientRect().width;
                const varValue = getComputedStyle($el[0]).getPropertyValue(
                    cssVar('collapsible-content-width')
                );
                expect(varValue.trim()).to.equal(`${width}px`);
            });
        });

        it('user style is preserved alongside internal CSS variables', () => {
            cy.findByTestId('styled-collapsible-content').should(
                'have.css',
                'background-color',
                'rgb(255, 99, 71)'
            );
        });
    });

    // ── 7. Controlled Variant ───────────────────────────────

    describe('controlled variant', () => {
        it('external open checkbox controls collapsible state', () => {
            shouldBeClosed();
            cy.findByLabelText('open').click();
            shouldBeOpen();
            cy.findByTestId('collapsible-root').should('have.attr', 'data-state', 'open');
            cy.findByLabelText('open').click();
            shouldBeClosed();
            cy.findByTestId('collapsible-root').should('have.attr', 'data-state', 'closed');
        });

        it('Trigger still toggles when controlled', () => {
            shouldBeClosed();
            cy.findByRole('button', {name: 'toggle'}).click();
            shouldBeOpen();
            cy.findByLabelText('open').should('be.checked');
            cy.findByRole('button', {name: 'toggle'}).click();
            shouldBeClosed();
            cy.findByLabelText('open').should('not.be.checked');
        });
    });

    // ── Axe Accessibility Audit ─────────────────────────────

    describe('axe audit', () => {
        it('no violations in collapsed state', () => {
            cy.checkComponentA11y();
        });

        it('no violations when expanded', () => {
            cy.findByRole('button', {name: 'toggle'}).click();
            shouldBeOpen();
            cy.checkComponentA11y();
        });
    });
});
