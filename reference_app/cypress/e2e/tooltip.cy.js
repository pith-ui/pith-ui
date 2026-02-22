describe('Tooltip', () => {
    // ── Helpers ──────────────────────────────────────────────

    function shouldBeOpen(n = 1) {
        cy.findByTestId(`tooltip-content-${n}`).should('exist');
    }

    function shouldBeClosed(n = 1) {
        cy.findByTestId(`tooltip-content-${n}`).should('not.exist');
    }

    function trigger(n = 1) {
        return cy.findByTestId(`tooltip-trigger-${n}`);
    }

    function content(n = 1) {
        return cy.findByTestId(`tooltip-content-${n}`);
    }

    beforeEach(() => {
        cy.visit('/tooltip');
    });

    // ── 1. Data Attributes ──────────────────────────────────

    describe('data attributes', () => {
        it('Trigger has data-state="closed" initially', () => {
            trigger().should('have.attr', 'data-state', 'closed');
        });

        it('Trigger data-state changes on hover', () => {
            trigger().realHover();
            trigger().invoke('attr', 'data-state').should('match', /open/);
        });

        it('Content has data-state when visible', () => {
            trigger().realHover();
            shouldBeOpen();
            content().invoke('attr', 'data-state').should('match', /open/);
        });

        it('Content has data-side attribute', () => {
            trigger().realHover();
            shouldBeOpen();
            content().should('have.attr', 'data-side');
        });

        it('Content has data-align attribute', () => {
            trigger().realHover();
            shouldBeOpen();
            content().should('have.attr', 'data-align');
        });

        it('Trigger data-state returns to "closed" when tooltip dismissed', () => {
            // Open via focus (reliable), close via Escape
            trigger().focus();
            trigger().invoke('attr', 'data-state').should('match', /open/);
            cy.realPress('Escape');
            trigger().should('have.attr', 'data-state', 'closed');
        });
    });

    // ── 2. Keyboard Navigation ──────────────────────────────

    describe('keyboard navigation', () => {
        it('Tab to trigger opens tooltip', () => {
            shouldBeClosed();
            trigger().focus();
            shouldBeOpen();
        });

        it('Escape closes tooltip', () => {
            trigger().focus();
            shouldBeOpen();
            cy.realPress('Escape');
            shouldBeClosed();
        });

        it('Tab away closes tooltip', () => {
            trigger().focus();
            shouldBeOpen();
            cy.realPress('Tab');
            shouldBeClosed();
        });
    });

    // ── 3. Pointer Interaction ──────────────────────────────

    describe('pointer interaction', () => {
        it('hover opens tooltip', () => {
            shouldBeClosed();
            trigger().realHover();
            shouldBeOpen();
        });

        it('Escape closes hover-opened tooltip', () => {
            trigger().realHover();
            shouldBeOpen();
            cy.realPress('Escape');
            shouldBeClosed();
        });

        it('moving from trigger to content keeps tooltip open', () => {
            trigger().realHover();
            shouldBeOpen();
            content().realHover();
            shouldBeOpen();
        });
    });

    // ── 4. Focus Interaction ────────────────────────────────

    describe('focus interaction', () => {
        it('focusing trigger opens tooltip', () => {
            shouldBeClosed();
            trigger().focus();
            shouldBeOpen();
        });

        it('blurring trigger closes tooltip', () => {
            trigger().focus();
            shouldBeOpen();
            trigger().blur();
            shouldBeClosed();
        });
    });

    // ── 5. Skip Delay ───────────────────────────────────────

    describe('skip delay', () => {
        it('second tooltip opens when focus moves from first trigger to second', () => {
            trigger(1).focus();
            shouldBeOpen(1);
            trigger(2).focus();
            shouldBeOpen(2);
        });
    });

    // ── 6. Content Positioning ──────────────────────────────

    describe('content positioning', () => {
        it('Content has data-side="bottom" by default', () => {
            trigger().realHover();
            shouldBeOpen();
            content().should('have.attr', 'data-side', 'bottom');
        });

        it('Content has data-align="center" by default', () => {
            trigger().realHover();
            shouldBeOpen();
            content().should('have.attr', 'data-align', 'center');
        });
    });
});
