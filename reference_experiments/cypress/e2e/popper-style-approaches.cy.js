// ── Experiment: PopperContent-like Style Approaches ──────────────────────────
//
// Tests whether CSS var aliases on the INNER element survive user attrs
// landing on the WRAPPER element (via add_any_attr).

describe('PopperContent Style Approaches', () => {
    beforeEach(() => {
        cy.visit('/popper-style-approaches');
    });

    function getCssVar(testId, varName) {
        return cy.findByTestId(testId).then(($el) => {
            return getComputedStyle($el[0]).getPropertyValue(varName).trim();
        });
    }

    // ── Approach A: inner_style prop ────────────────────────────────────────

    describe('A: inner_style prop', () => {
        it('CSS var alias is set on inner element', () => {
            getCssVar('approach-a-inner', '--dropdown-menu-content-available-width').then((val) => {
                // This is a var() reference — may resolve to empty if the source var isn't set,
                // but the PROPERTY should exist on the element
                cy.findByTestId('approach-a-inner').then(($el) => {
                    const style = $el[0].style;
                    const raw = style.getPropertyValue('--dropdown-menu-content-available-width');
                    expect(raw.trim()).to.not.be.empty;
                });
            });
        });

        it('user attr:style on wrapper does NOT override inner CSS var', () => {
            cy.findByTestId('approach-a-inner').then(($el) => {
                const style = $el[0].style;
                const val = style.getPropertyValue('--dropdown-menu-content-transform-origin');
                // Should be the internal alias, not the user's override
                expect(val.trim()).to.equal('var(--popper-transform-origin)');
            });
        });

        it('user data attr lands on wrapper, not inner', () => {
            cy.findByTestId('approach-a-inner').should('not.have.attr', 'data-custom');
            cy.get('[data-wrapper="true"]').first().should('have.attr', 'data-custom', 'test');
        });
    });

    // ── Approach B: Context-based styles ────────────────────────────────────

    describe('B: context-based styles', () => {
        it('CSS var alias is set on inner element', () => {
            cy.findByTestId('approach-b-inner').then(($el) => {
                const style = $el[0].style;
                const raw = style.getPropertyValue('--dropdown-menu-content-available-width');
                expect(raw.trim()).to.not.be.empty;
            });
        });

        it('user attr:style on wrapper does NOT override inner CSS var', () => {
            cy.findByTestId('approach-b-inner').then(($el) => {
                const style = $el[0].style;
                const val = style.getPropertyValue('--dropdown-menu-content-transform-origin');
                expect(val.trim()).to.equal('var(--popper-transform-origin)');
            });
        });

        it('user data attr lands on wrapper, not inner', () => {
            cy.findByTestId('approach-b-inner').should('not.have.attr', 'data-custom');
        });
    });

    // ── Approach C: Control (no transfer) ───────────────────────────────────

    describe('C: control (no transfer)', () => {
        it('user attr:style stays on wrapper, inner has no styles', () => {
            cy.findByTestId('approach-c-inner').then(($el) => {
                const style = $el[0].style;
                const val = style.getPropertyValue('--dropdown-menu-content-transform-origin');
                expect(val.trim()).to.be.empty;
            });
        });

        it('user data attr stays on wrapper, not on inner', () => {
            cy.findByTestId('approach-c-inner').should('not.have.attr', 'data-custom');
        });
    });
});
