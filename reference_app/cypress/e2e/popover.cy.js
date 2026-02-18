describe('Popover', () => {
    // ── Helpers ──────────────────────────────────────────────

    function shouldBeOpen() {
        cy.findByRole('dialog').should('exist');
    }

    function shouldBeClosed() {
        cy.findByRole('dialog').should('not.exist');
    }

    beforeEach(() => {
        cy.visit('/popover');
    });

    // ── 1. Accessibility Semantics ──────────────────────────

    describe('accessibility', () => {
        it('Content has role="dialog"', () => {
            cy.findByText('open').click();
            cy.findByRole('dialog').should('exist');
        });

        it('Trigger has aria-haspopup="dialog"', () => {
            cy.findByText('open').should('have.attr', 'aria-haspopup', 'dialog');
        });

        it('Trigger aria-expanded reflects open state', () => {
            cy.findByText('open').should('have.attr', 'aria-expanded', 'false');
            cy.findByText('open').click();
            shouldBeOpen();
            cy.findByText('open').should('have.attr', 'aria-expanded', 'true');
        });

        it('Trigger aria-controls references Content id', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.findByText('open')
                .invoke('attr', 'aria-controls')
                .then((controlsId) => {
                    cy.findByRole('dialog').should('have.attr', 'id', controlsId);
                });
        });
    });

    // ── 2. Data Attributes ──────────────────────────────────

    describe('data attributes', () => {
        it('Trigger data-state reflects open state', () => {
            cy.findByText('open').should('have.attr', 'data-state', 'closed');
            cy.findByText('open').click();
            shouldBeOpen();
            cy.findByText('open').should('have.attr', 'data-state', 'open');
            cy.findByRole('button', {name: 'close'}).click();
            shouldBeClosed();
            cy.findByText('open').should('have.attr', 'data-state', 'closed');
        });

        it('Content has data-state="open" when popover is open', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.findByRole('dialog').should('have.attr', 'data-state', 'open');
        });

        it('Content has data-side attribute', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.findByRole('dialog').should('have.attr', 'data-side');
        });

        it('Content has data-align attribute', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.findByRole('dialog').should('have.attr', 'data-align');
        });
    });

    // ── 3. Keyboard Navigation ──────────────────────────────

    describe('keyboard navigation', () => {
        it('Space opens popover', () => {
            cy.findByText('open').focus();
            cy.realPress('Space');
            shouldBeOpen();
        });

        it('Space closes popover via trigger toggle', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.findByText('open').focus();
            cy.realPress('Space');
            shouldBeClosed();
        });

        it('Enter opens popover', () => {
            cy.findByText('open').focus();
            cy.realPress('Enter');
            shouldBeOpen();
        });

        it('Enter closes popover via trigger toggle', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.findByText('open').focus();
            cy.realPress('Enter');
            shouldBeClosed();
        });

        it('Escape closes popover and restores focus to Trigger', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.realPress('Escape');
            shouldBeClosed();
            cy.findByText('open').should('be.focused');
        });

        it('Tab moves to next focusable element (non-modal, not trapped)', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.findByRole('button', {name: 'close'}).should('be.focused');
            // Tab past close button should leave the popover (non-modal)
            cy.realPress('Tab');
            cy.findByRole('button', {name: 'close'}).should('not.be.focused');
        });

        it('Shift+Tab moves to previous focusable element', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.findByRole('button', {name: 'close'}).should('be.focused');
            cy.realPress(['Shift', 'Tab']);
            cy.findByRole('button', {name: 'close'}).should('not.be.focused');
        });
    });

    // ── 4. Pointer Interaction ──────────────────────────────

    describe('pointer interaction', () => {
        it('click Trigger opens, click Close closes', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.findByRole('button', {name: 'close'}).click();
            shouldBeClosed();
        });

        it('click inside content does not close', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.findByText('Popover content').click();
            shouldBeOpen();
        });

        it('click outside closes (non-modal)', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.get('body').realClick({position: {x: 1, y: 1}});
            shouldBeClosed();
        });

        it('touch outside closes (non-modal)', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.findByTestId('outside-input').realTouch();
            shouldBeClosed();
        });
    });

    // ── 5. Focus Management ─────────────────────────────────

    describe('focus management', () => {
        it('auto-focuses close button on open', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.findByRole('button', {name: 'close'}).should('be.focused');
        });

        it('restores focus to Trigger on close via Escape', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.realPress('Escape');
            shouldBeClosed();
            cy.findByText('open').should('be.focused');
        });

        it('restores focus to Trigger on close via Close button', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.findByRole('button', {name: 'close'}).click();
            shouldBeClosed();
            cy.findByText('open').should('be.focused');
        });
    });

    // ── 6. Modal Variant ────────────────────────────────────

    describe('modal variant', () => {
        beforeEach(() => {
            cy.findByLabelText('modal').click();
        });

        it('keyboard open/close/escape work', () => {
            cy.findByText('open').focus();
            cy.realPress('Space');
            shouldBeOpen();
            cy.realPress('Escape');
            shouldBeClosed();
            cy.findByText('open').should('be.focused');
        });

        it('traps focus within popover', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.findByRole('button', {name: 'close'}).should('be.focused');
            // Tab: close -> wraps to close (only focusable element, trapped)
            cy.realPress('Tab');
            cy.findByRole('button', {name: 'close'}).should('be.focused');
        });

        it('outside click closes', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.get('body').realClick({position: {x: 1, y: 1}});
            shouldBeClosed();
        });

        it('blocks pointer events outside while open', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.get('body').should('have.css', 'pointer-events', 'none');
        });

        it('Escape closes and restores focus', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.realPress('Escape');
            shouldBeClosed();
            cy.findByText('open').should('be.focused');
        });

        it('outside input not interactive while open', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            // Count button should not increment because pointer-events are blocked
            cy.findByTestId('count-value').should('have.text', '0');
            cy.findByTestId('count-button').click({force: true});
            // Popover should close from outside click, but count should not increment
            // because the pointer-events: none blocks the interaction
            shouldBeClosed();
            cy.findByTestId('count-value').should('have.text', '0');
        });
    });
});
