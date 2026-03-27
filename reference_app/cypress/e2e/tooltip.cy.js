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

    // ── 7. ARIA Attributes ────────────────────────────────────

    describe('ARIA attributes', () => {
        it('trigger has aria-describedby pointing to content when open', () => {
            // tooltip-bp-1
            trigger().focus();
            shouldBeOpen();
            trigger().should('have.attr', 'aria-describedby').and('not.be.empty');
        });

        it('trigger does not have aria-describedby when closed', () => {
            // tooltip-bp-1
            trigger().should('not.have.attr', 'aria-describedby');
        });

        it('a role="tooltip" element exists in the DOM when open', () => {
            // tooltip-bp-2
            trigger().focus();
            shouldBeOpen();
            cy.get('[role="tooltip"]').should('exist');
        });

        it('no role="tooltip" element exists when closed', () => {
            // tooltip-bp-2
            cy.get('[role="tooltip"]').should('not.exist');
        });
    });

    // ── 8. Controlled Mode ────────────────────────────────────

    describe('controlled mode', () => {
        it('external checkbox opens tooltip', () => {
            // tooltip-msc-1
            cy.findByTestId('tooltip-content-controlled').should('not.exist');
            cy.findByLabelText('open controlled').click();
            cy.findByTestId('tooltip-content-controlled').should('exist');
        });

        it('external control closes tooltip', () => {
            // tooltip-msc-1
            cy.findByLabelText('open controlled').click();
            cy.findByTestId('tooltip-content-controlled').should('exist');
            // Use dedicated button to avoid click-outside dismiss racing with checkbox toggle
            cy.findByTestId('controlled-external-close').click();
            cy.findByTestId('tooltip-content-controlled').should('not.exist');
            cy.findByTestId('controlled-open-state').should('have.text', 'closed');
        });

        it('hovering controlled trigger updates external checkbox', () => {
            // tooltip-msc-1
            cy.findByLabelText('open controlled').should('not.be.checked');
            cy.findByTestId('tooltip-trigger-controlled').realHover();
            cy.findByTestId('tooltip-content-controlled').should('exist');
            cy.findByLabelText('open controlled').should('be.checked');
        });
    });

    // ── Axe Accessibility Audit ─────────────────────────────

    describe('axe audit', () => {
        it('no violations in closed state', () => {
            cy.checkComponentA11y();
        });

        it('no violations when open', () => {
            trigger(1).realHover();
            shouldBeOpen(1);
            cy.checkComponentA11y();
        });
    });
});
