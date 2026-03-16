// ── Experiment: AttributeInterceptor + AnyAttribute Clone ────────────────────
//
// Hypothesis: AnyAttribute implements Clone, so we can spread intercepted attrs
// onto multiple elements via `{..attrs.clone()}` and `{..attrs}`.
//
// If any test here fails, DO NOT delete it — skip it and document why it fails.

describe('Attribute Clone', () => {
    beforeEach(() => {
        cy.visit('/attribute-clone');
    });

    // ── Static Attributes ───────────────────────────────────────────────────

    describe('static attributes', () => {
        it('both targets receive the class attribute', () => {
            cy.findByTestId('static-fixture').within(() => {
                cy.findByTestId('target-a').should('have.class', 'shared-class');
                cy.findByTestId('target-b').should('have.class', 'shared-class');
            });
        });

        it('both targets receive data-custom attribute', () => {
            cy.findByTestId('static-fixture').within(() => {
                cy.findByTestId('target-a').should('have.attr', 'data-custom', 'hello');
                cy.findByTestId('target-b').should('have.attr', 'data-custom', 'hello');
            });
        });

        it('both targets receive aria-label attribute', () => {
            cy.findByTestId('static-fixture').within(() => {
                cy.findByTestId('target-a').should('have.attr', 'aria-label', 'shared label');
                cy.findByTestId('target-b').should('have.attr', 'aria-label', 'shared label');
            });
        });
    });

    // ── Reactive Attributes ─────────────────────────────────────────────────

    describe('reactive attributes', () => {
        it('both targets have initial reactive value', () => {
            cy.findByTestId('reactive-fixture').within(() => {
                cy.findByTestId('target-a').should('have.attr', 'data-count', '0');
                cy.findByTestId('target-b').should('have.attr', 'data-count', '0');
            });
        });

        it('both targets update when the signal changes', () => {
            cy.findByTestId('reactive-fixture').within(() => {
                cy.findByTestId('increment').click();
                cy.findByTestId('target-a').should('have.attr', 'data-count', '1');
                cy.findByTestId('target-b').should('have.attr', 'data-count', '1');
            });
        });

        it('both targets stay in sync after multiple updates', () => {
            cy.findByTestId('reactive-fixture').within(() => {
                cy.findByTestId('increment').click();
                cy.findByTestId('increment').click();
                cy.findByTestId('increment').click();

                cy.findByTestId('target-a')
                    .invoke('attr', 'data-count')
                    .then((valA) => {
                        cy.findByTestId('target-b').should('have.attr', 'data-count', valA);
                    });
            });
        });
    });

    // ── Multiple Heterogeneous Attributes ───────────────────────────────────

    describe('multiple attributes', () => {
        it('both targets receive all five attributes', () => {
            cy.findByTestId('multi-fixture').within(() => {
                ['target-a', 'target-b'].forEach((testId) => {
                    cy.findByTestId(testId).should('have.class', 'class-one');
                    cy.findByTestId(testId).should('have.attr', 'data-foo', 'foo-val');
                    cy.findByTestId(testId).should('have.attr', 'data-bar', 'bar-val');
                    cy.findByTestId(testId).should('have.attr', 'aria-describedby', 'desc');
                    cy.findByTestId(testId).should('have.attr', 'title', 'tooltip text');
                });
            });
        });
    });

    // ── Independence ────────────────────────────────────────────────────────

    describe('independence', () => {
        it('removing an attribute from target-a does not affect target-b', () => {
            cy.findByTestId('static-fixture').within(() => {
                // Programmatically remove data-custom from target-a
                cy.findByTestId('target-a').then(($el) => {
                    $el[0].removeAttribute('data-custom');
                });

                // target-a should no longer have it
                cy.findByTestId('target-a').should('not.have.attr', 'data-custom');

                // target-b should still have it — they are independent DOM nodes
                cy.findByTestId('target-b').should('have.attr', 'data-custom', 'hello');
            });
        });
    });
});
