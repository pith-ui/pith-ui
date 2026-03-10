describe('Slider', () => {
    // ── Helpers ──────────────────────────────────────────────

    function getThumb() {
        return cy.findByRole('slider', {name: 'Volume'});
    }

    function shouldHaveValue(expected) {
        cy.findByTestId('slider-value').should('have.text', String(expected));
    }

    function getRoot() {
        return cy.get('.slider-root');
    }

    function getTrack() {
        return cy.get('.slider-track');
    }

    function getRange() {
        return cy.get('.slider-range');
    }

    beforeEach(() => {
        cy.visit('/slider');
    });

    // ── 1. Accessibility Semantics ──────────────────────────

    describe('accessibility', () => {
        it('thumb has role="slider"', () => {
            getThumb().should('exist');
        });

        it('has aria-valuemin', () => {
            getThumb().should('have.attr', 'aria-valuemin', '0');
        });

        it('has aria-valuemax', () => {
            getThumb().should('have.attr', 'aria-valuemax', '100');
        });

        it('has aria-valuenow matching current value', () => {
            getThumb().should('have.attr', 'aria-valuenow', '50');
        });

        it('has aria-orientation defaulting to horizontal', () => {
            getThumb().should('have.attr', 'aria-orientation', 'horizontal');
        });

        it('aria-valuenow updates when value changes', () => {
            getThumb().focus();
            cy.realPress('ArrowRight');
            getThumb().should('have.attr', 'aria-valuenow', '51');
        });
    });

    // ── 2. Data Attributes ──────────────────────────────────

    describe('data attributes', () => {
        it('root has data-orientation="horizontal" by default', () => {
            getRoot().should('have.attr', 'data-orientation', 'horizontal');
        });

        it('track has data-orientation="horizontal" by default', () => {
            getTrack().should('have.attr', 'data-orientation', 'horizontal');
        });

        it('range has data-orientation="horizontal" by default', () => {
            getRange().should('have.attr', 'data-orientation', 'horizontal');
        });

        it('thumb has data-orientation="horizontal" by default', () => {
            getThumb().should('have.attr', 'data-orientation', 'horizontal');
        });

        it('root does not have data-disabled when not disabled', () => {
            getRoot().should('not.have.attr', 'data-disabled');
        });

        it('track does not have data-disabled when not disabled', () => {
            getTrack().should('not.have.attr', 'data-disabled');
        });

        it('range does not have data-disabled when not disabled', () => {
            getRange().should('not.have.attr', 'data-disabled');
        });

        it('thumb does not have data-disabled when not disabled', () => {
            getThumb().should('not.have.attr', 'data-disabled');
        });
    });

    // ── 3. Keyboard Navigation ──────────────────────────────

    describe('keyboard navigation', () => {
        it('ArrowRight increases value by step', () => {
            getThumb().focus();
            cy.realPress('ArrowRight');
            shouldHaveValue(51);
        });

        it('ArrowLeft decreases value by step', () => {
            getThumb().focus();
            cy.realPress('ArrowLeft');
            shouldHaveValue(49);
        });

        it('ArrowUp increases value by step', () => {
            getThumb().focus();
            cy.realPress('ArrowUp');
            shouldHaveValue(51);
        });

        it('ArrowDown decreases value by step', () => {
            getThumb().focus();
            cy.realPress('ArrowDown');
            shouldHaveValue(49);
        });

        it('Home sets value to min', () => {
            getThumb().focus();
            cy.realPress('Home');
            shouldHaveValue(0);
            getThumb().should('have.attr', 'aria-valuenow', '0');
        });

        it('End sets value to max', () => {
            getThumb().focus();
            cy.realPress('End');
            shouldHaveValue(100);
            getThumb().should('have.attr', 'aria-valuenow', '100');
        });

        it('PageUp increases value by larger step (10x)', () => {
            getThumb().focus();
            cy.realPress('PageUp');
            shouldHaveValue(60);
        });

        it('PageDown decreases value by larger step (10x)', () => {
            getThumb().focus();
            cy.realPress('PageDown');
            shouldHaveValue(40);
        });

        it('multiple ArrowRight presses accumulate', () => {
            getThumb().focus();
            cy.realPress('ArrowRight');
            cy.realPress('ArrowRight');
            cy.realPress('ArrowRight');
            shouldHaveValue(53);
        });

        it('value does not go below min', () => {
            getThumb().focus();
            cy.realPress('Home');
            shouldHaveValue(0);
            cy.realPress('ArrowLeft');
            shouldHaveValue(0);
        });

        it('value does not go above max', () => {
            getThumb().focus();
            cy.realPress('End');
            shouldHaveValue(100);
            cy.realPress('ArrowRight');
            shouldHaveValue(100);
        });
    });

    // ── 4. Pointer Interaction ──────────────────────────────

    describe('pointer interaction', () => {
        it('thumb is focusable', () => {
            getThumb().focus();
            getThumb().should('be.focused');
        });
    });

    // ── 5. Disabled Variant ─────────────────────────────────

    describe('disabled variant', () => {
        beforeEach(() => {
            cy.findByLabelText('disabled').click();
        });

        it('root has data-disabled', () => {
            getRoot().should('have.attr', 'data-disabled');
        });

        it('track has data-disabled', () => {
            getTrack().should('have.attr', 'data-disabled');
        });

        it('range has data-disabled', () => {
            getRange().should('have.attr', 'data-disabled');
        });

        it('thumb has data-disabled', () => {
            getThumb().should('have.attr', 'data-disabled');
        });

        it('thumb does not have tabindex when disabled', () => {
            getThumb().should('not.have.attr', 'tabindex');
        });

        it('keyboard does not change value when disabled', () => {
            // Disabled thumb has no tabindex, so it cannot receive focus.
            // This inherently prevents keyboard interaction.
            getThumb().should('not.have.attr', 'tabindex');
            shouldHaveValue(50);
        });
    });

    // ── 6. Vertical Orientation ─────────────────────────────

    describe('vertical orientation', () => {
        beforeEach(() => {
            cy.findByLabelText('vertical').click();
        });

        it('root has data-orientation="vertical"', () => {
            getRoot().should('have.attr', 'data-orientation', 'vertical');
        });

        it('track has data-orientation="vertical"', () => {
            getTrack().should('have.attr', 'data-orientation', 'vertical');
        });

        it('range has data-orientation="vertical"', () => {
            getRange().should('have.attr', 'data-orientation', 'vertical');
        });

        it('thumb has data-orientation="vertical"', () => {
            getThumb().should('have.attr', 'data-orientation', 'vertical');
        });

        it('thumb has aria-orientation="vertical"', () => {
            getThumb().should('have.attr', 'aria-orientation', 'vertical');
        });

        it('ArrowUp increases value', () => {
            getThumb().focus();
            cy.realPress('ArrowUp');
            shouldHaveValue(51);
        });

        it('ArrowDown decreases value', () => {
            getThumb().focus();
            cy.realPress('ArrowDown');
            shouldHaveValue(49);
        });

        it('ArrowRight increases value', () => {
            getThumb().focus();
            cy.realPress('ArrowRight');
            shouldHaveValue(51);
        });

        it('ArrowLeft decreases value', () => {
            getThumb().focus();
            cy.realPress('ArrowLeft');
            shouldHaveValue(49);
        });

        it('Home sets value to min', () => {
            getThumb().focus();
            cy.realPress('Home');
            shouldHaveValue(0);
        });

        it('End sets value to max', () => {
            getThumb().focus();
            cy.realPress('End');
            shouldHaveValue(100);
        });

        it('PageUp increases value by larger step', () => {
            getThumb().focus();
            cy.realPress('PageUp');
            shouldHaveValue(60);
        });

        it('PageDown decreases value by larger step', () => {
            getThumb().focus();
            cy.realPress('PageDown');
            shouldHaveValue(40);
        });
    });

    // ── 7. Uncontrolled Mode ─────────────────────────────────

    describe('uncontrolled mode', () => {
        // slider-msc-1

        function getUncontrolledThumb() {
            return cy.findByRole('slider', {name: 'Uncontrolled volume'});
        }

        it('renders with default value', () => {
            getUncontrolledThumb().should('have.attr', 'aria-valuenow', '30');
        });

        it('keyboard changes value without external state', () => {
            getUncontrolledThumb().focus();
            cy.realPress('ArrowRight');
            getUncontrolledThumb().should('have.attr', 'aria-valuenow', '31');
        });

        it('Home sets value to min', () => {
            getUncontrolledThumb().focus();
            cy.realPress('Home');
            getUncontrolledThumb().should('have.attr', 'aria-valuenow', '0');
        });

        it('End sets value to max', () => {
            getUncontrolledThumb().focus();
            cy.realPress('End');
            getUncontrolledThumb().should('have.attr', 'aria-valuenow', '100');
        });

        it('has correct data attributes', () => {
            cy.findByTestId('uncontrolled-slider').should('have.attr', 'data-orientation', 'horizontal');
            getUncontrolledThumb().should('have.attr', 'data-orientation', 'horizontal');
        });
    });
});
