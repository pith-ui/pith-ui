// ── Experiment: Attribute Spreading Under Presence/Show ──────────────────────
//
// Tests whether attributes survive Show toggling (unmount → remount) under
// various strategies. Each fixture tests a different approach:
//
// 1. External Show baseline: attrs spread on element, parent Show cycles component
// 2. StoredValue+Effect workaround: attrs extracted and re-applied via Effect
// 3. Wrong-element problem: attrs land on wrapper (outside Show) not target (inside)
//    (Spreading inside Show doesn't compile — see fixture 3 comments in .rs file)
// 4. Internal Show + Effect: workaround with Show inside the component
// 5. Reactive attrs + Show cycling: signal-driven attrs with the workaround
//
// Failing tests are SKIPPED with explanations, never deleted.

describe('Presence/Show Attribute Spreading', () => {
    beforeEach(() => {
        cy.visit('/presence-attrs');
    });

    // ── Helpers ─────────────────────────────────────────────────────────────

    function assertAttrsPresent(testId) {
        cy.findByTestId(testId).should('have.class', 'test-class');
        cy.findByTestId(testId).should('have.attr', 'data-custom', 'test-value');
        cy.findByTestId(testId).should('have.attr', 'aria-label', 'test-label');
    }

    function assertAttrsAbsent(testId) {
        cy.findByTestId(testId).should('not.have.class', 'test-class');
        cy.findByTestId(testId).should('not.have.attr', 'data-custom');
        cy.findByTestId(testId).should('not.have.attr', 'aria-label');
    }

    // ── 1. External Show Baseline ───────────────────────────────────────────
    //
    // Parent Show wraps the entire component. When Show cycles, the component
    // is fully destroyed and recreated, so AttributeInterceptor runs fresh.

    describe('external Show baseline', () => {
        it('attrs are present on initial render', () => {
            assertAttrsPresent('broken-target');
        });

        it('attrs survive Show cycling (hide → show)', () => {
            // Hide — component destroyed
            cy.findByTestId('broken-toggle').click();
            cy.findByTestId('broken-target').should('not.exist');

            // Show — component recreated from scratch
            cy.findByTestId('broken-toggle').click();
            cy.findByTestId('broken-target').should('exist');

            assertAttrsPresent('broken-target');
        });
    });

    // ── 2. StoredValue + Effect Workaround (external Show) ──────────────────

    describe('StoredValue + Effect workaround (external Show)', () => {
        it('attrs are present on initial render', () => {
            assertAttrsPresent('effect-target');
        });

        it('attrs survive Show cycling (hide → show)', () => {
            cy.findByTestId('effect-toggle').click();
            cy.findByTestId('effect-target').should('not.exist');

            cy.findByTestId('effect-toggle').click();
            cy.findByTestId('effect-target').should('exist');

            assertAttrsPresent('effect-target');
        });
    });

    // ── 3. Wrong-Element Problem ────────────────────────────────────────────
    //
    // CONTEXT: Spreading {..attrs} inside Show does NOT COMPILE because Show
    // requires ChildrenFn but the spread produces an incompatible view type.
    // (See fixture 3 comments in presence_attrs.rs for the full compiler error.)
    //
    // The fallback is to spread attrs on a wrapper OUTSIDE Show. This compiles
    // and the attrs survive cycling — but they're on the WRONG ELEMENT.
    // The target inside Show gets nothing.

    describe('wrong-element problem (attrs on wrapper, not target)', () => {
        it('attrs are on the wrapper element', () => {
            assertAttrsPresent('wrapper-with-attrs');
        });

        it('target inside Show has NO attrs', () => {
            assertAttrsAbsent('target-without-attrs');
        });

        it('wrapper attrs survive Show cycling', () => {
            // Wrapper stays mounted; only target inside Show cycles
            cy.findByTestId('wrapper-toggle').click();
            cy.findByTestId('target-without-attrs').should('not.exist');

            cy.findByTestId('wrapper-toggle').click();
            cy.findByTestId('target-without-attrs').should('exist');

            // Wrapper still has attrs (it was never unmounted)
            assertAttrsPresent('wrapper-with-attrs');
            // Target still has no attrs
            assertAttrsAbsent('target-without-attrs');
        });
    });

    // ── 4. Internal Show + Effect Workaround ────────────────────────────────
    //
    // The component owns its own Show and uses the StoredValue+Effect pattern
    // to apply attrs to the target element inside Show via node_ref.
    // This is the real-world pattern used by MenuContent.

    describe('internal Show + Effect workaround', () => {
        it('attrs are present on initial render', () => {
            assertAttrsPresent('internal-effect-target');
        });

        it('attrs survive internal Show cycling (hide → show)', () => {
            cy.findByTestId('internal-effect-toggle').click();
            cy.findByTestId('internal-effect-target').should('not.exist');

            cy.findByTestId('internal-effect-toggle').click();
            cy.findByTestId('internal-effect-target').should('exist');

            assertAttrsPresent('internal-effect-target');
        });

        it('attrs survive multiple Show cycles', () => {
            for (let i = 0; i < 3; i++) {
                cy.findByTestId('internal-effect-toggle').click();
                cy.findByTestId('internal-effect-target').should('not.exist');

                cy.findByTestId('internal-effect-toggle').click();
                cy.findByTestId('internal-effect-target').should('exist');
                assertAttrsPresent('internal-effect-target');
            }
        });
    });

    // ── 5. Reactive Attrs + Show Cycling ────────────────────────────────────
    //
    // Tests the LIMITATION of the StoredValue+Effect workaround: extract_attrs
    // flattens reactive closures to static strings at extraction time. After
    // that, signal updates cannot propagate to the stored attrs.

    describe('reactive attrs + Show cycling (Effect workaround)', () => {
        it('reactive attr has initial value', () => {
            cy.findByTestId('reactive-show-target').should('have.attr', 'data-count', '0');
        });

        // SKIP: extract_attrs() flattens reactive closures to static strings at
        // extraction time. The closure `move || count.get().to_string()` is evaluated
        // ONCE when extract_attrs builds attrs on the temp element, producing "0".
        // After that, StoredValue holds the frozen string — signal updates never propagate.
        // This is a fundamental limitation of the StoredValue+Effect workaround.
        it.skip('FAILS: reactive attr updates before cycling — extract_attrs freezes reactive values', () => {
            cy.findByTestId('reactive-show-increment').click();
            cy.findByTestId('reactive-show-target').should('have.attr', 'data-count', '1');
        });

        // SKIP: Same root cause. After increment, StoredValue still holds "0" (the value
        // at extraction time). The Effect re-applies "0" on re-mount, not the current signal value.
        it.skip('FAILS: reactive attr value persists after Show cycling — frozen at extraction time', () => {
            cy.findByTestId('reactive-show-increment').click();
            cy.findByTestId('reactive-show-target').should('have.attr', 'data-count', '1');

            cy.findByTestId('reactive-show-toggle').click();
            cy.findByTestId('reactive-show-target').should('not.exist');
            cy.findByTestId('reactive-show-toggle').click();
            cy.findByTestId('reactive-show-target').should('exist');

            cy.findByTestId('reactive-show-target').should('have.attr', 'data-count');
        });

        // SKIP: Same root cause. Post-cycle signal updates cannot reach the element because
        // the Effect applies frozen StoredValue attrs, and no reactive subscription exists
        // between the signal and the DOM attribute.
        it.skip('FAILS: reactive attr updates AFTER Show cycling — no reactive subscription survives extract_attrs', () => {
            cy.findByTestId('reactive-show-toggle').click();
            cy.findByTestId('reactive-show-target').should('not.exist');
            cy.findByTestId('reactive-show-toggle').click();
            cy.findByTestId('reactive-show-target').should('exist');

            cy.findByTestId('reactive-show-increment').click();
            cy.findByTestId('reactive-show-target').should('have.attr', 'data-count', '1');
        });
    });
});
