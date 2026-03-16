// ── Experiment: Class Override Order with {..} Spread ────────────────────────
//
// Tests whether `attr:class` (setAttribute) clobbers `class:` directives
// (classList.toggle), mirroring the style-override experiment for classes.
//
// Key question: Do classes behave the same as styles w.r.t. clobbering?

describe('Class Override Order', () => {
    beforeEach(() => {
        cy.visit('/class-override');
    });

    // ── 1. class: directive first, then spread with attr:class ───────────
    //
    // Internal: class:internal-class=true (before spread)
    // User: attr:class="user-class" (through spread)
    //
    // If class behaves like style: attr:class would clobber class: directive.
    // If class uses classList instead: both should coexist.

    describe('class: directive first, then spread (user attr:class)', () => {
        it('user class is present', () => {
            cy.get('[data-testid="class-directive-then-spread"]')
                .should('have.class', 'user-class');
        });

        // RESULT: attr:class DOES clobber class: directives when class: is before
        // the spread. Same behavior as attr:style — setAttribute replaces the entire
        // class attribute, wiping classList.toggle'd classes set earlier.
        it('internal class IS clobbered by attr:class (same as attr:style)', () => {
            cy.get('[data-testid="class-directive-then-spread"]')
                .should('not.have.class', 'internal-class');
        });
    });

    // ── 2. spread first, then class: directive (user attr:class) ─────────
    //
    // Spread (with attr:class) first, then class:internal-class=true.
    // If class: is applied after attr:class, it should survive regardless.

    describe('spread first, then class: directive (user attr:class)', () => {
        it('user class is present', () => {
            cy.get('[data-testid="spread-then-class-directive"]')
                .should('have.class', 'user-class');
        });

        it('internal class after spread should survive', () => {
            cy.get('[data-testid="spread-then-class-directive"]')
                .should('have.class', 'internal-class');
        });
    });

    // ── 3. Both sides use class: directives ──────────────────────────────
    //
    // Internal: class:internal-class=true
    // User: class:user-class=true (not attr:class)
    // Both use classList.toggle — should be fully additive.

    describe('both sides use class: directives', () => {
        it('internal class is present', () => {
            cy.get('[data-testid="class-directive-both"]')
                .should('have.class', 'internal-class');
        });

        it('user class is present', () => {
            cy.get('[data-testid="class-directive-both"]')
                .should('have.class', 'user-class');
        });
    });

    // ── 4. Multiple internal class: + user attr:class ────────────────────
    //
    // Internal: class:internal-a=true, class:internal-b=true
    // User: attr:class="user-class"
    // Tests whether attr:class clobbers ALL class: directives.

    describe('multiple internal class: + user attr:class', () => {
        it('user class is present', () => {
            cy.get('[data-testid="multi-class-vs-attr"]')
                .should('have.class', 'user-class');
        });

        // RESULT: attr:class clobbers ALL class: directives that come before the spread.
        it('internal-a IS clobbered by attr:class', () => {
            cy.get('[data-testid="multi-class-vs-attr"]')
                .should('not.have.class', 'internal-a');
        });

        it('internal-b IS clobbered by attr:class', () => {
            cy.get('[data-testid="multi-class-vs-attr"]')
                .should('not.have.class', 'internal-b');
        });
    });

    // ── 5. Vanilla interceptor (no ForwardedAttrs) ───────────────────────
    //
    // Direct {..attrs} spread + class: directive.
    // User passes class:user-class=true.

    describe('vanilla interceptor + class: directives', () => {
        it('internal class is present', () => {
            cy.get('[data-testid="vanilla-interceptor-class"]')
                .should('have.class', 'internal-class');
        });

        it('user class is present', () => {
            cy.get('[data-testid="vanilla-interceptor-class"]')
                .should('have.class', 'user-class');
        });
    });

    // ── 6. Native Leptos spreading (no interceptor) ──────────────────────
    //
    // No interceptor, Leptos native attr propagation.
    // User passes class:user-class=true.

    describe('native Leptos class spreading', () => {
        it('internal class is present', () => {
            cy.get('[data-testid="native-spread-class"]')
                .should('have.class', 'internal-class');
        });

        it('user class is present', () => {
            cy.get('[data-testid="native-spread-class"]')
                .should('have.class', 'user-class');
        });
    });

    // ── 7. No conflict (user passes non-class attr) ─────────────────────

    describe('no conflict (user passes non-class attr)', () => {
        it('internal class is preserved', () => {
            cy.get('[data-testid="no-class-conflict"]')
                .should('have.class', 'internal-class');
        });

        it('data-custom is present', () => {
            cy.get('[data-testid="no-class-conflict"]')
                .should('have.attr', 'data-custom', 'hello');
        });
    });

    // ── 8. class: after spread, user also class: (both additive) ─────────

    describe('class: after spread, user also class:', () => {
        it('internal class is present', () => {
            cy.get('[data-testid="directive-after-spread-user-directive"]')
                .should('have.class', 'internal-class');
        });

        it('user class is present', () => {
            cy.get('[data-testid="directive-after-spread-user-directive"]')
                .should('have.class', 'user-class');
        });
    });
});
