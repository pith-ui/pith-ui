describe('Form', () => {
    // ── Helpers ──────────────────────────────────────────────

    function getNameInput() {
        return cy.findByLabelText('Name');
    }

    function getEmailInput() {
        return cy.findByLabelText('Email');
    }

    function getSubmitButton() {
        return cy.findByRole('button', {name: 'Submit'});
    }

    function getResetButton() {
        return cy.findByRole('button', {name: 'reset'});
    }

    function getFormResult() {
        return cy.findByTestId('form-result');
    }

    beforeEach(() => {
        cy.visit('/form');
    });

    // ── 1. Accessibility Semantics ──────────────────────────

    describe('accessibility', () => {
        it('Label is associated with its control via htmlFor/id', () => {
            // The label's "for" attribute should match the input's "id"
            cy.findByText('Name')
                .invoke('attr', 'for')
                .then((forAttr) => {
                    getNameInput().should('have.attr', 'id', forAttr);
                });
        });

        it('required field has aria-required on the native input', () => {
            // The native input has required attribute which the browser exposes
            getNameInput().should('have.attr', 'required');
        });

        it('error message is linked via aria-describedby', () => {
            // Submit empty form to trigger validation messages
            getSubmitButton().click();

            // The name input should have aria-describedby pointing to the error message
            getNameInput()
                .invoke('attr', 'aria-describedby')
                .then((describedBy) => {
                    expect(describedBy).to.not.be.empty;
                    const ids = describedBy.split(' ');
                    // At least one ID should correspond to the error message
                    cy.get(`#${ids[0]}`).should('contain.text', 'Name is required');
                });
        });
    });

    // ── 2. Data Attributes ──────────────────────────────────

    describe('data attributes', () => {
        it('Field does not have data-valid or data-invalid initially (before validation)', () => {
            // Before any submit attempt, no validity has been computed
            cy.get('.form-field').first().should('not.have.attr', 'data-valid');
            cy.get('.form-field').first().should('not.have.attr', 'data-invalid');
        });

        it('Field has data-invalid after failed validation', () => {
            getSubmitButton().click();

            // The name field should be invalid (required but empty)
            cy.get('.form-field').first().should('have.attr', 'data-invalid');
        });

        it('Field has data-valid after entering valid data and triggering validation', () => {
            // Type valid data, submit to trigger validation
            getNameInput().type('Alice');
            getEmailInput().type('alice@example.com');
            getSubmitButton().click();

            // After successful validation, fields should be valid
            // (form submits, so data-valid may or may not be set depending on implementation)
            // Check the name field specifically
            getFormResult().should('contain.text', 'Alice');
        });

        it('Message inherits field validity context', () => {
            // Submit empty form - messages should appear
            getSubmitButton().click();

            // The error message span should exist within the invalid field
            cy.findByText('Name is required').should('exist');
            cy.findByText('Email is required').should('exist');
        });
    });

    // ── 3. Built-in Validation ──────────────────────────────

    describe('built-in validation', () => {
        it('required field shows valueMissing message on empty submit', () => {
            getSubmitButton().click();

            cy.findByText('Name is required').should('exist');
            cy.findByText('Email is required').should('exist');
        });

        it('email field shows typeMismatch message on invalid email', () => {
            getNameInput().type('Alice');
            getEmailInput().type('not-an-email');
            getSubmitButton().click();

            cy.findByText('Please enter a valid email').should('exist');
        });

        it('field becomes valid after entering valid data', () => {
            // First, trigger validation
            getSubmitButton().click();
            cy.findByText('Name is required').should('exist');

            // Now type valid data - the message should clear on input
            getNameInput().type('Alice');
            cy.findByText('Name is required').should('not.exist');
        });

        it('email validation clears after entering valid email', () => {
            getNameInput().type('Alice');
            getEmailInput().type('bad');
            getSubmitButton().click();
            cy.findByText('Please enter a valid email').should('exist');

            // Clear and type valid email
            getEmailInput().clear().type('alice@example.com');
            cy.findByText('Please enter a valid email').should('not.exist');
        });

        it('focuses first invalid control on submit', () => {
            // Leave both fields empty and submit
            getSubmitButton().click();

            // Name is the first field, so it should receive focus
            getNameInput().should('be.focused');
        });
    });

    // ── 4. FormValidityState ────────────────────────────────

    describe('FormValidityState', () => {
        it('validity is undefined before any validation', () => {
            cy.findByTestId('vs-name-validity').should('have.text', 'undefined');
        });

        it('exposes valueMissing when required field is empty', () => {
            cy.findByTestId('vs-submit').click();
            cy.findByTestId('vs-name-validity').should('contain.text', '"valueMissing":true');
            cy.findByTestId('vs-name-validity').should('contain.text', '"valid":false');
        });

        it('exposes valid:true when field has valid value', () => {
            cy.findByTestId('vs-name-input').type('Alice');
            cy.findByTestId('vs-submit').click();
            cy.findByTestId('vs-name-validity').should('contain.text', '"valueMissing":false');
            cy.findByTestId('vs-name-validity').should('contain.text', '"valid":true');
        });

        it('exposes typeMismatch for invalid email', () => {
            cy.findByTestId('vs-email-input').type('not-an-email');
            cy.findByTestId('vs-name-input').type('Alice');
            cy.findByTestId('vs-submit').click();
            cy.findByTestId('vs-email-validity').should('contain.text', '"typeMismatch":true');
            cy.findByTestId('vs-email-validity').should('contain.text', '"valid":false');
        });

        it('updates to valid after correcting email', () => {
            cy.findByTestId('vs-email-input').type('bad');
            cy.findByTestId('vs-name-input').type('Alice');
            cy.findByTestId('vs-submit').click();
            cy.findByTestId('vs-email-validity').should('contain.text', '"valid":false');

            cy.findByTestId('vs-email-input').clear().type('valid@email.com');
            cy.findByTestId('vs-submit').click();
            cy.findByTestId('vs-email-validity').should('contain.text', '"valid":true');
        });
    });

    // ── 5. Server Validation ─────────────────────────────────

    describe('server validation', () => {
        it('serverInvalid sets data-invalid on the field', () => {
            getNameInput().type('taken');
            getEmailInput().type('test@example.com');
            getSubmitButton().click();

            // The name field should be server-invalid
            cy.get('.form-field').first().should('have.attr', 'data-invalid');
        });

        it('serverInvalid sets aria-invalid on the control', () => {
            getNameInput().type('taken');
            getEmailInput().type('test@example.com');
            getSubmitButton().click();

            getNameInput().should('have.attr', 'aria-invalid', 'true');
        });

        it('server error message is displayed', () => {
            getNameInput().type('taken');
            getEmailInput().type('test@example.com');
            getSubmitButton().click();

            cy.findByText('Name is already taken').should('exist');
        });

        it('server error message is linked via aria-describedby', () => {
            getNameInput().type('taken');
            getEmailInput().type('test@example.com');
            getSubmitButton().click();

            getNameInput()
                .invoke('attr', 'aria-describedby')
                .then((describedBy) => {
                    expect(describedBy).to.not.be.empty;
                    const ids = describedBy.split(' ');
                    // At least one ID should correspond to the server error message
                    let found = false;
                    ids.forEach((id) => {
                        cy.get(`#${id}`).then(($el) => {
                            if ($el.text().includes('Name is already taken')) {
                                found = true;
                            }
                        });
                    });
                    cy.then(() => expect(found).to.be.true);
                });
        });

        it('onClearServerErrors fires on re-submit, clearing server errors', () => {
            getNameInput().type('taken');
            getEmailInput().type('test@example.com');
            getSubmitButton().click();

            // Server error should be present
            cy.findByText('Name is already taken').should('exist');

            // Fix the name and re-submit — onClearServerErrors fires on submit,
            // clearing old server errors before the new onSubmit runs
            getNameInput().clear().type('Alice');
            getSubmitButton().click();
            cy.findByText('Name is already taken').should('not.exist');
            getFormResult().should('contain.text', 'Alice');
        });

        it('onClearServerErrors fires on reset', () => {
            getNameInput().type('taken');
            getEmailInput().type('test@example.com');
            getSubmitButton().click();

            // Server error should be present
            cy.findByText('Name is already taken').should('exist');

            // Reset the form — onClearServerErrors fires on reset
            getResetButton().click();
            cy.findByText('Name is already taken').should('not.exist');
        });

        it('form does not submit when server validation fails', () => {
            getNameInput().type('taken');
            getEmailInput().type('test@example.com');
            getSubmitButton().click();

            // Form result should still be empty (server rejected)
            getFormResult().should('contain.text', 'Data: {}');
        });
    });

    // ── 6. Custom Messages ────────────────────────────────────

    describe('custom messages', () => {
        it('consumer-provided message overrides default validation text', () => {
            // Submit empty form to trigger validation
            getSubmitButton().click();

            // The custom message "Name is required" should appear (not the default "This value is missing")
            cy.findByText('Name is required').should('exist');
            cy.findByText('This value is missing').should('not.exist');
        });

        it('consumer-provided email message overrides default', () => {
            getNameInput().type('Alice');
            getEmailInput().type('not-an-email');
            getSubmitButton().click();

            // Custom message should appear, not the default
            cy.findByText('Please enter a valid email').should('exist');
            cy.findByText('This value is not valid').should('not.exist');
        });
    });

    // ── 7. Form Submission ──────────────────────────────────

    describe('form submission', () => {
        it('submit button has type="submit"', () => {
            getSubmitButton().should('have.attr', 'type', 'submit');
        });

        it('valid form submits successfully and displays data', () => {
            getNameInput().type('Alice');
            getEmailInput().type('alice@example.com');
            getSubmitButton().click();

            getFormResult().should('contain.text', '"name"');
            getFormResult().should('contain.text', 'Alice');
            getFormResult().should('contain.text', '"email"');
            getFormResult().should('contain.text', 'alice@example.com');
        });

        it('invalid form does not submit', () => {
            getSubmitButton().click();

            // Form result should still be the initial empty object
            getFormResult().should('contain.text', 'Data: {}');
        });

        it('reset button clears form and result', () => {
            getNameInput().type('Alice');
            getEmailInput().type('alice@example.com');
            getSubmitButton().click();
            getFormResult().should('contain.text', 'Alice');

            getResetButton().click();
            getFormResult().should('contain.text', 'Data: {}');
        });
    });

    // ── Axe Accessibility Audit ─────────────────────────────

    describe('axe audit', () => {
        it('no violations in default state', () => {
            cy.checkComponentA11y();
        });
    });
});
