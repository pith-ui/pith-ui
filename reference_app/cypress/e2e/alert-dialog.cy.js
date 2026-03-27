describe('AlertDialog', () => {
    // ── Helpers ──────────────────────────────────────────────

    function shouldBeOpen() {
        cy.findByRole('alertdialog').should('exist');
    }

    function shouldBeClosed() {
        cy.findByRole('alertdialog').should('not.exist');
    }

    beforeEach(() => {
        cy.visit('/alert-dialog');
    });

    // ── 1. Accessibility Semantics ──────────────────────────

    describe('accessibility', () => {
        it('Content has role="alertdialog"', () => {
            cy.findByText('delete').click();
            cy.findByRole('alertdialog').should('exist');
        });

        it('Content has aria-labelledby pointing to Title', () => {
            cy.findByText('delete').click();
            cy.findByRole('alertdialog')
                .invoke('attr', 'aria-labelledby')
                .then((labelId) => {
                    cy.get(`#${labelId}`).should('have.text', 'Are you sure?');
                });
        });

        it('Content has aria-describedby pointing to Description', () => {
            cy.findByText('delete').click();
            cy.findByRole('alertdialog')
                .invoke('attr', 'aria-describedby')
                .then((descId) => {
                    cy.get(`#${descId}`).should('have.text', 'This action cannot be undone.');
                });
        });

        it('Trigger has aria-haspopup="dialog"', () => {
            cy.findByText('delete').should('have.attr', 'aria-haspopup', 'dialog');
        });

        it('Trigger aria-expanded reflects open state', () => {
            cy.findByText('delete').should('have.attr', 'aria-expanded', 'false');
            cy.findByText('delete').click();
            shouldBeOpen();
            cy.findByText('delete').should('have.attr', 'aria-expanded', 'true');
        });

        it('Trigger aria-controls references Content id', () => {
            cy.findByText('delete').click();
            shouldBeOpen();
            cy.findByText('delete')
                .invoke('attr', 'aria-controls')
                .then((controlsId) => {
                    cy.findByRole('alertdialog').should('have.attr', 'id', controlsId);
                });
        });
    });

    // ── 2. Data Attributes ──────────────────────────────────

    describe('data attributes', () => {
        it('Trigger data-state reflects open state', () => {
            cy.findByText('delete').should('have.attr', 'data-state', 'closed');
            cy.findByText('delete').click();
            shouldBeOpen();
            cy.findByText('delete').should('have.attr', 'data-state', 'open');
            cy.findByRole('button', { name: 'cancel' }).click();
            shouldBeClosed();
            cy.findByText('delete').should('have.attr', 'data-state', 'closed');
        });

        it('Overlay has data-state="open" when dialog is open', () => {
            cy.findByText('delete').click();
            shouldBeOpen();
            cy.get('.alert-dialog-overlay').should('have.attr', 'data-state', 'open');
        });

        it('Content has data-state="open" when dialog is open', () => {
            cy.findByText('delete').click();
            shouldBeOpen();
            cy.findByRole('alertdialog').should('have.attr', 'data-state', 'open');
        });
    });

    // ── 3. Keyboard Navigation ──────────────────────────────

    describe('keyboard navigation', () => {
        it('Space opens via Trigger', () => {
            cy.findByText('delete').focus();
            cy.realPress('Space');
            shouldBeOpen();
        });

        it('Enter opens via Trigger', () => {
            cy.findByText('delete').focus();
            cy.realPress('Enter');
            shouldBeOpen();
        });

        it('Escape closes and restores focus to Trigger', () => {
            cy.findByText('delete').click();
            shouldBeOpen();
            cy.realPress('Escape');
            shouldBeClosed();
            cy.findByText('delete').should('be.focused');
        });

        it('Space closes via Cancel button', () => {
            cy.findByText('delete').click();
            shouldBeOpen();
            cy.findByRole('button', { name: 'cancel' }).focus();
            cy.realPress('Space');
            shouldBeClosed();
        });

        it('Enter closes via Action button', () => {
            cy.findByText('delete').click();
            shouldBeOpen();
            cy.findByRole('button', { name: 'confirm' }).focus();
            cy.realPress('Enter');
            shouldBeClosed();
        });

        it('Tab moves to next focusable element', () => {
            cy.findByText('delete').click();
            shouldBeOpen();
            cy.findByRole('button', { name: 'cancel' }).should('be.focused');
            cy.realPress('Tab');
            cy.findByRole('button', { name: 'confirm' }).should('be.focused');
        });

        it('Shift+Tab moves to previous focusable element', () => {
            cy.findByText('delete').click();
            shouldBeOpen();
            cy.realPress('Tab');
            cy.findByRole('button', { name: 'confirm' }).should('be.focused');
            cy.realPress(['Shift', 'Tab']);
            cy.findByRole('button', { name: 'cancel' }).should('be.focused');
        });
    });

    // ── 4. Pointer Interaction ──────────────────────────────

    describe('pointer interaction', () => {
        it('click Trigger opens', () => {
            cy.findByText('delete').click();
            shouldBeOpen();
        });

        it('click Action closes', () => {
            cy.findByText('delete').click();
            shouldBeOpen();
            cy.findByRole('button', { name: 'confirm' }).click();
            shouldBeClosed();
        });

        it('click Cancel closes', () => {
            cy.findByText('delete').click();
            shouldBeOpen();
            cy.findByRole('button', { name: 'cancel' }).click();
            shouldBeClosed();
        });

        it('click outside does NOT close', () => {
            cy.findByText('delete').click();
            shouldBeOpen();
            cy.get('.alert-dialog-overlay').realClick({ position: { x: 10, y: 10 } });
            shouldBeOpen();
        });

        it('touch outside does NOT close', () => {
            cy.findByText('delete').click();
            shouldBeOpen();
            cy.get('.alert-dialog-overlay').realTouch({ position: { x: 10, y: 10 } });
            shouldBeOpen();
        });

        it('click inside content does not close', () => {
            cy.findByText('delete').click();
            shouldBeOpen();
            cy.findByRole('alertdialog').click();
            shouldBeOpen();
        });

        it('blocks pointer events outside', () => {
            cy.findByText('delete').click();
            shouldBeOpen();
            cy.get('body').should('have.css', 'pointer-events', 'none');
        });

        it('outside count button is not clickable while open', () => {
            cy.findByText('delete').click();
            shouldBeOpen();
            // The count button should not respond while the alert dialog is open
            cy.findByText('0').should('exist');
            cy.findByRole('button', { name: 'cancel' }).click();
            shouldBeClosed();
            // Now the count button should work
            cy.findByText('0').click();
            cy.findByText('1').should('exist');
        });
    });

    // ── 5. Focus Management ─────────────────────────────────

    describe('focus management', () => {
        it('auto-focuses Cancel button on open', () => {
            cy.findByText('delete').click();
            shouldBeOpen();
            cy.findByRole('button', { name: 'cancel' }).should('be.focused');
        });

        it('restores focus to Trigger on close via Cancel', () => {
            cy.findByText('delete').click();
            shouldBeOpen();
            cy.findByRole('button', { name: 'cancel' }).click();
            shouldBeClosed();
            cy.findByText('delete').should('be.focused');
        });

        it('restores focus to Trigger on close via Action', () => {
            cy.findByText('delete').click();
            shouldBeOpen();
            cy.findByRole('button', { name: 'confirm' }).click();
            shouldBeClosed();
            cy.findByText('delete').should('be.focused');
        });

        it('restores focus to Trigger on Escape', () => {
            cy.findByText('delete').click();
            shouldBeOpen();
            cy.realPress('Escape');
            shouldBeClosed();
            cy.findByText('delete').should('be.focused');
        });

        it('traps focus within dialog (Tab wraps around)', () => {
            cy.findByText('delete').click();
            shouldBeOpen();
            cy.findByRole('button', { name: 'cancel' }).should('be.focused');
            // Tab: cancel -> confirm
            cy.realPress('Tab');
            cy.findByRole('button', { name: 'confirm' }).should('be.focused');
            // Tab: confirm -> wraps to cancel (trapped)
            cy.realPress('Tab');
            cy.findByRole('button', { name: 'cancel' }).should('be.focused');
        });

        it('traps focus within dialog (Shift+Tab wraps around)', () => {
            cy.findByText('delete').click();
            shouldBeOpen();
            cy.findByRole('button', { name: 'cancel' }).should('be.focused');
            // Shift+Tab: cancel -> wraps to confirm (trapped)
            cy.realPress(['Shift', 'Tab']);
            cy.findByRole('button', { name: 'confirm' }).should('be.focused');
        });
    });

    // ── 6. Controlled Mode ──────────────────────────────────

    describe('controlled mode', () => {
        it('external checkbox opens alert dialog', () => {
            cy.findByTestId('controlled-content').should('not.exist');
            cy.findByTestId('controlled-checkbox').check();
            cy.findByTestId('controlled-content').should('exist');
        });

        it('external control closes alert dialog', () => {
            cy.findByTestId('controlled-checkbox').check();
            cy.findByTestId('controlled-content').should('exist');
            cy.findByTestId('controlled-external-close').click({force: true});
            cy.findByTestId('controlled-content').should('not.exist');
            cy.findByTestId('controlled-state').should('have.text', 'closed');
        });

        it('trigger opens controlled alert dialog', () => {
            cy.findByTestId('controlled-content').should('not.exist');
            cy.findByTestId('controlled-trigger').click();
            cy.findByTestId('controlled-content').should('exist');
            cy.findByTestId('controlled-checkbox').should('be.checked');
        });

        it('closing via cancel updates external state', () => {
            cy.findByTestId('controlled-trigger').click();
            cy.findByTestId('controlled-content').should('exist');
            cy.findByTestId('controlled-cancel').click();
            cy.findByTestId('controlled-content').should('not.exist');
            cy.findByTestId('controlled-checkbox').should('not.be.checked');
        });

        it('closing via Escape updates external state', () => {
            cy.findByTestId('controlled-trigger').click();
            cy.findByTestId('controlled-content').should('exist');
            cy.realPress('Escape');
            cy.findByTestId('controlled-content').should('not.exist');
            cy.findByTestId('controlled-checkbox').should('not.be.checked');
        });
    });

    // ── Axe Accessibility Audit ─────────────────────────────

    describe('axe audit', () => {
        it('no violations in closed state', () => {
            cy.checkComponentA11y();
        });

        it('no violations when open', () => {
            cy.findByText('delete').click();
            shouldBeOpen();
            cy.checkComponentA11y();
        });
    });
});
