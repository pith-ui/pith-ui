// ── Experiment: NavigationMenu Reactive Attr Forwarding ──────────────────────
//
// Tests whether reactive attrs (signal-driven) on NavigationMenuContent
// update when rendered through a Viewport.
//
// The hypothesis is that extract_attrs freezes reactive values. If so,
// data-count will be "0" even after incrementing.
//
// Failing tests are SKIPPED with explanations, never deleted.

describe('NavigationMenu Reactive Attrs', () => {
    beforeEach(() => {
        cy.visit('/nav-menu-reactive-attrs');
    });

    // ── Helpers ─────────────────────────────────────────────────────────────

    function openItemA() {
        cy.findByTestId('trigger-a').realHover();
        cy.findByTestId('content-a').should('exist');
    }

    function closeMenu() {
        cy.findByTestId('outside').realClick();
        cy.findByTestId('content-a').should('not.exist');
    }

    // ── Static attrs work ───────────────────────────────────────────────────

    describe('static attrs through viewport', () => {
        it('data-testid is forwarded to content', () => {
            openItemA();
            cy.findByTestId('content-a').should('exist');
        });

        it('class is forwarded to content', () => {
            openItemA();
            cy.findByTestId('content-a').should('have.class', 'nav-content');
        });
    });

    // ── Reactive attrs ──────────────────────────────────────────────────────

    describe('reactive attrs through viewport', () => {
        it('reactive attr has initial value when content opens', () => {
            openItemA();
            cy.findByTestId('content-a').should('have.attr', 'data-count', '0');
        });

        // SKIP: extract_attrs() in NavigationMenuContent flattens reactive closures to
        // static strings at extraction time. The viewport path stores these frozen strings
        // in ContentData.extra_attrs. Signal updates after extraction are lost.
        // React handles this by re-registering ContentData in useLayoutEffect when props
        // change, but our extract_attrs approach doesn't re-run on signal changes.
        it.skip('FAILS: reactive attr updates while content is open — extract_attrs freezes reactive values', () => {
            openItemA();
            cy.findByTestId('content-a').should('have.attr', 'data-count', '0');
            cy.findByTestId('increment').click({force: true});
            cy.findByTestId('content-a').should('have.attr', 'data-count', '1');
        });

        it.skip('FAILS: reactive attr reflects latest value on re-open — frozen at extraction time', () => {
            cy.findByTestId('increment').click();
            cy.findByTestId('increment').click();
            openItemA();
            cy.findByTestId('content-a').should('have.attr', 'data-count', '2');
        });

        it.skip('FAILS: reactive attr updates after close and re-open — no re-extraction on signal change', () => {
            openItemA();
            cy.findByTestId('content-a').should('have.attr', 'data-count', '0');
            closeMenu();
            cy.findByTestId('increment').click();
            openItemA();
            cy.findByTestId('content-a').should('have.attr', 'data-count', '1');
        });
    });
});
