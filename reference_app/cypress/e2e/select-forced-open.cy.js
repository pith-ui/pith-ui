describe('Select (forced open)', () => {
    beforeEach(() => {
        cy.visit('/select/forced-open');
    });

    // ── Forced Open, No Value, Item-Aligned ─────────────────

    describe('forced open no value (item-aligned)', () => {
        it('content is visible on page load without interaction', () => {
            cy.get('[data-testid="forced-novalue-content"]').should('be.visible');
        });

        it('trigger has data-state="open"', () => {
            cy.get('[data-testid="forced-novalue-trigger"]').should('have.attr', 'data-state', 'open');
        });

        it('trigger has aria-expanded="true"', () => {
            cy.get('[data-testid="forced-novalue-trigger"]').should('have.attr', 'aria-expanded', 'true');
        });

        it('content has role="listbox"', () => {
            cy.get('[data-testid="forced-novalue-content"]').should('have.attr', 'role', 'listbox');
        });

        it('items are visible and have role="option"', () => {
            cy.get('[data-testid="forced-novalue-content"]').find('[role="option"]').should('have.length', 3);
        });

        it('no item has data-state="checked"', () => {
            cy.get('[data-testid="forced-novalue-content"]')
                .find('[role="option"]')
                .filter('[data-state="checked"]')
                .should('have.length', 0);
        });

        it('trigger shows placeholder text', () => {
            cy.get('[data-testid="forced-novalue-trigger"]').should('contain.text', 'Pick an option');
        });

        it('content does not have data-side (item-aligned)', () => {
            cy.get('[data-testid="forced-novalue-content"]').should('not.have.attr', 'data-side');
        });
    });

    // ── Forced Open, Popper mode ─────────────────────────────

    describe('forced open (popper)', () => {
        function getForcedTrigger() {
            return cy.get('[data-testid="forced-trigger"]');
        }

        it('content is visible on page load without interaction', () => {
            cy.get('[data-testid="forced-content"]').should('be.visible');
        });

        it('trigger has data-state="open"', () => {
            getForcedTrigger().should('have.attr', 'data-state', 'open');
        });

        it('trigger has aria-expanded="true"', () => {
            getForcedTrigger().should('have.attr', 'aria-expanded', 'true');
        });

        it('content has role="listbox"', () => {
            cy.get('[data-testid="forced-content"]').should('have.attr', 'role', 'listbox');
        });

        it('items are visible and have role="option"', () => {
            cy.get('[data-testid="forced-content"]').find('[role="option"]').should('have.length', 3);
        });

        it('selected item has data-state="checked"', () => {
            cy.get('[data-testid="forced-content"]')
                .find('[role="option"]')
                .filter('[data-state="checked"]')
                .should('have.length', 1)
                .and('contain.text', 'Banana');
        });

        it('trigger shows default value text', () => {
            getForcedTrigger().should('contain.text', 'Banana');
        });

        it('content has data-side (popper mode)', () => {
            cy.get('[data-testid="forced-content"]').should('have.attr', 'data-side');
        });
    });

    // ── Forced Open, Item-Aligned mode ───────────────────────

    describe('forced open (item-aligned)', () => {
        function getForcedAlignedTrigger() {
            return cy.get('[data-testid="forced-aligned-trigger"]');
        }

        it('content is visible on page load without interaction', () => {
            cy.get('[data-testid="forced-aligned-content"]').should('be.visible');
        });

        it('trigger has data-state="open"', () => {
            getForcedAlignedTrigger().should('have.attr', 'data-state', 'open');
        });

        it('trigger has aria-expanded="true"', () => {
            getForcedAlignedTrigger().should('have.attr', 'aria-expanded', 'true');
        });

        it('content has role="listbox"', () => {
            cy.get('[data-testid="forced-aligned-content"]').should('have.attr', 'role', 'listbox');
        });

        it('items are visible and have role="option"', () => {
            cy.get('[data-testid="forced-aligned-content"]').find('[role="option"]').should('have.length', 3);
        });

        it('selected item has data-state="checked"', () => {
            cy.get('[data-testid="forced-aligned-content"]')
                .find('[role="option"]')
                .filter('[data-state="checked"]')
                .should('have.length', 1)
                .and('contain.text', 'Banana');
        });

        it('trigger shows default value text', () => {
            getForcedAlignedTrigger().should('contain.text', 'Banana');
        });

        it('content does not have data-side (item-aligned, not popper)', () => {
            cy.get('[data-testid="forced-aligned-content"]').should('not.have.attr', 'data-side');
        });
    });

    // ── Attribute Forwarding (styles) ─────────────────────

    describe('attribute forwarding (styles)', () => {
        it('SelectValue has pointer-events: none', () => {
            // The SelectValue span is inside the trigger
            cy.get('[data-testid="forced-trigger"]')
                .find('span[style]')
                .first()
                .should('have.css', 'pointer-events', 'none');
        });

        it('SelectContent (popper) has internal layout styles', () => {
            cy.get('[data-testid="forced-content"]').should('have.css', 'display', 'flex');
            cy.get('[data-testid="forced-content"]').should(
                'have.css',
                'flex-direction',
                'column'
            );
            cy.get('[data-testid="forced-content"]').should(
                'have.css',
                'outline-style',
                'none'
            );
            cy.get('[data-testid="forced-content"]').should(
                'have.css',
                'box-sizing',
                'border-box'
            );
        });

        it('SelectContent (popper) non-conflicting user styles merge with internal styles', () => {
            cy.get('[data-testid="forced-content"]').should(
                'have.css',
                'background-color',
                'rgb(255, 99, 71)'
            );
        });

        it('SelectContent (popper) internal CSS variables are set alongside user styles', () => {
            cy.get('[data-testid="forced-content"]').then(($el) => {
                const style = getComputedStyle($el[0]);
                const availW = style.getPropertyValue('--radix-select-content-available-width');
                const availH = style.getPropertyValue('--radix-select-content-available-height');
                expect(availW.trim()).to.not.be.empty;
                expect(availH.trim()).to.not.be.empty;
            });
        });

        it('SelectContent (item-aligned) has box-sizing: border-box and outline: none', () => {
            cy.get('[data-testid="forced-aligned-content"]').should(
                'have.css',
                'box-sizing',
                'border-box'
            );
            cy.get('[data-testid="forced-aligned-content"]').should(
                'have.css',
                'outline-style',
                'none'
            );
        });
    });
});
