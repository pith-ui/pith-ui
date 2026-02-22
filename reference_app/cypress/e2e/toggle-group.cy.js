describe('Toggle Group', () => {
    // ── Helpers ──────────────────────────────────────────────

    function getItem(name) {
        // In single mode items have role="radio", in multiple mode they're buttons
        return cy.findByText(name, {selector: '.toggle-group-item'});
    }

    function getGroup() {
        return cy.findByRole('group', {name: 'Options'});
    }

    function shouldBeOn(name) {
        getItem(name).should('have.attr', 'data-state', 'on');
    }

    function shouldBeOff(name) {
        getItem(name).should('have.attr', 'data-state', 'off');
    }

    beforeEach(() => {
        cy.visit('/toggle-group');
    });

    // ── 1. Accessibility Semantics ──────────────────────────

    describe('accessibility', () => {
        it('group has role="group"', () => {
            getGroup().should('exist');
        });

        it('items render as button elements', () => {
            cy.get('.toggle-group-item').should('have.length', 3);
            cy.get('.toggle-group-item').first().should('match', 'button');
        });

        it('group has accessible label', () => {
            getGroup().should('have.attr', 'aria-label', 'Options');
        });

        it('single mode items have role="radio" and aria-checked', () => {
            getItem('Item 1').should('have.attr', 'role', 'radio');
            getItem('Item 1').should('have.attr', 'aria-checked', 'false');
            getItem('Item 1').should('not.have.attr', 'aria-pressed');
        });

        it('single mode toggled item has aria-checked="true"', () => {
            getItem('Item 1').click();
            getItem('Item 1').should('have.attr', 'aria-checked', 'true');
        });

        it('multiple mode items have aria-pressed (not role="radio")', () => {
            cy.findByLabelText('multiple').click();
            getItem('Item 1').should('not.have.attr', 'role', 'radio');
            getItem('Item 1').should('have.attr', 'aria-pressed', 'false');
            getItem('Item 1').should('not.have.attr', 'aria-checked');
        });
    });

    // ── 2. Data Attributes ──────────────────────────────────

    describe('data attributes', () => {
        it('items have data-state "off" by default', () => {
            shouldBeOff('Item 1');
            shouldBeOff('Item 2');
            shouldBeOff('Item 3');
        });

        it('clicking item sets data-state "on", others remain "off" (single)', () => {
            getItem('Item 1').click();
            shouldBeOn('Item 1');
            shouldBeOff('Item 2');
            shouldBeOff('Item 3');
        });

        it('root has data-orientation "horizontal" by default', () => {
            getGroup().should('have.attr', 'data-orientation', 'horizontal');
        });

        it('items have data-orientation matching root', () => {
            getItem('Item 1').should('have.attr', 'data-orientation', 'horizontal');
            getItem('Item 3').should('have.attr', 'data-orientation', 'horizontal');
        });

        it('disabled item has data-disabled', () => {
            getItem('Item 2').should('have.attr', 'data-disabled');
        });

        it('non-disabled items do not have data-disabled', () => {
            getItem('Item 1').should('not.have.attr', 'data-disabled');
            getItem('Item 3').should('not.have.attr', 'data-disabled');
        });
    });

    // ── 3. Keyboard Navigation ──────────────────────────────

    describe('keyboard navigation', () => {
        it('Tab enters group and focuses first item', () => {
            // Items use roving tabindex managed by the group
            // Focus an item directly, then arrow keys work
            getItem('Item 1').focus();
            getItem('Item 1').should('be.focused');
        });

        it('Tab focuses active item when one is pressed', () => {
            getItem('Item 3').click();
            shouldBeOn('Item 3');
            // Move focus away from the group
            cy.realPress('Tab');
            // Tab back into the group should focus the pressed item
            cy.realPress(['Shift', 'Tab']);
            getItem('Item 3').should('be.focused');
        });

        it('ArrowRight moves focus to next item', () => {
            getItem('Item 1').focus();
            cy.realPress('ArrowRight');
            // Item 2 is disabled, should skip to Item 3
            getItem('Item 3').should('be.focused');
        });

        it('ArrowLeft moves focus to previous item', () => {
            getItem('Item 3').focus();
            cy.realPress('ArrowLeft');
            // Item 2 is disabled, should skip to Item 1
            getItem('Item 1').should('be.focused');
        });

        it('ArrowRight wraps around to first item', () => {
            getItem('Item 3').focus();
            cy.realPress('ArrowRight');
            getItem('Item 1').should('be.focused');
        });

        it('ArrowLeft wraps around to last item', () => {
            getItem('Item 1').focus();
            cy.realPress('ArrowLeft');
            getItem('Item 3').should('be.focused');
        });

        it('Home moves focus to first item', () => {
            getItem('Item 3').focus();
            cy.realPress('Home');
            getItem('Item 1').should('be.focused');
        });

        it('End moves focus to last item', () => {
            getItem('Item 1').focus();
            cy.realPress('End');
            getItem('Item 3').should('be.focused');
        });

        it('Space toggles focused item', () => {
            getItem('Item 1').focus();
            cy.realPress('Space');
            shouldBeOn('Item 1');
            cy.realPress('Space');
            shouldBeOff('Item 1');
        });

        it('Enter toggles focused item', () => {
            getItem('Item 1').focus();
            cy.realPress('Enter');
            shouldBeOn('Item 1');
            cy.realPress('Enter');
            shouldBeOff('Item 1');
        });

        it('arrow keys skip disabled items', () => {
            getItem('Item 1').focus();
            // ArrowRight should skip Item 2 (disabled) and go to Item 3
            cy.realPress('ArrowRight');
            getItem('Item 3').should('be.focused');
            // ArrowLeft should skip Item 2 (disabled) and go to Item 1
            cy.realPress('ArrowLeft');
            getItem('Item 1').should('be.focused');
        });
    });

    // ── 4. Pointer Interaction ──────────────────────────────

    describe('pointer interaction', () => {
        it('click selects item (single mode)', () => {
            getItem('Item 1').click();
            shouldBeOn('Item 1');
            shouldBeOff('Item 3');
        });

        it('click deselects when clicking selected item (single mode)', () => {
            getItem('Item 1').click();
            shouldBeOn('Item 1');
            getItem('Item 1').click();
            shouldBeOff('Item 1');
        });

        it('clicking another item deselects the first (single mode)', () => {
            getItem('Item 1').click();
            shouldBeOn('Item 1');
            getItem('Item 3').click();
            shouldBeOn('Item 3');
            shouldBeOff('Item 1');
        });

        it('clicking disabled item does nothing', () => {
            getItem('Item 2').click({force: true});
            shouldBeOff('Item 2');
        });
    });

    // ── 5. Single Type ──────────────────────────────────────

    describe('single type', () => {
        it('only one item can be active at a time', () => {
            getItem('Item 1').click();
            shouldBeOn('Item 1');
            getItem('Item 3').click();
            shouldBeOn('Item 3');
            shouldBeOff('Item 1');
        });

        it('clicking selected item deselects it', () => {
            getItem('Item 1').click();
            shouldBeOn('Item 1');
            getItem('Item 1').click();
            shouldBeOff('Item 1');
        });
    });

    // ── 6. Multiple Type ────────────────────────────────────

    describe('multiple type', () => {
        beforeEach(() => {
            cy.findByLabelText('multiple').click();
        });

        it('multiple items can be active', () => {
            getItem('Item 1').click();
            shouldBeOn('Item 1');
            getItem('Item 3').click();
            shouldBeOn('Item 3');
            shouldBeOn('Item 1');
        });

        it('clicking active item deselects only that item', () => {
            getItem('Item 1').click();
            getItem('Item 3').click();
            shouldBeOn('Item 1');
            shouldBeOn('Item 3');
            getItem('Item 1').click();
            shouldBeOff('Item 1');
            shouldBeOn('Item 3');
        });

        it('items have aria-pressed instead of role="radio"', () => {
            getItem('Item 1').should('have.attr', 'aria-pressed', 'false');
            getItem('Item 1').should('not.have.attr', 'role', 'radio');
            getItem('Item 1').click();
            getItem('Item 1').should('have.attr', 'aria-pressed', 'true');
        });
    });

    // ── 7. Vertical Orientation ─────────────────────────────

    describe('vertical orientation', () => {
        beforeEach(() => {
            cy.findByLabelText('vertical').click();
        });

        it('data-orientation updates to vertical', () => {
            getGroup().should('have.attr', 'data-orientation', 'vertical');
            getItem('Item 1').should('have.attr', 'data-orientation', 'vertical');
        });

        it('ArrowDown moves to next item', () => {
            getItem('Item 1').focus();
            cy.realPress('ArrowDown');
            // Item 2 is disabled, skip to Item 3
            getItem('Item 3').should('be.focused');
        });

        it('ArrowUp moves to previous item', () => {
            getItem('Item 3').focus();
            cy.realPress('ArrowUp');
            // Item 2 is disabled, skip to Item 1
            getItem('Item 1').should('be.focused');
        });

        it('ArrowRight does not navigate in vertical mode', () => {
            getItem('Item 1').focus();
            cy.realPress('ArrowRight');
            getItem('Item 1').should('be.focused');
        });

        it('ArrowLeft does not navigate in vertical mode', () => {
            getItem('Item 1').focus();
            cy.realPress('ArrowLeft');
            getItem('Item 1').should('be.focused');
        });

        it('Home and End still work in vertical mode', () => {
            getItem('Item 1').focus();
            cy.realPress('End');
            getItem('Item 3').should('be.focused');
            cy.realPress('Home');
            getItem('Item 1').should('be.focused');
        });
    });

    // ── 8. Disabled Variant ─────────────────────────────────

    describe('disabled variant', () => {
        beforeEach(() => {
            cy.findByLabelText('disabled').click();
        });

        it('items cannot be toggled', () => {
            getItem('Item 1').click({force: true});
            shouldBeOff('Item 1');
            getItem('Item 3').click({force: true});
            shouldBeOff('Item 3');
        });

        it('all items have data-disabled', () => {
            getItem('Item 1').should('have.attr', 'data-disabled');
            getItem('Item 2').should('have.attr', 'data-disabled');
            getItem('Item 3').should('have.attr', 'data-disabled');
        });

        it('all items have disabled attribute', () => {
            getItem('Item 1').should('be.disabled');
            getItem('Item 2').should('be.disabled');
            getItem('Item 3').should('be.disabled');
        });
    });
});
