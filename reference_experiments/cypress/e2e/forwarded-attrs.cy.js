// ── Experiment: ForwardedAttrs — Reactive Attribute Forwarding ───────────────
//
// Tests the ForwardedAttrs primitive, which should solve the limitations
// of the extract_attrs + StoredValue + Effect workaround:
//
// 1. Static attrs survive Show cycling (same as old workaround)
// 2. Reactive attrs maintain reactivity (NEW — old workaround freezes them)
// 3. Multiple targets get independent reactive subscriptions (NEW)
//
// Failing tests are SKIPPED with explanations, never deleted.

describe('ForwardedAttrs', () => {
    beforeEach(() => {
        cy.visit('/forwarded-attrs');
    });

    // ── Helpers ─────────────────────────────────────────────────────────────

    function assertStaticAttrs(testId) {
        cy.findByTestId(testId).should('have.class', 'test-class');
        cy.findByTestId(testId).should('have.attr', 'data-custom', 'test-value');
        cy.findByTestId(testId).should('have.attr', 'aria-label', 'test-label');
    }

    // ── 1. Static Attrs + Internal Show ─────────────────────────────────────

    describe('static attrs + internal Show', () => {
        it('attrs are present on initial render', () => {
            assertStaticAttrs('static-target');
        });

        it('attrs survive Show cycling (hide → show)', () => {
            cy.findByTestId('static-toggle').click();
            cy.findByTestId('static-target').should('not.exist');

            cy.findByTestId('static-toggle').click();
            cy.findByTestId('static-target').should('exist');
            assertStaticAttrs('static-target');
        });

        it('attrs survive multiple Show cycles', () => {
            for (let i = 0; i < 3; i++) {
                cy.findByTestId('static-toggle').click();
                cy.findByTestId('static-target').should('not.exist');

                cy.findByTestId('static-toggle').click();
                cy.findByTestId('static-target').should('exist');
                assertStaticAttrs('static-target');
            }
        });
    });

    // ── 2. Reactive Attrs + Internal Show ───────────────────────────────────
    //
    // These are the KEY tests. The old extract_attrs workaround FREEZES
    // reactive values. ForwardedAttrs should preserve reactivity by keeping
    // the AnyAttribute (with its reactive closures) and calling build() on
    // the target element.

    describe('reactive attrs + internal Show', () => {
        it('reactive attr has initial value', () => {
            cy.findByTestId('reactive-target').should('have.attr', 'data-count', '0');
        });

        it('reactive attr updates when signal changes', () => {
            cy.findByTestId('reactive-increment').click();
            cy.findByTestId('reactive-target').should('have.attr', 'data-count', '1');
        });

        it('reactive attr updates multiple times', () => {
            cy.findByTestId('reactive-increment').click();
            cy.findByTestId('reactive-increment').click();
            cy.findByTestId('reactive-increment').click();
            cy.findByTestId('reactive-target').should('have.attr', 'data-count', '3');
        });

        it('reactive attr persists latest value after Show cycling', () => {
            // Increment to 2
            cy.findByTestId('reactive-increment').click();
            cy.findByTestId('reactive-increment').click();
            cy.findByTestId('reactive-target').should('have.attr', 'data-count', '2');

            // Cycle Show
            cy.findByTestId('reactive-toggle').click();
            cy.findByTestId('reactive-target').should('not.exist');
            cy.findByTestId('reactive-toggle').click();
            cy.findByTestId('reactive-target').should('exist');

            // After re-mount, build() is called again with the same AnyAttribute.
            // The closure reads the signal's CURRENT value.
            cy.findByTestId('reactive-target').should('have.attr', 'data-count', '2');
        });

        it('reactive attr updates AFTER Show cycling', () => {
            // Cycle Show first
            cy.findByTestId('reactive-toggle').click();
            cy.findByTestId('reactive-target').should('not.exist');
            cy.findByTestId('reactive-toggle').click();
            cy.findByTestId('reactive-target').should('exist');

            // Now increment — does reactivity still work on the re-mounted element?
            cy.findByTestId('reactive-increment').click();
            cy.findByTestId('reactive-target').should('have.attr', 'data-count', '1');

            cy.findByTestId('reactive-increment').click();
            cy.findByTestId('reactive-target').should('have.attr', 'data-count', '2');
        });

        it('reactive attr survives multiple Show cycles with interleaved updates', () => {
            // Increment, cycle, increment, cycle, check
            cy.findByTestId('reactive-increment').click();
            cy.findByTestId('reactive-target').should('have.attr', 'data-count', '1');

            cy.findByTestId('reactive-toggle').click();
            cy.findByTestId('reactive-target').should('not.exist');
            cy.findByTestId('reactive-toggle').click();
            cy.findByTestId('reactive-target').should('have.attr', 'data-count', '1');

            cy.findByTestId('reactive-increment').click();
            cy.findByTestId('reactive-target').should('have.attr', 'data-count', '2');

            cy.findByTestId('reactive-toggle').click();
            cy.findByTestId('reactive-target').should('not.exist');
            cy.findByTestId('reactive-toggle').click();
            cy.findByTestId('reactive-target').should('have.attr', 'data-count', '2');

            cy.findByTestId('reactive-increment').click();
            cy.findByTestId('reactive-target').should('have.attr', 'data-count', '3');
        });
    });

    // ── 3. Multi-Target + Reactive Attrs ────────────────────────────────────

    describe('multi-target + reactive attrs', () => {
        it('both targets have initial static and reactive attrs', () => {
            cy.findByTestId('multi-target-a').should('have.class', 'shared-class');
            cy.findByTestId('multi-target-b').should('have.class', 'shared-class');
            cy.findByTestId('multi-target-a').should('have.attr', 'data-count', '0');
            cy.findByTestId('multi-target-b').should('have.attr', 'data-count', '0');
        });

        it('both targets update reactively in sync', () => {
            cy.findByTestId('multi-increment').click();
            cy.findByTestId('multi-target-a').should('have.attr', 'data-count', '1');
            cy.findByTestId('multi-target-b').should('have.attr', 'data-count', '1');

            cy.findByTestId('multi-increment').click();
            cy.findByTestId('multi-target-a').should('have.attr', 'data-count', '2');
            cy.findByTestId('multi-target-b').should('have.attr', 'data-count', '2');
        });

        it('both targets survive Show cycling with reactivity', () => {
            cy.findByTestId('multi-increment').click();
            cy.findByTestId('multi-target-a').should('have.attr', 'data-count', '1');

            // Cycle Show
            cy.findByTestId('multi-toggle').click();
            cy.findByTestId('multi-target-a').should('not.exist');
            cy.findByTestId('multi-toggle').click();
            cy.findByTestId('multi-target-a').should('exist');

            // Both should have the latest value
            cy.findByTestId('multi-target-a').should('have.attr', 'data-count', '1');
            cy.findByTestId('multi-target-b').should('have.attr', 'data-count', '1');

            // Both should still be reactive
            cy.findByTestId('multi-increment').click();
            cy.findByTestId('multi-target-a').should('have.attr', 'data-count', '2');
            cy.findByTestId('multi-target-b').should('have.attr', 'data-count', '2');
        });

        it('targets are independent — DOM mutation on one does not affect the other', () => {
            cy.findByTestId('multi-target-a').then(($el) => {
                $el[0].removeAttribute('data-count');
            });
            cy.findByTestId('multi-target-a').should('not.have.attr', 'data-count');
            cy.findByTestId('multi-target-b').should('have.attr', 'data-count', '0');
        });
    });
});
