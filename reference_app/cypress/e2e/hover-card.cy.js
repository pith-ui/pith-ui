describe('Hover Card', () => {
    // ── Helpers ──────────────────────────────────────────────

    function shouldBeOpen() {
        cy.findByTestId('hover-card-content').should('exist');
    }

    function shouldBeClosed() {
        cy.findByTestId('hover-card-content').should('not.exist');
    }

    // Fire pointerleave on a given element. realHover() on another element
    // doesn't reliably fire pointerleave in headless Electron, so we
    // dispatch the events natively.
    function pointerLeave(testId) {
        cy.findByTestId(testId).then(($el) => {
            const target = $el[0];
            target.dispatchEvent(
                new PointerEvent('pointerout', {bubbles: true, pointerType: 'mouse'})
            );
            target.dispatchEvent(
                new PointerEvent('pointerleave', {bubbles: false, pointerType: 'mouse'})
            );
        });
    }

    beforeEach(() => {
        cy.visit('/hover-card');
    });

    // ── 1. Data Attributes ──────────────────────────────────

    describe('data attributes', () => {
        it('Trigger has data-state="closed" initially', () => {
            cy.findByTestId('hover-card-trigger').should('have.attr', 'data-state', 'closed');
        });

        it('Trigger data-state becomes "open" on hover', () => {
            cy.findByTestId('hover-card-trigger').realHover();
            cy.findByTestId('hover-card-trigger').should('have.attr', 'data-state', 'open');
        });

        it('Content has data-state="open" when visible', () => {
            cy.findByTestId('hover-card-trigger').realHover();
            shouldBeOpen();
            cy.findByTestId('hover-card-content').should('have.attr', 'data-state', 'open');
        });

        it('Content has data-side attribute', () => {
            cy.findByTestId('hover-card-trigger').realHover();
            shouldBeOpen();
            cy.findByTestId('hover-card-content').should('have.attr', 'data-side');
        });

        it('Content has data-align attribute', () => {
            cy.findByTestId('hover-card-trigger').realHover();
            shouldBeOpen();
            cy.findByTestId('hover-card-content').should('have.attr', 'data-align');
        });

        it('Trigger data-state returns to "closed" after mouse leaves', () => {
            cy.findByTestId('hover-card-trigger').realHover();
            cy.findByTestId('hover-card-trigger').should('have.attr', 'data-state', 'open');
            pointerLeave('hover-card-trigger');
            cy.findByTestId('hover-card-trigger').should('have.attr', 'data-state', 'closed');
        });
    });

    // ── 2. Pointer Interaction ──────────────────────────────

    describe('pointer interaction', () => {
        it('hovering trigger opens content', () => {
            shouldBeClosed();
            cy.findByTestId('hover-card-trigger').realHover();
            shouldBeOpen();
        });

        it('moving mouse away from trigger closes content', () => {
            cy.findByTestId('hover-card-trigger').realHover();
            shouldBeOpen();
            pointerLeave('hover-card-trigger');
            shouldBeClosed();
        });

        it('moving mouse from trigger to content keeps it open', () => {
            cy.findByTestId('hover-card-trigger').realHover();
            shouldBeOpen();
            cy.findByTestId('hover-card-content').realHover();
            // Content should remain open while mouse is over it
            shouldBeOpen();
        });

        it('moving mouse away from content closes it', () => {
            cy.findByTestId('hover-card-trigger').realHover();
            shouldBeOpen();
            cy.findByTestId('hover-card-content').realHover();
            shouldBeOpen();
            pointerLeave('hover-card-content');
            shouldBeClosed();
        });
    });

    // ── 3. Content Positioning ──────────────────────────────

    describe('content positioning', () => {
        it('Content has data-side="bottom" by default', () => {
            cy.findByTestId('hover-card-trigger').realHover();
            shouldBeOpen();
            cy.findByTestId('hover-card-content').should('have.attr', 'data-side', 'bottom');
        });

        it('Content has data-align="center" by default', () => {
            cy.findByTestId('hover-card-trigger').realHover();
            shouldBeOpen();
            cy.findByTestId('hover-card-content').should('have.attr', 'data-align', 'center');
        });
    });
});
