// ── Experiment: Spreadable ForwardedAttrs ────────────────────────────────────
//
// Tests ForwardedAttrs.spread() used with {..} syntax:
// 1. Can spread multiple times (multi-target)
// 2. Maintains reactivity (signal-driven attrs update)
// 3. Survives Show rebuild (mount/unmount cycling)
//
// Failing tests are SKIPPED with explanations, never deleted.

describe('Spreadable ForwardedAttrs', () => {
    beforeEach(() => {
        cy.visit('/spreadable-attrs');
    });

    // ── Helpers ─────────────────────────────────────────────────────────────

    function assertStaticAttrs(testId) {
        cy.findByTestId(testId).should('have.class', 'test-class');
        cy.findByTestId(testId).should('have.attr', 'data-custom', 'test-value');
        cy.findByTestId(testId).should('have.attr', 'aria-label', 'test-label');
    }

    // ── 1. Basic spread outside Show ────────────────────────────────────────

    describe('basic spread (outside Show)', () => {
        it('attrs are present on initial render', () => {
            assertStaticAttrs('basic-target');
        });
    });

    // ── 2. Spread inside Show ───────────────────────────────────────────────

    describe('spread inside Show', () => {
        it('attrs are present on initial render', () => {
            assertStaticAttrs('show-target');
        });

        it('attrs survive Show cycling (hide → show)', () => {
            cy.findByTestId('show-toggle').click();
            cy.findByTestId('show-target').should('not.exist');

            cy.findByTestId('show-toggle').click();
            cy.findByTestId('show-target').should('exist');
            assertStaticAttrs('show-target');
        });

        it('attrs survive multiple Show cycles', () => {
            for (let i = 0; i < 3; i++) {
                cy.findByTestId('show-toggle').click();
                cy.findByTestId('show-target').should('not.exist');

                cy.findByTestId('show-toggle').click();
                cy.findByTestId('show-target').should('exist');
                assertStaticAttrs('show-target');
            }
        });
    });

    // ── 3. Multiple spreads + reactive attrs ────────────────────────────────

    describe('multiple spreads + reactive attrs', () => {
        it('both targets have static and reactive attrs on initial render', () => {
            cy.findByTestId('multi-a').should('have.class', 'shared-class');
            cy.findByTestId('multi-b').should('have.class', 'shared-class');
            cy.findByTestId('multi-a').should('have.attr', 'data-count', '0');
            cy.findByTestId('multi-b').should('have.attr', 'data-count', '0');
        });

        it('both targets update reactively in sync', () => {
            cy.findByTestId('multi-increment').click();
            cy.findByTestId('multi-a').should('have.attr', 'data-count', '1');
            cy.findByTestId('multi-b').should('have.attr', 'data-count', '1');
        });

        it('both targets survive Show cycling with reactivity', () => {
            cy.findByTestId('multi-increment').click();

            cy.findByTestId('multi-toggle').click();
            cy.findByTestId('multi-a').should('not.exist');
            cy.findByTestId('multi-toggle').click();

            cy.findByTestId('multi-a').should('have.attr', 'data-count', '1');
            cy.findByTestId('multi-b').should('have.attr', 'data-count', '1');

            // Still reactive after cycling
            cy.findByTestId('multi-increment').click();
            cy.findByTestId('multi-a').should('have.attr', 'data-count', '2');
            cy.findByTestId('multi-b').should('have.attr', 'data-count', '2');
        });
    });

    // ── 4. Reactive attrs + Show cycling ────────────────────────────────────

    describe('reactive attrs + Show cycling', () => {
        it('reactive attr has initial value', () => {
            cy.findByTestId('reactive-target').should('have.attr', 'data-count', '0');
        });

        it('reactive attr updates when signal changes', () => {
            cy.findByTestId('reactive-increment').click();
            cy.findByTestId('reactive-target').should('have.attr', 'data-count', '1');
        });

        it('reactive attr persists latest value after Show cycling', () => {
            cy.findByTestId('reactive-increment').click();
            cy.findByTestId('reactive-increment').click();
            cy.findByTestId('reactive-target').should('have.attr', 'data-count', '2');

            cy.findByTestId('reactive-toggle').click();
            cy.findByTestId('reactive-target').should('not.exist');
            cy.findByTestId('reactive-toggle').click();

            cy.findByTestId('reactive-target').should('have.attr', 'data-count', '2');
        });

        it('reactive attr updates AFTER Show cycling', () => {
            cy.findByTestId('reactive-toggle').click();
            cy.findByTestId('reactive-target').should('not.exist');
            cy.findByTestId('reactive-toggle').click();
            cy.findByTestId('reactive-target').should('exist');

            cy.findByTestId('reactive-increment').click();
            cy.findByTestId('reactive-target').should('have.attr', 'data-count', '1');
        });

        it('survives multiple cycles with interleaved updates', () => {
            cy.findByTestId('reactive-increment').click();
            cy.findByTestId('reactive-target').should('have.attr', 'data-count', '1');

            cy.findByTestId('reactive-toggle').click();
            cy.findByTestId('reactive-toggle').click();
            cy.findByTestId('reactive-target').should('have.attr', 'data-count', '1');

            cy.findByTestId('reactive-increment').click();
            cy.findByTestId('reactive-target').should('have.attr', 'data-count', '2');

            cy.findByTestId('reactive-toggle').click();
            cy.findByTestId('reactive-toggle').click();
            cy.findByTestId('reactive-target').should('have.attr', 'data-count', '2');

            cy.findByTestId('reactive-increment').click();
            cy.findByTestId('reactive-target').should('have.attr', 'data-count', '3');
        });
    });
});
