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
    });

    // ── 2. Data Attributes ──────────────────────────────────

    describe('data attributes', () => {
        it('Toast has data-state="open" when visible', () => {
            addToast();
            shouldBeVisible();
            cy.findByText('Toast title').closest('[data-state]').should('have.attr', 'data-state', 'open');
        });

        it('Toast data-state transitions to "closed" on dismiss', () => {
            addToast();
            shouldBeVisible();
            // Click the close button (×)
            cy.findByText('×').click();
            // After animation/transition, toast should not exist
            shouldNotBeVisible();
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
});
