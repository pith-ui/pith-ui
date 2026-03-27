describe('Progress', () => {
    // ── Helpers ──────────────────────────────────────────────

    function getRoot() {
        return cy.findByRole('progressbar');
    }

    function getIndicator() {
        return cy.findByTestId('progress-indicator');
    }

    function getValueDisplay() {
        return cy.findByTestId('progress-value');
    }

    beforeEach(() => {
        cy.visit('/progress');
    });

    // ── 1. Accessibility Semantics ──────────────────────────

    describe('accessibility', () => {
        it('has role="progressbar"', () => {
            getRoot().should('exist');
        });

        it('has aria-valuenow reflecting current value', () => {
            getRoot().should('have.attr', 'aria-valuenow', '30');
        });

        it('has aria-valuemin="0"', () => {
            getRoot().should('have.attr', 'aria-valuemin', '0');
        });

        it('has aria-valuemax reflecting max', () => {
            getRoot().should('have.attr', 'aria-valuemax', '100');
        });

        it('has aria-valuetext with percentage label', () => {
            getRoot().should('have.attr', 'aria-valuetext', '30%');
        });

        it('aria-valuenow is absent when indeterminate', () => {
            cy.findByText('set indeterminate').click();
            getRoot().should('not.have.attr', 'aria-valuenow');
        });

        it('aria-valuetext is absent when indeterminate', () => {
            cy.findByText('set indeterminate').click();
            getRoot().should('not.have.attr', 'aria-valuetext');
        });
    });

    // ── 2. Data Attributes ──────────────────────────────────

    describe('data attributes', () => {
        describe('root', () => {
            it('data-state is "loading" when value < max', () => {
                getRoot().should('have.attr', 'data-state', 'loading');
            });

            it('data-state is "complete" when value = max', () => {
                cy.findByText('set complete').click();
                getRoot().should('have.attr', 'data-state', 'complete');
            });

            it('data-state is "indeterminate" when value is null', () => {
                cy.findByText('set indeterminate').click();
                getRoot().should('have.attr', 'data-state', 'indeterminate');
            });

            it('data-value reflects current value', () => {
                getRoot().should('have.attr', 'data-value', '30');
            });

            it('data-value is absent when indeterminate', () => {
                cy.findByText('set indeterminate').click();
                getRoot().should('not.have.attr', 'data-value');
            });

            it('data-max reflects max value', () => {
                getRoot().should('have.attr', 'data-max', '100');
            });
        });

        describe('indicator', () => {
            it('data-state matches root data-state when loading', () => {
                getIndicator().should('have.attr', 'data-state', 'loading');
            });

            it('data-state matches root data-state when complete', () => {
                cy.findByText('set complete').click();
                getIndicator().should('have.attr', 'data-state', 'complete');
            });

            it('data-state matches root data-state when indeterminate', () => {
                cy.findByText('set indeterminate').click();
                getIndicator().should('have.attr', 'data-state', 'indeterminate');
            });

            it('data-value matches root data-value', () => {
                getIndicator().should('have.attr', 'data-value', '30');
            });

            it('data-value is absent when indeterminate', () => {
                cy.findByText('set indeterminate').click();
                getIndicator().should('not.have.attr', 'data-value');
            });

            it('data-max matches root data-max', () => {
                getIndicator().should('have.attr', 'data-max', '100');
            });
        });
    });

    // ── 3. Value Updates ────────────────────────────────────

    describe('value updates', () => {
        it('clicking increment updates value by 10', () => {
            getValueDisplay().should('have.text', '30');
            cy.findByText('increment').click();
            getValueDisplay().should('have.text', '40');
            getRoot().should('have.attr', 'aria-valuenow', '40');
            getRoot().should('have.attr', 'data-value', '40');
        });

        it('multiple increments update correctly', () => {
            cy.findByText('increment').click();
            cy.findByText('increment').click();
            cy.findByText('increment').click();
            getValueDisplay().should('have.text', '60');
            getRoot().should('have.attr', 'aria-valuenow', '60');
        });

        it('value cannot exceed max', () => {
            // Start at 30, click increment 8 times to try to exceed 100
            for (let i = 0; i < 8; i++) {
                cy.findByText('increment').click();
            }
            getValueDisplay().should('have.text', '100');
            getRoot().should('have.attr', 'aria-valuenow', '100');
            getRoot().should('have.attr', 'data-state', 'complete');
        });

        it('set complete sets value to max and state to complete', () => {
            cy.findByText('set complete').click();
            getValueDisplay().should('have.text', '100');
            getRoot().should('have.attr', 'aria-valuenow', '100');
            getRoot().should('have.attr', 'data-state', 'complete');
            getRoot().should('have.attr', 'data-value', '100');
        });

        it('set indeterminate sets state to indeterminate', () => {
            cy.findByText('set indeterminate').click();
            getValueDisplay().should('have.text', 'indeterminate');
            getRoot().should('have.attr', 'data-state', 'indeterminate');
            getRoot().should('not.have.attr', 'data-value');
            getRoot().should('not.have.attr', 'aria-valuenow');
        });

        it('increment from indeterminate sets value to 10', () => {
            cy.findByText('set indeterminate').click();
            getValueDisplay().should('have.text', 'indeterminate');
            cy.findByText('increment').click();
            getValueDisplay().should('have.text', '10');
            getRoot().should('have.attr', 'data-state', 'loading');
            getRoot().should('have.attr', 'aria-valuenow', '10');
        });

        it('negative value becomes indeterminate', () => {
            cy.findByTestId('set-negative').click();
            getRoot().should('have.attr', 'data-state', 'indeterminate');
            getRoot().should('not.have.attr', 'aria-valuenow');
            getRoot().should('not.have.attr', 'data-value');
        });

        it('value exceeding max becomes indeterminate', () => {
            cy.findByTestId('set-over-max').click();
            getRoot().should('have.attr', 'data-state', 'indeterminate');
            getRoot().should('not.have.attr', 'aria-valuenow');
            getRoot().should('not.have.attr', 'data-value');
        });

        it('setting value to max transitions to complete state', () => {
            // progress-dcnv-1: test behavioral contract (data attributes) instead of visual style
            cy.findByText('set complete').click();
            getRoot().should('have.attr', 'data-state', 'complete');
            getRoot().should('have.attr', 'data-value', '100');
            getRoot().should('have.attr', 'aria-valuenow', '100');
        });
    });

    // ── 4. Attribute Forwarding ─────────────────────────────

    describe('attribute forwarding', () => {
        it('root forwards className/class to the DOM element', () => {
            getRoot().should('have.class', 'progress-root');
        });

        it('root forwards custom data attributes to the DOM element', () => {
            getRoot().should('have.attr', 'data-custom', 'progress-root-custom');
        });

        it('root preserves component attributes alongside forwarded attributes', () => {
            getRoot()
                .should('have.attr', 'role', 'progressbar')
                .and('have.attr', 'data-state', 'loading')
                .and('have.class', 'progress-root')
                .and('have.attr', 'data-custom', 'progress-root-custom');
        });

        it('indicator forwards data-testid to the DOM element', () => {
            getIndicator().should('exist');
        });

        it('indicator forwards className/class to the DOM element', () => {
            getIndicator().should('have.class', 'progress-indicator');
        });

        it('indicator forwards custom data attributes to the DOM element', () => {
            getIndicator().should('have.attr', 'data-custom', 'progress-indicator-custom');
        });

        it('indicator preserves component attributes alongside forwarded attributes', () => {
            getIndicator()
                .should('have.attr', 'data-state', 'loading')
                .and('have.class', 'progress-indicator')
                .and('have.attr', 'data-custom', 'progress-indicator-custom');
        });
    });

    // ── Axe Accessibility Audit ─────────────────────────────

    describe('axe audit', () => {
        it('no violations in default state', () => {
            cy.checkComponentA11y();
        });
    });
});
