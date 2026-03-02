describe('Toast', () => {
    // ── Helpers ──────────────────────────────────────────────

    function addToast() {
        cy.findByTestId('add-toast').click();
    }

    function shouldBeVisible() {
        cy.findByText('Toast title').should('exist');
    }

    function shouldNotBeVisible() {
        cy.findByText('Toast title').should('not.exist');
    }

    function addMultiToasts(n) {
        for (let i = 0; i < n; i++) {
            cy.findByTestId('add-multi-toast').click();
        }
    }

    beforeEach(() => {
        cy.visit('/toast');
    });

    // ── 1. Accessibility Semantics ──────────────────────────

    describe('accessibility', () => {
        it('Viewport is an ordered list', () => {
            cy.findByTestId('toast-viewport').should('have.prop', 'tagName', 'OL');
        });

        it('Toast is a list item', () => {
            addToast();
            shouldBeVisible();
            cy.findByText('Toast title').closest('li').should('exist');
        });

        it('Viewport region has aria label', () => {
            // The viewport wrapper (DismissableLayer.Branch) has role="region"
            // and aria-label. The label is on the parent wrapper, not the <ol> itself.
            cy.get('[role="region"][aria-label]').should('exist');
        });
    });

    // ── 2. Data Attributes ──────────────────────────────────

    describe('data attributes', () => {
        it('Toast has data-state="open" when visible', () => {
            addToast();
            shouldBeVisible();
            cy.findByText('Toast title')
                .closest('[data-state]')
                .should('have.attr', 'data-state', 'open');
        });

        it('Toast data-state transitions to "closed" on dismiss', () => {
            addToast();
            shouldBeVisible();
            // Click the close button (×)
            cy.findByText('×').click();
            // After animation/transition, toast should not exist
            shouldNotBeVisible();
        });

        it('Toast has data-swipe-direction attribute', () => {
            addToast();
            shouldBeVisible();
            cy.findByText('Toast title')
                .closest('[data-swipe-direction]')
                .should('have.attr', 'data-swipe-direction', 'right');
        });
    });

    // ── 3. Keyboard Navigation ──────────────────────────────

    describe('keyboard navigation', () => {
        it('F8 focuses the toast viewport', () => {
            addToast();
            shouldBeVisible();
            cy.realPress('F8');
            cy.findByTestId('toast-viewport').should('be.focused');
        });

        it('Tab navigates within toast', () => {
            addToast();
            shouldBeVisible();
            cy.realPress('F8');
            cy.realPress('Tab');
            // Should focus the Undo (action) button or Close button within toast
            cy.focused().should('exist');
        });

        it('Escape dismisses focused toast', () => {
            addToast();
            shouldBeVisible();
            // Focus into the toast area
            cy.realPress('F8');
            cy.realPress('Tab');
            cy.realPress('Escape');
            shouldNotBeVisible();
        });

        it('Enter on Close button dismisses toast', () => {
            addToast();
            shouldBeVisible();
            cy.findByText('×').focus();
            cy.realPress('Enter');
            shouldNotBeVisible();
        });

        it('Space on Close button dismisses toast', () => {
            addToast();
            shouldBeVisible();
            cy.findByText('×').focus();
            cy.realPress('Space');
            shouldNotBeVisible();
        });

        it('Enter on Action button dismisses toast', () => {
            addToast();
            shouldBeVisible();
            cy.findByText('Undo').focus();
            cy.realPress('Enter');
            shouldNotBeVisible();
        });
    });

    // ── 4. Pointer Interaction ──────────────────────────────

    describe('pointer interaction', () => {
        it('clicking "Add toast" shows a toast', () => {
            shouldNotBeVisible();
            addToast();
            shouldBeVisible();
        });

        it('clicking Close dismisses the toast', () => {
            addToast();
            shouldBeVisible();
            cy.findByText('×').click();
            shouldNotBeVisible();
        });

        it('clicking Action dismisses the toast', () => {
            addToast();
            shouldBeVisible();
            cy.findByText('Undo').click();
            shouldNotBeVisible();
        });

        it('toast count increments', () => {
            cy.findByTestId('toast-count').should('have.text', '0');
            addToast();
            cy.findByTestId('toast-count').should('have.text', '1');
        });
    });

    // ── 5. Dismiss Behavior ─────────────────────────────────

    describe('dismiss behavior', () => {
        it('Close button dismisses', () => {
            addToast();
            shouldBeVisible();
            cy.findByText('×').click();
            shouldNotBeVisible();
        });

        it('Action button dismisses', () => {
            addToast();
            shouldBeVisible();
            cy.findByText('Undo').click();
            shouldNotBeVisible();
        });

        it('Escape key dismisses when focused', () => {
            addToast();
            shouldBeVisible();
            cy.realPress('F8');
            cy.realPress('Tab');
            cy.realPress('Escape');
            shouldNotBeVisible();
        });

        it('auto-dismiss after duration when enabled', () => {
            // Enable auto-dismiss (2000ms)
            cy.findByLabelText('auto-dismiss').click();
            addToast();
            shouldBeVisible();
            // Toast should disappear after the duration (2000ms) — use Cypress retry
            cy.findByText('Toast title', {timeout: 10000}).should('not.exist');
        });

        it('stays visible indefinitely when auto-dismiss is off', () => {
            addToast();
            shouldBeVisible();
            // Wait a reasonable time — toast should still be visible
            cy.wait(1000); // eslint-disable-line cypress/no-unnecessary-waiting
            shouldBeVisible();
        });
    });

    // ── 6. Content ──────────────────────────────────────────

    describe('content', () => {
        it('displays title and description', () => {
            addToast();
            cy.findByText('Toast title').should('exist');
            cy.findByText('Toast description').should('exist');
        });

        it('displays action button', () => {
            addToast();
            cy.findByText('Undo').should('exist');
        });

        it('displays close button', () => {
            addToast();
            cy.findByText('×').should('exist');
        });
    });

    // ── 7. Uncontrolled Mode ────────────────────────────────

    describe('uncontrolled mode', () => {
        it('uncontrolled toast appears when mounted', () => {
            cy.findByText('Uncontrolled toast').should('not.exist');
            cy.findByTestId('show-uncontrolled').click();
            cy.findByText('Uncontrolled toast').should('exist');
        });

        it('uncontrolled toast has data-state="open"', () => {
            cy.findByTestId('show-uncontrolled').click();
            cy.findByText('Uncontrolled toast')
                .closest('[data-state]')
                .should('have.attr', 'data-state', 'open');
        });

        it('uncontrolled toast close button dismisses', () => {
            cy.findByTestId('show-uncontrolled').click();
            cy.findByText('Uncontrolled toast').should('exist');
            cy.findByTestId('uncontrolled-close').click();
            cy.findByText('Uncontrolled toast').should('not.exist');
        });

        it('uncontrolled toast close via Escape', () => {
            cy.findByTestId('show-uncontrolled').click();
            cy.findByText('Uncontrolled toast').should('exist');
            // Focus the toast
            cy.findByText('Uncontrolled toast').closest('li').focus();
            cy.realPress('Escape');
            cy.findByText('Uncontrolled toast').should('not.exist');
        });

        it('uncontrolled toast is a list item inside viewport', () => {
            cy.findByTestId('show-uncontrolled').click();
            cy.findByText('Uncontrolled toast').closest('li').should('exist');
            // Verify the toast li is inside the viewport ol
            cy.findByTestId('toast-viewport')
                .find('li')
                .should('contain.text', 'Uncontrolled toast');
        });
    });

    // ── 8. Viewport Tab Order ───────────────────────────────
    //
    // Toast viewport manages its own tab order programmatically:
    // - Tab from before-viewport enters the most recently added toast first
    // - Tab proceeds through focusable elements within that toast
    // - Then moves to the next most recent toast (reverse DOM order)
    // - Tab past the last toast exits the viewport to after-viewport
    // - Shift+Tab reverses the order

    describe('viewport tab order', () => {
        it('Tab from before-viewport focuses the most recent toast root (single toast)', () => {
            addMultiToasts(1);
            cy.findByTestId('multi-toast-1').should('exist');
            cy.findByTestId('before-viewport').focus();
            cy.realPress('Tab');
            // Should land on the toast root (li) — the most recent (and only) toast
            cy.focused().should('have.attr', 'data-testid', 'multi-toast-1');
        });

        it('Tab from before-viewport focuses the most recent toast (multiple toasts)', () => {
            addMultiToasts(3);
            cy.findByTestId('multi-toast-3').should('exist');
            cy.findByTestId('before-viewport').focus();
            cy.realPress('Tab');
            // Should land on toast 3 (most recent), NOT toast 1
            cy.focused().should('have.attr', 'data-testid', 'multi-toast-3');
        });

        it('Tab traverses within a toast before moving to the next toast', () => {
            addMultiToasts(2);
            cy.findByTestId('multi-toast-2').should('exist');
            cy.findByTestId('before-viewport').focus();

            // Tab 1: toast 2 root (most recent)
            cy.realPress('Tab');
            cy.focused().should('have.attr', 'data-testid', 'multi-toast-2');

            // Tab 2: action button within toast 2
            cy.realPress('Tab');
            cy.focused().should('have.attr', 'data-testid', 'multi-action-2');

            // Tab 3: close button within toast 2
            cy.realPress('Tab');
            cy.focused().should('have.attr', 'data-testid', 'multi-close-2');

            // Tab 4: toast 1 root (next most recent)
            cy.realPress('Tab');
            cy.focused().should('have.attr', 'data-testid', 'multi-toast-1');

            // Tab 5: action button within toast 1
            cy.realPress('Tab');
            cy.focused().should('have.attr', 'data-testid', 'multi-action-1');

            // Tab 6: close button within toast 1
            cy.realPress('Tab');
            cy.focused().should('have.attr', 'data-testid', 'multi-close-1');
        });

        it('Tab past last toast exits to after-viewport', () => {
            addMultiToasts(1);
            cy.findByTestId('multi-toast-1').should('exist');
            cy.findByTestId('before-viewport').focus();

            // Tab through: toast root, action, close
            cy.realPress('Tab'); // toast root
            cy.realPress('Tab'); // action
            cy.realPress('Tab'); // close

            // Next tab should exit the viewport to after-viewport
            cy.realPress('Tab');
            cy.focused().should('have.attr', 'data-testid', 'after-viewport');
        });

        it('Shift+Tab from after-viewport focuses oldest toast last element', () => {
            addMultiToasts(2);
            cy.findByTestId('multi-toast-2').should('exist');
            cy.findByTestId('after-viewport').focus();

            // Shift+Tab enters at the oldest toast's last focusable element
            cy.realPress(['Shift', 'Tab']);
            cy.focused().should('have.attr', 'data-testid', 'multi-close-1');
        });

        it('Shift+Tab traverses in reverse: oldest toast first, then next', () => {
            addMultiToasts(2);
            cy.findByTestId('multi-toast-2').should('exist');
            cy.findByTestId('after-viewport').focus();

            // Shift+Tab 1: close button of toast 1 (oldest)
            cy.realPress(['Shift', 'Tab']);
            cy.focused().should('have.attr', 'data-testid', 'multi-close-1');

            // Shift+Tab 2: action button of toast 1
            cy.realPress(['Shift', 'Tab']);
            cy.focused().should('have.attr', 'data-testid', 'multi-action-1');

            // Shift+Tab 3: toast 1 root
            cy.realPress(['Shift', 'Tab']);
            cy.focused().should('have.attr', 'data-testid', 'multi-toast-1');

            // Shift+Tab 4: close button of toast 2 (most recent)
            cy.realPress(['Shift', 'Tab']);
            cy.focused().should('have.attr', 'data-testid', 'multi-close-2');

            // Shift+Tab 5: action button of toast 2
            cy.realPress(['Shift', 'Tab']);
            cy.focused().should('have.attr', 'data-testid', 'multi-action-2');

            // Shift+Tab 6: toast 2 root
            cy.realPress(['Shift', 'Tab']);
            cy.focused().should('have.attr', 'data-testid', 'multi-toast-2');
        });

        it('Shift+Tab past first toast exits to before-viewport', () => {
            addMultiToasts(1);
            cy.findByTestId('multi-toast-1').should('exist');
            cy.findByTestId('after-viewport').focus();

            // Shift+Tab through: close, action, toast root
            cy.realPress(['Shift', 'Tab']); // close
            cy.realPress(['Shift', 'Tab']); // action
            cy.realPress(['Shift', 'Tab']); // toast root

            // Next Shift+Tab should exit the viewport to before-viewport
            cy.realPress(['Shift', 'Tab']);
            cy.focused().should('have.attr', 'data-testid', 'before-viewport');
        });

        it('F8 focuses viewport, then Tab moves to most recent toast', () => {
            addMultiToasts(2);
            cy.findByTestId('multi-toast-2').should('exist');
            cy.realPress('F8');
            cy.findByTestId('toast-viewport').should('be.focused');
            cy.realPress('Tab');
            // Should focus the most recent toast (toast 2)
            cy.focused().should('have.attr', 'data-testid', 'multi-toast-2');
        });

        it('three toasts: full forward tab cycle', () => {
            addMultiToasts(3);
            cy.findByTestId('multi-toast-3').should('exist');
            cy.findByTestId('before-viewport').focus();

            // Toast 3 (most recent)
            cy.realPress('Tab');
            cy.focused().should('have.attr', 'data-testid', 'multi-toast-3');
            cy.realPress('Tab');
            cy.focused().should('have.attr', 'data-testid', 'multi-action-3');
            cy.realPress('Tab');
            cy.focused().should('have.attr', 'data-testid', 'multi-close-3');

            // Toast 2
            cy.realPress('Tab');
            cy.focused().should('have.attr', 'data-testid', 'multi-toast-2');
            cy.realPress('Tab');
            cy.focused().should('have.attr', 'data-testid', 'multi-action-2');
            cy.realPress('Tab');
            cy.focused().should('have.attr', 'data-testid', 'multi-close-2');

            // Toast 1 (oldest)
            cy.realPress('Tab');
            cy.focused().should('have.attr', 'data-testid', 'multi-toast-1');
            cy.realPress('Tab');
            cy.focused().should('have.attr', 'data-testid', 'multi-action-1');
            cy.realPress('Tab');
            cy.focused().should('have.attr', 'data-testid', 'multi-close-1');

            // Exit viewport
            cy.realPress('Tab');
            cy.focused().should('have.attr', 'data-testid', 'after-viewport');
        });
    });
});
