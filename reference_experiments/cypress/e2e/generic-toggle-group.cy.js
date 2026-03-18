// Experiment: Generic Toggle Group
//
// Tests that all three approaches to type-safe toggle group modes
// produce the same runtime behavior: clicking items toggles state,
// single mode allows one value, multiple mode allows many.

describe('Generic Toggle Group', () => {
    beforeEach(() => {
        cy.visit('/generic-toggle-group');
    });

    // ── Approach 1: Generic trait core ──

    describe('Approach 1 - Generic trait core', () => {
        it('single mode: clicking an item selects it', () => {
            cy.get('[data-testid="approach1-item-a"]').click();
            cy.get('[data-testid="approach1-item-a"]').should('have.attr', 'data-state', 'on');
            cy.get('[data-testid="a1-single-output"]').should('contain.text', 'Value: a');
        });

        it('single mode: clicking another item replaces selection', () => {
            cy.get('[data-testid="approach1-item-a"]').click();
            cy.get('[data-testid="approach1-item-b"]').click();
            cy.get('[data-testid="approach1-item-a"]').should('have.attr', 'data-state', 'off');
            cy.get('[data-testid="approach1-item-b"]').should('have.attr', 'data-state', 'on');
            cy.get('[data-testid="a1-single-output"]').should('contain.text', 'Value: b');
        });

        it('single mode: clicking same item deselects it', () => {
            cy.get('[data-testid="approach1-item-a"]').click();
            cy.get('[data-testid="approach1-item-a"]').click();
            cy.get('[data-testid="approach1-item-a"]').should('have.attr', 'data-state', 'off');
            cy.get('[data-testid="a1-single-output"]').should('contain.text', 'Value: ');
        });

        it('multiple mode: clicking items adds to selection', () => {
            cy.get('[data-testid="approach1-item-x"]').click();
            cy.get('[data-testid="approach1-item-y"]').click();
            cy.get('[data-testid="approach1-item-x"]').should('have.attr', 'data-state', 'on');
            cy.get('[data-testid="approach1-item-y"]').should('have.attr', 'data-state', 'on');
            cy.get('[data-testid="a1-multi-output"]').should('contain.text', 'Values: x, y');
        });

        it('multiple mode: clicking selected item removes it', () => {
            cy.get('[data-testid="approach1-item-x"]').click();
            cy.get('[data-testid="approach1-item-y"]').click();
            cy.get('[data-testid="approach1-item-x"]').click();
            cy.get('[data-testid="approach1-item-x"]').should('have.attr', 'data-state', 'off');
            cy.get('[data-testid="approach1-item-y"]').should('have.attr', 'data-state', 'on');
            cy.get('[data-testid="a1-multi-output"]').should('contain.text', 'Values: y');
        });
    });

    // ── Approach 2: Split components ──

    describe('Approach 2 - Split components', () => {
        it('single mode: clicking an item selects it', () => {
            cy.get('[data-testid="approach2-item-a"]').click();
            cy.get('[data-testid="approach2-item-a"]').should('have.attr', 'data-state', 'on');
            cy.get('[data-testid="a2-single-output"]').should('contain.text', 'Value: a');
        });

        it('single mode: clicking another item replaces selection', () => {
            cy.get('[data-testid="approach2-item-a"]').click();
            cy.get('[data-testid="approach2-item-b"]').click();
            cy.get('[data-testid="approach2-item-a"]').should('have.attr', 'data-state', 'off');
            cy.get('[data-testid="approach2-item-b"]').should('have.attr', 'data-state', 'on');
            cy.get('[data-testid="a2-single-output"]').should('contain.text', 'Value: b');
        });

        it('multiple mode: clicking items adds to selection', () => {
            cy.get('[data-testid="approach2-item-x"]').click();
            cy.get('[data-testid="approach2-item-y"]').click();
            cy.get('[data-testid="approach2-item-x"]').should('have.attr', 'data-state', 'on');
            cy.get('[data-testid="approach2-item-y"]').should('have.attr', 'data-state', 'on');
            cy.get('[data-testid="a2-multi-output"]').should('contain.text', 'Values: x, y');
        });

        it('multiple mode: clicking selected item removes it', () => {
            cy.get('[data-testid="approach2-item-x"]').click();
            cy.get('[data-testid="approach2-item-y"]').click();
            cy.get('[data-testid="approach2-item-x"]').click();
            cy.get('[data-testid="approach2-item-x"]').should('have.attr', 'data-state', 'off');
            cy.get('[data-testid="a2-multi-output"]').should('contain.text', 'Values: y');
        });
    });

    // ── Approach 3: Enum value ──

    describe('Approach 3 - Enum value', () => {
        it('single mode: clicking an item selects it', () => {
            cy.get('[data-testid="approach3-item-a"]').click();
            cy.get('[data-testid="approach3-item-a"]').should('have.attr', 'data-state', 'on');
            cy.get('[data-testid="a3-single-output"]').should('contain.text', 'Value: a');
        });

        it('single mode: clicking another item replaces selection', () => {
            cy.get('[data-testid="approach3-item-a"]').click();
            cy.get('[data-testid="approach3-item-b"]').click();
            cy.get('[data-testid="approach3-item-a"]').should('have.attr', 'data-state', 'off');
            cy.get('[data-testid="approach3-item-b"]').should('have.attr', 'data-state', 'on');
            cy.get('[data-testid="a3-single-output"]').should('contain.text', 'Value: b');
        });

        it('multiple mode: clicking items adds to selection', () => {
            cy.get('[data-testid="approach3-item-x"]').click();
            cy.get('[data-testid="approach3-item-y"]').click();
            cy.get('[data-testid="approach3-item-x"]').should('have.attr', 'data-state', 'on');
            cy.get('[data-testid="approach3-item-y"]').should('have.attr', 'data-state', 'on');
            cy.get('[data-testid="a3-multi-output"]').should('contain.text', 'Values: x, y');
        });

        it('multiple mode: clicking selected item removes it', () => {
            cy.get('[data-testid="approach3-item-x"]').click();
            cy.get('[data-testid="approach3-item-y"]').click();
            cy.get('[data-testid="approach3-item-x"]').click();
            cy.get('[data-testid="approach3-item-x"]').should('have.attr', 'data-state', 'off');
            cy.get('[data-testid="a3-multi-output"]').should('contain.text', 'Values: y');
        });
    });
});
