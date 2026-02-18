describe('Dialog', () => {
    // ── Helpers ──────────────────────────────────────────────

    function shouldBeOpen() {
        cy.findByRole('dialog').should('exist');
    }

    function shouldBeClosed() {
        cy.findByRole('dialog').should('not.exist');
    }

    beforeEach(() => {
        cy.visit('/dialog');
    });

    // ── 1. Accessibility Semantics ──────────────────────────

    describe('accessibility', () => {
        it('Content has role="dialog"', () => {
            cy.findByText('open').click();
            cy.findByRole('dialog').should('exist');
        });

        it('Content has aria-labelledby pointing to Title', () => {
            cy.findByText('open').click();
            cy.findByRole('dialog')
                .invoke('attr', 'aria-labelledby')
                .then((labelId) => {
                    cy.get(`#${labelId}`).should('have.text', 'title');
                });
        });

        it('Content has aria-describedby pointing to Description', () => {
            cy.findByText('open').click();
            cy.findByRole('dialog')
                .invoke('attr', 'aria-describedby')
                .then((descId) => {
                    cy.get(`#${descId}`).should('have.text', 'description');
                });
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

        it('Overlay has data-state="open" when dialog is open', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.get('.dialog-overlay').should('have.attr', 'data-state', 'open');
        });

        it('Content has data-state="open" when dialog is open', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.findByRole('dialog').should('have.attr', 'data-state', 'open');
        });
    });

    // ── 3. Keyboard Navigation ──────────────────────────────

    describe('keyboard navigation', () => {
        it('Space opens dialog', () => {
            cy.findByText('open').focus();
            cy.realPress('Space');
            shouldBeOpen();
        });

        it('Space closes dialog via Close button', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.findByRole('button', {name: 'close'}).focus();
            cy.realPress('Space');
            shouldBeClosed();
        });

        it('Enter opens dialog', () => {
            cy.findByText('open').focus();
            cy.realPress('Enter');
            shouldBeOpen();
        });

        it('Enter closes dialog via Close button', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.findByRole('button', {name: 'close'}).focus();
            cy.realPress('Enter');
            shouldBeClosed();
        });

        it('Escape closes dialog and restores focus to Trigger', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.realPress('Escape');
            shouldBeClosed();
            cy.findByText('open').should('be.focused');
        });

        it('Tab moves to next focusable element', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.findByRole('button', {name: 'close'}).should('be.focused');
            cy.realPress('Tab');
            cy.findByText('destroy me').should('be.focused');
        });

        it('Shift+Tab moves to previous focusable element', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.realPress('Tab');
            cy.findByText('destroy me').should('be.focused');
            cy.realPress(['Shift', 'Tab']);
            cy.findByRole('button', {name: 'close'}).should('be.focused');
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
            cy.findByRole('dialog').click();
            shouldBeOpen();
        });

        it('click outside (overlay) closes modal', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.get('.dialog-overlay').realClick({position: {x: 10, y: 10}});
            shouldBeClosed();
        });

        it('touch outside closes modal', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.get('.dialog-overlay').realTouch({position: {x: 10, y: 10}});
            shouldBeClosed();
        });

        it('blocks pointer events outside modal', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.get('body').should('have.css', 'pointer-events', 'none');
        });

        it('pointer-events restored after animated close', () => {
            cy.findByLabelText('animated').click();
            cy.findByText('open').click();
            shouldBeOpen();
            cy.realPress('Escape');
            shouldBeClosed();
            cy.findByText('0').click();
            cy.findByText('1').should('exist');
        });
    });

    // ── 5. Focus Management ─────────────────────────────────

    describe('focus management', () => {
        it('auto-focuses Close button on open', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.findByRole('button', {name: 'close'}).should('be.focused');
        });

        it('restores focus to Trigger on close', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.findByRole('button', {name: 'close'}).click();
            shouldBeClosed();
            cy.findByText('open').should('be.focused');
        });

        it('traps focus within dialog (Tab wraps around)', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.findByRole('button', {name: 'close'}).should('be.focused');
            // Tab: close -> destroy me
            cy.realPress('Tab');
            cy.findByText('destroy me').should('be.focused');
            // Tab: destroy me -> wraps to close (trapped)
            cy.realPress('Tab');
            cy.findByRole('button', {name: 'close'}).should('be.focused');
        });

        it('keeps focus trapped when focused element is removed', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            // Tab to destroy me
            cy.realPress('Tab');
            cy.findByText('destroy me').should('be.focused');
            // Click destroy me (removes itself from DOM)
            cy.findByText('destroy me').click();
            cy.findByText('destroy me').should('not.exist');
            // Tab should still work within the dialog
            cy.realPress('Tab');
            cy.findByRole('button', {name: 'close'}).should('be.focused');
        });
    });

    // ── 6. Non-modal variant ────────────────────────────────

    describe('non-modal variant', () => {
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

        it('does not trap focus', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.findByRole('button', {name: 'close'}).should('be.focused');
            // Tab: close -> destroy me
            cy.realPress('Tab');
            cy.findByText('destroy me').should('be.focused');
            // Tab: destroy me -> should leave dialog (not wrap to close)
            cy.realPress('Tab');
            cy.findByRole('button', {name: 'close'}).should('not.be.focused');
        });

        it('outside click closes', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.get('body').realClick({position: {x: 1, y: 1}});
            shouldBeClosed();
        });

        it('outside input gets focus (not blocked)', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.findByPlaceholderText('name').realClick();
            shouldBeClosed();
            cy.findByPlaceholderText('name').should('be.focused');
        });

        it('pointer-events restored after animated close', () => {
            cy.findByLabelText('animated').click();
            cy.findByText('open').click();
            shouldBeOpen();
            cy.realPress('Escape');
            shouldBeClosed();
            cy.findByText('0').click();
            cy.findByText('1').should('exist');
        });
    });
});
