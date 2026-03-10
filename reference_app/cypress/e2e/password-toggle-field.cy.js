describe('PasswordToggleField', () => {
    // ── Helpers ──────────────────────────────────────────────

    function iconToggle() {
        return cy.findByRole('button', {name: /password/i});
    }

    function slotToggle() {
        return cy.get('[aria-controls="pin"]');
    }

    function formToggle() {
        return cy.get('[aria-controls="form-password"]');
    }

    function passwordInput() {
        return cy.findByLabelText('Password');
    }

    function pinInput() {
        return cy.findByLabelText('PIN');
    }

    function formPasswordInput() {
        return cy.findByLabelText('Form Password');
    }

    beforeEach(() => {
        cy.visit('/password-toggle-field');
    });

    // ── 1. Accessibility Semantics ──────────────────────────

    describe('accessibility', () => {
        it('icon toggle has aria-controls pointing to input id', () => {
            iconToggle().should('have.attr', 'aria-controls', 'password');
        });

        it('implicit aria-label is "Show password" when hidden (icon variant)', () => {
            cy.findByRole('button', {name: 'Show password'}).should('exist');
            cy.findByRole('button', {name: 'Show password'}).should('have.attr', 'aria-label', 'Show password');
        });

        it('implicit aria-label changes to "Hide password" when visible (icon variant)', () => {
            cy.findByRole('button', {name: 'Show password'}).click();
            cy.findByRole('button', {name: 'Hide password'}).should('exist');
            cy.findByRole('button', {name: 'Hide password'}).should('have.attr', 'aria-label', 'Hide password');
        });

        it('no implicit aria-label when toggle has text content (slot variant)', () => {
            slotToggle().should('not.have.attr', 'aria-label');
        });

        it('icon SVG has aria-hidden', () => {
            cy.findByTestId('eye-closed').should('have.attr', 'aria-hidden', 'true');
        });

        it('toggle renders as a button', () => {
            iconToggle().should('have.prop', 'tagName', 'BUTTON');
            iconToggle().should('have.attr', 'type', 'button');
        });
    });

    // ── 1b. Input Attributes ─────────────────────────────────

    describe('input attributes', () => {
        it('input has autocomplete attribute', () => {
            passwordInput().should('have.attr', 'autocomplete');
        });

        it('input has spellcheck="false"', () => {
            passwordInput().should('have.attr', 'spellcheck', 'false');
        });

        it('input has autocapitalize="off"', () => {
            passwordInput().should('have.attr', 'autocapitalize', 'off');
        });
    });

    // ── 2. Input Type Toggling ──────────────────────────────

    describe('input type', () => {
        it('input starts as type="password"', () => {
            passwordInput().should('have.attr', 'type', 'password');
        });

        it('input changes to type="text" when toggled', () => {
            cy.findByRole('button', {name: 'Show password'}).click();
            passwordInput().should('have.attr', 'type', 'text');
        });

        it('input changes back to type="password" on second toggle', () => {
            cy.findByRole('button', {name: 'Show password'}).click();
            cy.findByRole('button', {name: 'Hide password'}).click();
            passwordInput().should('have.attr', 'type', 'password');
        });
    });

    // ── 3. Keyboard Navigation ──────────────────────────────

    describe('keyboard navigation', () => {
        it('Space toggles visibility', () => {
            cy.findByRole('button', {name: 'Show password'}).focus();
            cy.realPress('Space');
            passwordInput().should('have.attr', 'type', 'text');
        });

        it('Enter toggles visibility', () => {
            cy.findByRole('button', {name: 'Show password'}).focus();
            cy.realPress('Enter');
            passwordInput().should('have.attr', 'type', 'text');
        });

        it('focus stays on toggle button after keyboard activation', () => {
            cy.findByRole('button', {name: 'Show password'}).focus();
            cy.realPress('Enter');
            cy.findByRole('button', {name: 'Hide password'}).should('be.focused');
        });

        it('Tab navigates between input and toggle', () => {
            passwordInput().focus();
            cy.realPress('Tab');
            iconToggle().should('be.focused');
        });
    });

    // ── 4. Pointer Interaction ──────────────────────────────

    describe('pointer interaction', () => {
        it('click toggles visibility', () => {
            cy.findByRole('button', {name: 'Show password'}).click();
            passwordInput().should('have.attr', 'type', 'text');
            cy.findByRole('button', {name: 'Hide password'}).click();
            passwordInput().should('have.attr', 'type', 'password');
        });

        it('focus moves to input after pointer toggle', () => {
            cy.findByRole('button', {name: 'Show password'}).realClick();
            passwordInput().should('be.focused');
        });
    });

    // ── 5. Value Preservation ───────────────────────────────

    describe('value preservation', () => {
        it('retains input value when toggled', () => {
            passwordInput().type('secret123');
            cy.findByRole('button', {name: 'Show password'}).click();
            passwordInput().should('have.value', 'secret123');
            cy.findByRole('button', {name: 'Hide password'}).click();
            passwordInput().should('have.value', 'secret123');
        });
    });

    // ── 6. Icon Rendering ───────────────────────────────────

    describe('icon rendering', () => {
        it('shows closed eye icon when hidden', () => {
            cy.findByTestId('eye-closed').should('exist');
            cy.findByTestId('eye-open').should('not.exist');
        });

        it('shows open eye icon when visible', () => {
            cy.findByRole('button', {name: 'Show password'}).click();
            cy.findByTestId('eye-open').should('exist');
            cy.findByTestId('eye-closed').should('not.exist');
        });
    });

    // ── 7. Slot Rendering ───────────────────────────────────

    describe('slot rendering', () => {
        it('shows "Show" text when hidden', () => {
            slotToggle().should('have.text', 'Show');
        });

        it('shows "Hide" text when visible', () => {
            slotToggle().click();
            slotToggle().should('have.text', 'Hide');
        });

        it('slot variant toggles input type', () => {
            pinInput().should('have.attr', 'type', 'password');
            slotToggle().click();
            pinInput().should('have.attr', 'type', 'text');
        });
    });

    // ── 8. Form Integration ─────────────────────────────────

    describe('form integration', () => {
        it('resets visibility to hidden on form submit', () => {
            formToggle().click();
            formPasswordInput().should('have.attr', 'type', 'text');
            cy.findByRole('button', {name: 'submit'}).click();
            formPasswordInput().should('have.attr', 'type', 'password');
            cy.findByTestId('form-result').should('have.text', 'submitted');
        });

        it('resets visibility to hidden on form reset', () => {
            formToggle().click();
            formPasswordInput().should('have.attr', 'type', 'text');
            cy.findByRole('button', {name: 'reset form'}).click();
            formPasswordInput().should('have.attr', 'type', 'password');
        });
    });

    // ── 9. Controlled Variant ───────────────────────────────

    describe('controlled variant', () => {
        it('external visible checkbox controls visibility', () => {
            passwordInput().should('have.attr', 'type', 'password');
            cy.findByLabelText('visible').click();
            passwordInput().should('have.attr', 'type', 'text');
            cy.findByLabelText('visible').click();
            passwordInput().should('have.attr', 'type', 'password');
        });

        it('toggling updates external visible checkbox', () => {
            cy.findByLabelText('visible').should('not.be.checked');
            cy.findByRole('button', {name: 'Show password'}).click();
            cy.findByLabelText('visible').should('be.checked');
            cy.findByRole('button', {name: 'Hide password'}).click();
            cy.findByLabelText('visible').should('not.be.checked');
        });
    });
});
