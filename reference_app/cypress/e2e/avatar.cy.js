describe('Avatar', () => {
    // ── Helpers ──────────────────────────────────────────────

    function getAvatar(name) {
        return cy.findByTestId(`avatar-${name}`);
    }

    function getStatusDisplay(name) {
        return cy.findByTestId(`status-${name}`);
    }

    beforeEach(() => {
        cy.visit('/avatar');
    });

    // ── 1. Rendering ────────────────────────────────────────

    describe('rendering', () => {
        it('renders the root as a span', () => {
            getAvatar('working').should('exist');
            getAvatar('working').should('have.prop', 'tagName', 'SPAN');
        });

        it('renders the fallback as a span', () => {
            getAvatar('no-image').find('span').should('exist');
        });
    });

    // ── 2. Image Loading ────────────────────────────────────

    describe('image loading', () => {
        it('shows image when src loads successfully', () => {
            getAvatar('working').find('img').should('exist');
        });

        it('hides fallback when image loads successfully', () => {
            getAvatar('working').find('img').should('exist');
            getAvatar('working').should('not.contain.text', 'WI');
        });

        it('fires onLoadingStatusChange with "loaded" for working image', () => {
            getStatusDisplay('working').should('have.text', 'loaded');
        });
    });

    // ── 3. Fallback Behavior ────────────────────────────────

    describe('fallback', () => {
        it('shows fallback when no image src is provided', () => {
            getAvatar('no-image').should('contain.text', 'NI');
        });

        it('does not render an img when no src is provided', () => {
            getAvatar('no-image').find('img').should('not.exist');
        });

        it('shows fallback when image src fails to load', () => {
            getAvatar('broken').should('contain.text', 'BI');
        });

        it('fires onLoadingStatusChange with "error" for broken image', () => {
            getStatusDisplay('broken').should('have.text', 'error');
        });

        it('does not render an img when src fails to load', () => {
            getAvatar('broken').find('img').should('not.exist');
        });
    });

    // ── 4. Delayed Fallback ─────────────────────────────────

    describe('delayed fallback', () => {
        it('shows fallback after delay when image fails', () => {
            // The delayed avatar has a 300ms delay on its fallback
            getAvatar('delayed').should('contain.text', 'DI');
        });

        it('does not show delayed fallback immediately', () => {
            // Visit fresh to catch the initial state before delay
            cy.visit('/avatar');
            // Immediately after load, the delayed fallback should NOT be visible
            getAvatar('delayed').should('not.contain.text', 'DI');
            // After the delay elapses, the fallback should appear
            getAvatar('delayed').should('contain.text', 'DI');
        });
    });

    // ── 5. Dynamic Source Changes ───────────────────────────

    describe('dynamic source', () => {
        it('initially shows fallback when no src is set', () => {
            getAvatar('dynamic').should('contain.text', 'DY');
            getAvatar('dynamic').find('img').should('not.exist');
        });

        it('shows image after setting a valid src', () => {
            cy.findByTestId('set-working-src').click();
            getAvatar('dynamic').find('img').should('exist');
            getAvatar('dynamic').should('not.contain.text', 'DY');
        });

        it('shows fallback after setting a broken src', () => {
            cy.findByTestId('set-working-src').click();
            getAvatar('dynamic').find('img').should('exist');
            cy.findByTestId('set-broken-src').click();
            getAvatar('dynamic').find('img').should('not.exist');
            getAvatar('dynamic').should('contain.text', 'DY');
        });

        it('shows fallback after clearing src', () => {
            cy.findByTestId('set-working-src').click();
            getAvatar('dynamic').find('img').should('exist');
            cy.findByTestId('clear-src').click();
            getAvatar('dynamic').find('img').should('not.exist');
            getAvatar('dynamic').should('contain.text', 'DY');
        });
    });

    // ── Attribute Forwarding ────────────────────────────────

    describe('attribute forwarding', () => {
        it('Root forwards className/class and custom data attributes', () => {
            getAvatar('no-image')
                .should('have.class', 'avatar-root')
                .and('have.attr', 'data-custom', 'avatar-root-custom');
        });

        it('Fallback forwards className/class and custom data attributes', () => {
            getAvatar('no-image')
                .find('[data-custom="avatar-fallback-custom"]')
                .should('exist')
                .and('have.class', 'avatar-fallback');
        });
    });
});
