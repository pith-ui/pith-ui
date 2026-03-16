// ── Experiment: Non-Overridable Internal Styles ─────────────────────────────
//
// Tests approaches for making internal CSS var aliases non-overridable,
// matching React's {…props.style, …internal} pattern.
//
// For each fixture, we check:
// - Does the internal var hold its value despite user override attempts?
// - Are non-conflicting internal vars preserved?
// - Are non-style user attrs (data-custom) still forwarded?

describe('Non-Overridable Internal Styles', () => {
    beforeEach(() => {
        cy.visit('/non-overridable-styles');
    });

    function getCssVar(testId, varName) {
        return cy.findByTestId(testId).then(($el) => {
            return getComputedStyle($el[0]).getPropertyValue(varName).trim();
        });
    }

    // ── 1. style: directives after spread on same element ───────────────

    describe('style: directives after spread on same element', () => {
        it('internal var wins over user attr:style', () => {
            getCssVar('inner-style-target', '--internal-var').then((val) => {
                expect(val).to.equal('internal-value');
            });
        });

        it('non-conflicting internal var preserved', () => {
            getCssVar('inner-style-target', '--internal-only').then((val) => {
                expect(val).to.equal('only-internal');
            });
        });

        it('non-style user attrs still forwarded', () => {
            cy.findByTestId('inner-style-target').should('have.attr', 'data-custom', 'test');
        });
    });

    // ── 2. Inner component sets style: (user attr:style via add_any_attr)

    describe('inner component with style: directives (user attr:style)', () => {
        // SKIP: When user attrs and internal style: directives are on the SAME element
        // (via add_any_attr), user attr:style clobbers internal style: directives.
        // This approach does NOT provide non-overridable behavior.
        it.skip('FAILS: user attr:style clobbers internal style: on same element via add_any_attr', () => {
            getCssVar('inner-attr-style-target', '--internal-var').then((val) => {
                expect(val).to.equal('internal-value');
            });
        });

        // SKIP: Same root cause — attr:style replaces the entire inline style
        it.skip('FAILS: non-conflicting internal var also clobbered by user attr:style', () => {
            getCssVar('inner-attr-style-target', '--internal-only').then((val) => {
                expect(val).to.equal('only-internal');
            });
        });

        it('non-style user attrs still forwarded', () => {
            cy.findByTestId('inner-attr-style-target').should('have.attr', 'data-custom', 'test');
        });
    });

    // ── 3. Inner component (user style: directive) ──────────────────────

    describe('inner component with style: directives (user style: directive)', () => {
        // SKIP: When user style: directive and internal style: directive target the
        // same property on the same element, the user's wins because add_any_attr
        // processes them in order and the user's comes later in the chain.
        it.skip('FAILS: user style: directive wins when both target same element via add_any_attr', () => {
            getCssVar('inner-style-dir-target', '--internal-var').then((val) => {
                expect(val).to.equal('internal-value');
            });
        });

        it('non-conflicting internal var preserved (style: directives are additive)', () => {
            getCssVar('inner-style-dir-target', '--internal-only').then((val) => {
                expect(val).to.equal('only-internal');
            });
        });
    });

    // ── 4a. Popper-like nested (user attr:style) ────────────────────────

    describe('popper-like nested (user attr:style)', () => {
        it('internal var on inner element is unaffected by user attr:style on outer', () => {
            getCssVar('popper-attr-style-target', '--internal-var').then((val) => {
                expect(val).to.equal('internal-value');
            });
        });

        it('non-conflicting internal var preserved', () => {
            getCssVar('popper-attr-style-target', '--internal-only').then((val) => {
                expect(val).to.equal('only-internal');
            });
        });

        it('user data attr forwarded to outer wrapper', () => {
            // User attrs land on the outer wrapper (first DOM element), not the inner
            cy.get('[data-testid="fixture-4a"] [data-custom="test"]').should('exist');
        });
    });

    // ── 4b. Popper-like nested (user style: directive) ──────────────────

    describe('popper-like nested (user style: directive)', () => {
        it('internal var on inner element is unaffected by user style: on outer', () => {
            getCssVar('popper-style-dir-target', '--internal-var').then((val) => {
                expect(val).to.equal('internal-value');
            });
        });

        it('non-conflicting internal var preserved', () => {
            getCssVar('popper-style-dir-target', '--internal-only').then((val) => {
                expect(val).to.equal('only-internal');
            });
        });
    });
});
