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

    // ── 1. Accessibility Semantics ──────────────────────────

    describe('accessibility', () => {
        it('Trigger renders as an anchor element', () => {
            cy.findByTestId('hover-card-trigger').should('have.prop', 'tagName', 'A');
        });

        it('Trigger has href attribute (link semantics)', () => {
            cy.findByTestId('hover-card-trigger').should('have.attr', 'href');
        });
    });

    // ── 2. Data Attributes ──────────────────────────────────

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

    // ── Controlled Mode ────────────────────────────────────

    describe('controlled mode', () => {
        beforeEach(() => {
            cy.visit('/hover-card');
            cy.findByLabelText('controlled').click({force: true});
        });

        it('opens via external state', () => {
            cy.findByTestId('controlled-content').should('not.exist');
            cy.findByTestId('open-controlled').click({force: true});
            cy.findByTestId('controlled-content').should('exist');
        });

        it('closes via external state', () => {
            cy.findByTestId('open-controlled').click({force: true});
            cy.findByTestId('controlled-content').should('exist');
            cy.findByTestId('close-controlled').click({force: true});
            cy.findByTestId('controlled-content').should('not.exist');
        });

        it('on_open_change callback fires on hover', () => {
            cy.findByTestId('controlled-state').should('have.text', 'closed');
            cy.findByTestId('controlled-trigger').realHover();
            cy.findByTestId('controlled-state').should('have.text', 'open');
        });
    });

    // ── Delay Timing ──────────────────────────────────────────

    describe('delay timing', () => {
        // hover-card-bp-2

        it('delayed hover card does not open immediately on hover', () => {
            cy.findByTestId('delayed-trigger').realHover();
            // With 500ms openDelay, content should NOT be visible immediately
            cy.findByTestId('delayed-content').should('not.exist');
        });

        it('delayed hover card opens after openDelay', () => {
            cy.findByTestId('delayed-trigger').realHover();
            // Should eventually open after the 500ms delay
            cy.findByTestId('delayed-content', {timeout: 5000}).should('exist');
        });

        it('delayed hover card does not close immediately on leave', () => {
            cy.findByTestId('delayed-trigger').realHover();
            cy.findByTestId('delayed-content', {timeout: 5000}).should('exist');
            // Leave the trigger
            pointerLeave('delayed-trigger');
            // With 300ms closeDelay, content should still exist briefly
            // (this assertion runs immediately, before the 300ms elapses)
            cy.findByTestId('delayed-content').should('exist');
        });

        it('delayed hover card closes after closeDelay', () => {
            cy.findByTestId('delayed-trigger').realHover();
            cy.findByTestId('delayed-content', {timeout: 5000}).should('exist');
            pointerLeave('delayed-trigger');
            // Should eventually close after the 300ms delay
            cy.findByTestId('delayed-content', {timeout: 5000}).should('not.exist');
        });
    });

    // ── Internal Styles ─────────────────────────────────────

    describe('internal styles', () => {
        it('content has --radix-hover-card-content-available-width', () => {
            cy.findByTestId('hover-card-trigger').realHover();
            shouldBeOpen();
            cy.findByTestId('hover-card-content').then(($el) => {
                const value = getComputedStyle($el[0]).getPropertyValue(
                    '--radix-hover-card-content-available-width'
                );
                expect(value.trim()).to.not.be.empty;
            });
        });

        it('content has --radix-hover-card-content-available-height', () => {
            cy.findByTestId('hover-card-trigger').realHover();
            shouldBeOpen();
            cy.findByTestId('hover-card-content').then(($el) => {
                const value = getComputedStyle($el[0]).getPropertyValue(
                    '--radix-hover-card-content-available-height'
                );
                expect(value.trim()).to.not.be.empty;
            });
        });
    });
});
