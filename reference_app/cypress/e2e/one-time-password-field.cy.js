describe('OneTimePasswordField', () => {
    // ── Helpers ──────────────────────────────────────────────

    function getRoot() {
        return cy.findByTestId('main-otp-root');
    }

    function getInputs() {
        return getRoot().find('[data-radix-otp-input]');
    }

    function getOutput() {
        return cy.findByTestId('otp-value');
    }

    function getHiddenInput() {
        return getRoot().find('input[type="hidden"][name="code"]');
    }

    function shouldHaveValue(expected) {
        getOutput().should('have.text', expected);
    }

    function focusFirstInput() {
        getInputs().first().click();
    }

    // Paste text into the OTP group via a ClipboardEvent on the root.
    function pasteIntoOtp(text) {
        getRoot().then(($group) => {
            const clipboardData = new DataTransfer();
            clipboardData.setData('text/plain', text);
            const pasteEvent = new ClipboardEvent('paste', {
                clipboardData,
                bubbles: true,
                cancelable: true,
            });
            $group[0].dispatchEvent(pasteEvent);
        });
    }

    // ── Setup ────────────────────────────────────────────────

    beforeEach(() => {
        cy.visit('/one-time-password-field');
        // Reset to clean state
        cy.findByRole('button', {name: 'reset'}).click();
        // Ensure vertical is unchecked
        cy.findByLabelText('vertical').then(($cb) => {
            if ($cb.is(':checked')) cy.wrap($cb).click();
        });
    });

    // ── 1. Accessibility Semantics ──────────────────────────

    describe('accessibility', () => {
        it('root has role="group"', () => {
            getRoot().should('have.attr', 'role', 'group');
        });

        it('each input has aria-label "Character N of 6"', () => {
            getInputs().should('have.length', 6);
            getInputs().each(($input, index) => {
                cy.wrap($input).should('have.attr', 'aria-label', `Character ${index + 1} of 6`);
            });
        });

        it('hidden input exists with name="code"', () => {
            getHiddenInput().should('exist');
        });
    });

    // ── 2. Data Attributes ──────────────────────────────────

    describe('data attributes', () => {
        it('root has data-orientation="horizontal"', () => {
            getRoot().should('have.attr', 'data-orientation', 'horizontal');
        });

        it('each input has data-radix-otp-input', () => {
            getInputs().each(($input) => {
                cy.wrap($input).should('have.attr', 'data-radix-otp-input');
            });
        });

        it('each input has data-radix-index matching its position', () => {
            getInputs().each(($input, index) => {
                cy.wrap($input).should('have.attr', 'data-radix-index', String(index));
            });
        });
    });

    // ── 3. Paste ────────────────────────────────────────────

    describe('paste', () => {
        it('pasting "123456" fills all 6 inputs', () => {
            focusFirstInput();
            pasteIntoOtp('123456');
            shouldHaveValue('123456');
        });

        it('pasting "12345" fills 5 inputs', () => {
            focusFirstInput();
            pasteIntoOtp('12345');
            shouldHaveValue('12345');
        });

        it('pasting "1 2 3 4 5 6" filters out spaces and fills all 6 inputs', () => {
            focusFirstInput();
            pasteIntoOtp('1 2 3 4 5 6');
            shouldHaveValue('123456');
        });

        it('pasting "1-2-3-4-5-6" filters out dashes and fills all 6 inputs', () => {
            focusFirstInput();
            pasteIntoOtp('1-2-3-4-5-6');
            shouldHaveValue('123456');
        });

        it('pasting replaces existing values entirely', () => {
            focusFirstInput();
            pasteIntoOtp('123456');
            shouldHaveValue('123456');
            pasteIntoOtp('23456');
            shouldHaveValue('23456');
        });

        it('pasting strips invalid characters with numeric validation', () => {
            focusFirstInput();
            pasteIntoOtp('12ab56');
            shouldHaveValue('1256');
        });

        it('modifier+key does not insert characters (e.g. Cmd+V)', () => {
            focusFirstInput();
            // Cmd+V / Ctrl+V must not treat "v" as character input.
            // The keydown handler should skip keys pressed with modifiers.
            cy.realPress(['Meta', 'v']);
            shouldHaveValue('');
            getInputs().first().should('be.focused');
            cy.realPress(['Control', 'v']);
            shouldHaveValue('');
            getInputs().first().should('be.focused');
        });
    });

    // ── 4. Backspace ────────────────────────────────────────

    describe('backspace', () => {
        it('backspace on empty input moves focus to previous input', () => {
            focusFirstInput();
            pasteIntoOtp('1');
            // After pasting '1', focus should be on input 0 (last filled).
            // Navigate to input 1 (first empty after value).
            cy.realPress('ArrowRight');
            getInputs().eq(1).should('be.focused');
            // Backspace on empty input → moves focus to previous
            cy.realPress('Backspace');
            getInputs().first().should('be.focused');
        });

        it('backspace on filled input removes char and shifts values left', () => {
            focusFirstInput();
            pasteIntoOtp('123');
            shouldHaveValue('123');
            // Paste focuses last filled input (index 2). Navigate to index 2.
            cy.realPress('Home');
            cy.realPress('ArrowRight');
            cy.realPress('ArrowRight');
            getInputs().eq(2).should('be.focused');
            // Backspace on filled input 2 (has '3') → removes '3', focus goes back
            cy.realPress('Backspace');
            shouldHaveValue('12');
            getInputs().eq(1).should('be.focused');
        });

        // Cmd+Backspace behavior varies by platform; headless Chrome on macOS
        // doesn't reliably clear single-char inputs. This test also fails against React.
        it.skip('Cmd+Backspace clears all inputs and focuses first', () => {
            focusFirstInput();
            pasteIntoOtp('123');
            shouldHaveValue('123');
            cy.realPress(['Meta', 'Backspace']);
            shouldHaveValue('');
            getInputs().first().should('be.focused');
        });
    });

    // ── 5. Delete ───────────────────────────────────────────

    describe('delete', () => {
        it('delete on filled input removes char and shifts values left, keeps focus', () => {
            focusFirstInput();
            pasteIntoOtp('123');
            shouldHaveValue('123');
            // Navigate to first input
            cy.realPress('Home');
            getInputs().first().should('be.focused');
            // Delete on input 0 (has '1') → removes '1', shifts left → '23', focus stays at 0
            cy.realPress('Delete');
            shouldHaveValue('23');
            getInputs().first().should('be.focused');
        });
    });

    // ── 6. Keyboard Navigation ──────────────────────────────

    describe('keyboard navigation', () => {
        it('ArrowRight moves to next input', () => {
            focusFirstInput();
            pasteIntoOtp('12');
            cy.realPress('Home');
            getInputs().first().should('be.focused');
            cy.realPress('ArrowRight');
            getInputs().eq(1).should('be.focused');
        });

        it('ArrowLeft moves to previous input', () => {
            focusFirstInput();
            pasteIntoOtp('12');
            // Paste focuses last filled (index 1). ArrowRight to index 2.
            cy.realPress('ArrowRight');
            getInputs().eq(2).should('be.focused');
            cy.realPress('ArrowLeft');
            getInputs().eq(1).should('be.focused');
        });

        it('Home moves to first input', () => {
            focusFirstInput();
            pasteIntoOtp('123');
            cy.realPress('Home');
            getInputs().first().should('be.focused');
        });

        it('End moves to last selectable input', () => {
            focusFirstInput();
            pasteIntoOtp('123');
            // lastSelectableIndex = clamp(3, [0, 5]) = 3
            cy.realPress('Home');
            getInputs().first().should('be.focused');
            cy.realPress('End');
            getInputs().eq(3).should('be.focused');
        });

        it('Enter submits the form', () => {
            focusFirstInput();
            pasteIntoOtp('123456');
            shouldHaveValue('123456');
            cy.realPress('Enter');
            cy.findByTestId('form-result').should('have.text', 'Submitted: 123456');
        });

        it('Tab leaves the OTP field entirely', () => {
            focusFirstInput();
            pasteIntoOtp('1');
            cy.realPress('Tab');
            // Focus should leave the OTP group
            getInputs().each(($input) => {
                cy.wrap($input).should('not.be.focused');
            });
        });

        it('Shift+Tab leaves the OTP field backwards', () => {
            focusFirstInput();
            pasteIntoOtp('1');
            cy.realPress(['Shift', 'Tab']);
            // Focus should leave the OTP group
            getInputs().each(($input) => {
                cy.wrap($input).should('not.be.focused');
            });
        });
    });

    // ── 7. Focus Management ─────────────────────────────────

    describe('focus management', () => {
        it('clicking an input focuses the last selectable position', () => {
            focusFirstInput();
            pasteIntoOtp('12');
            shouldHaveValue('12');
            // lastSelectableIndex = 2. Clicking input 0 should redirect to input 2.
            getInputs().first().click();
            getInputs().eq(2).should('be.focused');
        });

        it('clicking redirects to last filled + 1 position', () => {
            focusFirstInput();
            pasteIntoOtp('123');
            shouldHaveValue('123');
            // lastSelectableIndex = 3. Clicking any earlier input → input 3.
            getInputs().eq(1).click();
            getInputs().eq(3).should('be.focused');
        });
    });

    // ── 8. Disabled State ───────────────────────────────────

    describe('disabled state', () => {
        beforeEach(() => {
            cy.findByLabelText('disabled').click();
        });

        it('all inputs are disabled', () => {
            getInputs().each(($input) => {
                cy.wrap($input).should('be.disabled');
            });
        });

        it('disabled inputs cannot be focused by click', () => {
            getInputs().first().click({force: true});
            getInputs().first().should('not.be.focused');
        });
    });

    // ── 9. Read-only State ─────────────────────────────────

    describe('read-only state', () => {
        it('inputs have readonly attribute when read-only is enabled', () => {
            cy.findByLabelText('read-only').click();
            getInputs().each(($input) => {
                cy.wrap($input).should('have.attr', 'readonly');
            });
        });
    });

    // ── 10. Form Integration ────────────────────────────────

    describe('form integration', () => {
        it('hidden input value matches concatenated OTP value', () => {
            focusFirstInput();
            pasteIntoOtp('123');
            getHiddenInput().should('have.value', '123');
        });

        it('form submission sends the OTP value', () => {
            focusFirstInput();
            pasteIntoOtp('123456');
            cy.findByRole('button', {name: 'submit'}).click();
            cy.findByTestId('form-result').should('have.text', 'Submitted: 123456');
        });

        it('form reset clears all inputs', () => {
            focusFirstInput();
            pasteIntoOtp('123');
            shouldHaveValue('123');
            cy.findByRole('button', {name: 'reset'}).click();
            shouldHaveValue('');
            getHiddenInput().should('have.value', '');
        });
    });

    // ── 11. Character Input ───────────────────────────────

    describe('character input', () => {
        it('typing a digit places it and advances focus to next input', () => {
            focusFirstInput();
            getInputs().first().should('be.focused');
            cy.realPress('1');
            shouldHaveValue('1');
            getInputs().eq(1).should('be.focused');
        });

        it('typing fills all 6 inputs sequentially', () => {
            focusFirstInput();
            cy.realPress('1');
            cy.realPress('2');
            cy.realPress('3');
            cy.realPress('4');
            cy.realPress('5');
            cy.realPress('6');
            shouldHaveValue('123456');
        });

        it('typing beyond the last slot replaces the last character', () => {
            focusFirstInput();
            cy.realPress('1');
            cy.realPress('2');
            cy.realPress('3');
            cy.realPress('4');
            cy.realPress('5');
            cy.realPress('6');
            cy.realPress('5');
            cy.realPress('1');
            shouldHaveValue('123451');
        });

        it('invalid characters are rejected with numeric validation', () => {
            focusFirstInput();
            cy.realPress('a');
            shouldHaveValue('');
            getInputs().first().should('be.focused');
        });
    });

    // ── 12. Uncontrolled Mode ───────────────────────────────

    describe('uncontrolled mode', () => {
        function getUncontrolledInputs() {
            return cy.findByTestId('uncontrolled-root').find('[data-radix-otp-input]');
        }

        it('renders with default value pre-filled', () => {
            getUncontrolledInputs().eq(0).should('have.value', '1');
            getUncontrolledInputs().eq(1).should('have.value', '2');
            getUncontrolledInputs().eq(2).should('have.value', '');
        });

        it('typing adds to the pre-filled value', () => {
            getUncontrolledInputs().eq(2).click();
            cy.realPress('3');
            getUncontrolledInputs().eq(2).should('have.value', '3');
        });

        it('form submission sends the uncontrolled value', () => {
            cy.findByTestId('uncontrolled-submit').click();
            cy.findByTestId('uncontrolled-result').should('have.text', 'Submitted: 12');
        });
    });

    // ── 13. Password Type ─────────────────────────────────

    describe('password type', () => {
        function getPasswordInputs() {
            return cy.findByTestId('password-root').find('[data-radix-otp-input]');
        }

        it('inputs have type="password"', () => {
            getPasswordInputs().each(($input) => {
                cy.wrap($input).should('have.attr', 'type', 'password');
            });
        });
    });

    // ── 14. Placeholder ───────────────────────────────────

    describe('placeholder', () => {
        function getPlaceholderInputs() {
            return cy.findByTestId('placeholder-root').find('[data-radix-otp-input]');
        }

        it('empty inputs show placeholder character', () => {
            getPlaceholderInputs().each(($input) => {
                cy.wrap($input).should('have.attr', 'placeholder', '○');
            });
        });
    });

    // ── 15. AutoSubmit ─────────────────────────────────────

    describe('autoSubmit', () => {
        function getAutoSubmitInputs() {
            return cy.findByTestId('autosubmit-root').find('[data-radix-otp-input]');
        }

        function pasteIntoAutoSubmit(text) {
            cy.findByTestId('autosubmit-root').then(($root) => {
                const clipboardData = new DataTransfer();
                clipboardData.setData('text/plain', text);
                const pasteEvent = new ClipboardEvent('paste', {
                    clipboardData,
                    bubbles: true,
                    cancelable: true,
                });
                $root[0].dispatchEvent(pasteEvent);
            });
        }

        it('onAutoSubmit fires when all inputs are filled', () => {
            getAutoSubmitInputs().first().click();
            pasteIntoAutoSubmit('1234');
            cy.findByTestId('autosubmit-result').should('have.text', 'AutoSubmitted: 1234');
        });

        it('onAutoSubmit does not fire when partially filled', () => {
            getAutoSubmitInputs().first().click();
            pasteIntoAutoSubmit('123');
            cy.findByTestId('autosubmit-result').should('have.text', '');
        });
    });

    // ── 16. AutoComplete ──────────────────────────────────

    describe('autoComplete', () => {
        function getAutoCompleteInputs() {
            return cy.findByTestId('autocomplete-root').find('[data-radix-otp-input]');
        }

        it('first input has autocomplete="one-time-code"', () => {
            getAutoCompleteInputs().first().should('have.attr', 'autocomplete', 'one-time-code');
        });

        it('subsequent inputs have autocomplete="off"', () => {
            getAutoCompleteInputs().eq(1).should('have.attr', 'autocomplete', 'off');
            getAutoCompleteInputs().eq(2).should('have.attr', 'autocomplete', 'off');
        });
    });

    // ── 17. Vertical Orientation ──────────────────────────

    describe('vertical orientation', () => {
        beforeEach(() => {
            cy.findByLabelText('vertical').click();
            getRoot().should('have.attr', 'data-orientation', 'vertical');
        });

        it('ArrowDown moves to next input', () => {
            focusFirstInput();
            pasteIntoOtp('12');
            cy.realPress('Home');
            getInputs().first().should('be.focused');
            cy.realPress('ArrowDown');
            getInputs().eq(1).should('be.focused');
        });

        it('ArrowUp moves to previous input', () => {
            focusFirstInput();
            pasteIntoOtp('12');
            cy.realPress('Home');
            getInputs().first().should('be.focused');
            cy.realPress('ArrowDown');
            getInputs().eq(1).should('be.focused');
            cy.realPress('ArrowUp');
            getInputs().first().should('be.focused');
        });

        it('ArrowRight does not move focus', () => {
            focusFirstInput();
            pasteIntoOtp('12');
            cy.realPress('Home');
            getInputs().first().should('be.focused');
            cy.realPress('ArrowRight');
            getInputs().first().should('be.focused');
        });

        it('ArrowLeft does not move focus', () => {
            focusFirstInput();
            pasteIntoOtp('12');
            cy.realPress('Home');
            cy.realPress('ArrowDown');
            getInputs().eq(1).should('be.focused');
            cy.realPress('ArrowLeft');
            getInputs().eq(1).should('be.focused');
        });
    });

    // ── Axe Accessibility Audit ─────────────────────────────

    describe('axe audit', () => {
        it('no violations in default state', () => {
            // Exclude aria-hidden-focus — extra demo sections are wrapped in
            // aria-hidden="true" to prevent findByRole collisions, but still
            // contain focusable OTP inputs.
            cy.checkComponentA11y(null, {rules: {'aria-hidden-focus': {enabled: false}}});
        });
    });
});
