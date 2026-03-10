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

        it('non-modal does not trap focus (outside elements focusable)', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            // In non-modal, outside elements can receive focus directly
            cy.findByTestId('outside-input').focus();
            cy.findByTestId('outside-input').should('be.focused');
            // Popover remains open while outside element is focused
            shouldBeOpen();
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
            // pointer-events: none blocks outside interaction
            cy.get('body').should('have.css', 'pointer-events', 'none');
            // Outside focus should not be possible in modal mode
            cy.findByTestId('outside-input').should('not.be.focused');
        });
    });

    // ── 7. PopoverAnchor ──────────────────────────────────────

    describe('PopoverAnchor', () => {
        // popover-dcnv-1

        it('opens when trigger is clicked', () => {
            cy.findByTestId('anchor-trigger').click();
            cy.findByTestId('anchor-content').should('exist');
        });

        it('closes via close button', () => {
            cy.findByTestId('anchor-trigger').click();
            cy.findByTestId('anchor-content').should('exist');
            cy.findByTestId('anchor-close').click();
            cy.findByTestId('anchor-content').should('not.exist');
        });

        it('content positions near anchor, not trigger', () => {
            // The trigger and anchor are 200px apart horizontally.
            // The popover should position relative to the anchor element.
            cy.findByTestId('anchor-trigger').then(($trigger) => {
                const triggerRect = $trigger[0].getBoundingClientRect();

                cy.findByTestId('anchor-trigger').click();
                cy.findByTestId('anchor-content').should('exist');

                cy.findByTestId('popover-anchor').then(($anchor) => {
                    const anchorRect = $anchor[0].getBoundingClientRect();
                    cy.findByTestId('anchor-content').then(($content) => {
                        const contentRect = $content[0].getBoundingClientRect();
                        const contentCenterX = contentRect.left + contentRect.width / 2;
                        const anchorCenterX = anchorRect.left + anchorRect.width / 2;
                        const triggerCenterX = triggerRect.left + triggerRect.width / 2;

                        // Content should be closer to the anchor than to the trigger
                        const distToAnchor = Math.abs(contentCenterX - anchorCenterX);
                        const distToTrigger = Math.abs(contentCenterX - triggerCenterX);
                        expect(distToAnchor).to.be.lessThan(distToTrigger);
                    });
                });
            });
        });

        it('Escape closes and restores focus to trigger', () => {
            cy.findByTestId('anchor-trigger').click();
            cy.findByTestId('anchor-content').should('exist');
            cy.realPress('Escape');
            cy.findByTestId('anchor-content').should('not.exist');
            cy.findByTestId('anchor-trigger').should('be.focused');
        });
    });

    // ── Internal Styles ─────────────────────────────────────

    describe('internal styles', () => {
        it('content has --radix-popover-content-available-width', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.findByRole('dialog').then(($el) => {
                const value = getComputedStyle($el[0]).getPropertyValue(
                    '--radix-popover-content-available-width'
                );
                expect(value.trim()).to.not.be.empty;
            });
        });

        it('content has --radix-popover-content-available-height', () => {
            cy.findByText('open').click();
            shouldBeOpen();
            cy.findByRole('dialog').then(($el) => {
                const value = getComputedStyle($el[0]).getPropertyValue(
                    '--radix-popover-content-available-height'
                );
                expect(value.trim()).to.not.be.empty;
            });
        });
    });

    // ── 8. Controlled Mode ────────────────────────────────────

    describe('controlled mode', () => {
        it('external checkbox opens popover', () => {
            // popover-msc-1
            cy.findByTestId('controlled-content').should('not.exist');
            cy.findByLabelText('open controlled').click();
            cy.findByTestId('controlled-content').should('exist');
        });

        it('external control closes popover', () => {
            // popover-msc-1
            cy.findByLabelText('open controlled').click();
            cy.findByTestId('controlled-content').should('exist');
            // Use dedicated button to avoid outside-click dismiss racing with checkbox toggle
            cy.findByTestId('controlled-external-close').click();
            cy.findByTestId('controlled-content').should('not.exist');
            cy.findByTestId('controlled-open-state').should('have.text', 'closed');
        });

        it('clicking trigger updates external state', () => {
            // popover-msc-1
            cy.findByLabelText('open controlled').should('not.be.checked');
            cy.findByTestId('controlled-trigger').click();
            cy.findByTestId('controlled-content').should('exist');
            cy.findByLabelText('open controlled').should('be.checked');
        });

        it('closing via Escape updates external state', () => {
            // popover-msc-1
            cy.findByTestId('controlled-trigger').click();
            cy.findByTestId('controlled-content').should('exist');
            cy.findByLabelText('open controlled').should('be.checked');
            cy.realPress('Escape');
            cy.findByTestId('controlled-content').should('not.exist');
            cy.findByLabelText('open controlled').should('not.be.checked');
        });
    });
});
