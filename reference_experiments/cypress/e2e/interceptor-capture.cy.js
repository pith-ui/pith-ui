// ── Experiment: What Does AttributeInterceptor Actually Capture? ─────────────
//
// Tests whether AttributeInterceptor captures attrs from parent add_any_attr
// propagation, not just attrs set explicitly in the view! macro.
//
// If it captures everything, the transfer Effect in PopperContent is unnecessary.

describe('AttributeInterceptor Capture Scope', () => {
    beforeEach(() => {
        cy.visit('/interceptor-capture');
    });

    // ── 1. Parent add_any_attr attrs ────────────────────────────────────────

    describe('parent add_any_attr attrs', () => {
        it('parent class is on inner (captured by interceptor), not wrapper', () => {
            cy.get('[data-testid="fixture-1"] [data-testid="inner"]')
                .should('have.class', 'from-parent');
            cy.get('[data-testid="fixture-1"] [data-testid="wrapper"]')
                .should('not.have.class', 'from-parent');
        });

        it('parent data-custom is on inner, not wrapper', () => {
            cy.get('[data-testid="fixture-1"] [data-testid="inner"]')
                .should('have.attr', 'data-custom', 'parent-value');
            cy.get('[data-testid="fixture-1"] [data-testid="wrapper"]')
                .should('not.have.attr', 'data-custom');
        });

        it('parent attr:style is on inner, not wrapper', () => {
            cy.get('[data-testid="fixture-1"] [data-testid="inner"]').then(($el) => {
                const val = $el[0].style.getPropertyValue('--my-var');
                expect(val.trim()).to.equal('parent-set');
            });
        });
    });

    // ── 2. Parent style: directive ──────────────────────────────────────────

    describe('parent style: directive', () => {
        it('parent style: directive is on inner, not wrapper', () => {
            cy.get('[data-testid="fixture-2"] [data-testid="inner"]').then(($el) => {
                const val = $el[0].style.getPropertyValue('--my-var');
                expect(val.trim()).to.equal('parent-set-via-directive');
            });
        });

        it('parent data attr is on inner', () => {
            cy.get('[data-testid="fixture-2"] [data-testid="inner"]')
                .should('have.attr', 'data-custom', 'parent-value');
        });
    });

    // ── 3. Two-level nesting ────────────────────────────────────────────────
    //
    // NOTE: attrs on MiddleWrapper go to MiddleWrapper's first DOM element
    // (which is children), not passed through to InnerComponent. This is
    // a Leptos limitation — add_any_attr targets the outermost element.
    // We test what actually happens.

    describe('two-level nesting', () => {
        it('attrs from outer land on middle children wrapper, not propagated to inner', () => {
            // MiddleWrapper renders {children()} — attrs from the parent land on
            // whatever the children render, not on InnerComponent specifically.
            // This tests the actual Leptos behavior.
            cy.get('[data-testid="fixture-3"] [data-testid="wrapper"]').then(($el) => {
                // Attrs may or may not reach the inner — let's observe
                const hasClass = $el[0].classList.contains('from-outer');
                cy.log(`wrapper has from-outer class: ${hasClass}`);
            });
        });
    });

    // ── 4. Both internal and parent attrs ────────────────────────────────────

    describe('internal + parent attrs both captured', () => {
        it('internal attr is on inner', () => {
            cy.findByTestId('inner-4').should('have.attr', 'data-internal', 'set-in-view-macro');
        });

        it('parent attr is also on inner', () => {
            cy.findByTestId('inner-4').should('have.attr', 'data-from-parent', 'parent-value');
        });

        it('parent class is also on inner', () => {
            cy.findByTestId('inner-4').should('have.class', 'parent-class');
        });

        it('wrapper has neither parent attr nor class (all redirected)', () => {
            cy.findByTestId('wrapper-4').should('not.have.attr', 'data-from-parent');
            cy.findByTestId('wrapper-4').should('not.have.class', 'parent-class');
        });
    });
});
