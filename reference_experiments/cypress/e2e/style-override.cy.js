// ── Experiment: Style Override Order with {..} Spread ────────────────────────
//
// Tests how style: directives and {..spread()} interact when both set
// inline style properties. Key question: does the user's attr:style
// (via spread) override internal style: directives, or vice versa?

describe('Style Override Order', () => {
    beforeEach(() => {
        cy.visit('/style-override');
    });

    // Helper to get a CSS custom property value
    function getCssVar(testId, varName) {
        return cy.findByTestId(testId).then(($el) => {
            return getComputedStyle($el[0]).getPropertyValue(varName).trim();
        });
    }

    // ── 1. style: directive first, spread after ─────────────────────────────

    describe('style: directive first, then spread', () => {
        it('user style from spread overrides internal style: directive', () => {
            getCssVar('style-directive-target', '--my-var').then((val) => {
                expect(val).to.equal('user-value');
            });
        });
    });

    // ── 2. spread first, then style: directive ──────────────────────────────

    describe('spread first, then style: directive', () => {
        it('internal style: directive overrides user spread when declared after', () => {
            getCssVar('spread-first-target', '--my-var').then((val) => {
                expect(val).to.equal('internal-value');
            });
        });
    });

    // ── 3. Multiple internal vars, user overrides one ───────────────────────

    describe('multiple internal vars, user overrides one', () => {
        it('overridden var has user value', () => {
            getCssVar('multi-style-target', '--internal-a').then((val) => {
                expect(val).to.equal('user-override');
            });
        });

        // SKIP: attr:style from the spread replaces the ENTIRE inline style attribute
        // via setAttribute("style", ...), which clobbers all style: directive values
        // (set via setProperty). Even non-conflicting style: properties are lost.
        // This is a fundamental DOM behavior: setAttribute replaces, setProperty is additive.
        it.skip('FAILS: non-overridden var is clobbered — attr:style replaces all style: directives', () => {
            getCssVar('multi-style-target', '--internal-b').then((val) => {
                expect(val).to.equal('value-b');
            });
        });
    });

    // ── 4. No conflict ──────────────────────────────────────────────────────

    describe('no conflict (user has no style attr)', () => {
        it('internal var is present', () => {
            getCssVar('no-conflict-target', '--internal-var').then((val) => {
                expect(val).to.equal('internal-only');
            });
        });

        it('user data attr is also present', () => {
            cy.findByTestId('no-conflict-target').should('have.attr', 'data-custom', 'hello');
        });
    });

    // ── 5. Accordion-like pattern ───────────────────────────────────────────

    describe('accordion-like CSS var alias', () => {
        it('without user override: accordion var resolves through alias', () => {
            cy.get('[data-testid="fixture-5a"] [data-testid="accordion-like-target"]').then(($el) => {
                const val = getComputedStyle($el[0]).getPropertyValue('--accordion-content-height').trim();
                expect(val).to.equal('42px');
            });
        });

        it('with user override: user value wins', () => {
            cy.get('[data-testid="fixture-5b"] [data-testid="accordion-like-target"]').then(($el) => {
                const val = getComputedStyle($el[0]).getPropertyValue('--accordion-content-height').trim();
                expect(val).to.equal('999px');
            });
        });

        // SKIP: Same attr:style clobbering issue. The user's attr:style="--accordion-content-height: 999px"
        // replaces the entire inline style, wiping out style:--collapsible-content-height="42px".
        it.skip('FAILS: collapsible var clobbered by user attr:style — setAttribute replaces all style: values', () => {
            cy.get('[data-testid="fixture-5b"] [data-testid="accordion-like-target"]').then(($el) => {
                const val = getComputedStyle($el[0]).getPropertyValue('--collapsible-content-height').trim();
                expect(val).to.equal('42px');
            });
        });
    });

    // ── 6. User style: directive via ForwardedAttrs ─────────────────────────
    //
    // User passes style:--internal-a="user-override" (not attr:style).
    // Does ForwardedAttrs type erasure still cause clobbering?

    describe('user style: directive via ForwardedAttrs', () => {
        it('overridden var has user value', () => {
            getCssVar('style-directive-override-target', '--internal-a').then((val) => {
                expect(val).to.equal('user-override');
            });
        });

        it('non-overridden var keeps internal value', () => {
            getCssVar('style-directive-override-target', '--internal-b').then((val) => {
                expect(val).to.equal('value-b');
            });
        });
    });

    // ── 7. Vanilla AttributeInterceptor + direct spread ─────────────────────
    //
    // No ForwardedAttrs — direct {..attrs} from AttributeInterceptor.
    // Does into_any_attr() type erasure cause the same clobbering?

    describe('vanilla AttributeInterceptor + direct spread', () => {
        it('overridden var has user value', () => {
            getCssVar('vanilla-interceptor-target', '--internal-a').then((val) => {
                expect(val).to.equal('user-override');
            });
        });

        it('non-overridden var keeps internal value', () => {
            getCssVar('vanilla-interceptor-target', '--internal-b').then((val) => {
                expect(val).to.equal('value-b');
            });
        });
    });

    // ── 8. No interceptor — native Leptos spreading ─────────────────────────
    //
    // Leptos handles attr spreading natively via add_any_attr.
    // No type erasure through AnyAttribute.

    describe('native Leptos spreading (no interceptor)', () => {
        it('overridden var has user value', () => {
            getCssVar('native-spread-target', '--internal-a').then((val) => {
                expect(val).to.equal('user-override');
            });
        });

        it('non-overridden var keeps internal value', () => {
            getCssVar('native-spread-target', '--internal-b').then((val) => {
                expect(val).to.equal('value-b');
            });
        });
    });
});
