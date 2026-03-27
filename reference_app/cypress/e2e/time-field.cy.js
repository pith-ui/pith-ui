describe('TimeField', () => {
    // ── Helpers ──────────────────────────────────────────────

    function seg(type) {
        return cy.get(`[role='spinbutton'][data-type='${type}']`);
    }

    function hourSeg() { return seg('hour'); }
    function minuteSeg() { return seg('minute'); }
    function secondSeg() { return seg('second'); }
    function periodSeg() { return seg('day-period'); }

    function readout() { return cy.get('[data-testid="time-value"]'); }

    function reset() {
        cy.get('[data-testid="reset"]').click();
    }

    beforeEach(() => {
        cy.visit('/time-field');
        // Wait for reset-key re-mount to settle after visit
        reset();
        hourSeg().should('have.attr', 'data-placeholder');
    });

    // All tests skip for React — Radix UI has no TimeField primitive.
    // This wrapper makes every `it()` call skip when FRAMEWORK=react.
    const test = it.skipForFramework('react', 'Radix has no TimeField primitive');

    // ── 1. Accessibility Semantics ──────────────────────────

    describe('accessibility', () => {
        test('group container has role="group"', () => {
            cy.get('[role="group"]').should('exist');
        });

        test('group has an accessible label', () => {
            cy.get('[role="group"]').should('have.attr', 'aria-label', 'Time');
        });

        test('hour segment has role="spinbutton"', () => {
            hourSeg().should('exist');
            hourSeg().should('have.attr', 'aria-label', 'hour');
        });

        test('minute segment has role="spinbutton"', () => {
            minuteSeg().should('exist');
            minuteSeg().should('have.attr', 'aria-label', 'minute');
        });

        test('day-period segment has role="spinbutton"', () => {
            periodSeg().should('exist');
            periodSeg().should('have.attr', 'aria-label', 'AM/PM');
        });

        test('segments have aria-valuemin and aria-valuemax', () => {
            hourSeg().should('have.attr', 'aria-valuemin', '1');
            hourSeg().should('have.attr', 'aria-valuemax', '12');
            minuteSeg().should('have.attr', 'aria-valuemin', '0');
            minuteSeg().should('have.attr', 'aria-valuemax', '59');
            periodSeg().should('have.attr', 'aria-valuemin', '0');
            periodSeg().should('have.attr', 'aria-valuemax', '1');
        });

        test('placeholder segments have aria-valuetext="empty"', () => {
            hourSeg().should('have.attr', 'aria-valuetext', 'empty');
            minuteSeg().should('have.attr', 'aria-valuetext', 'empty');
        });
    });

    // ── 2. Data Attributes ──────────────────────────────────

    describe('data attributes', () => {
        test('segments have data-type attribute', () => {
            hourSeg().should('have.attr', 'data-type', 'hour');
            minuteSeg().should('have.attr', 'data-type', 'minute');
            periodSeg().should('have.attr', 'data-type', 'day-period');
        });

        test('literal separators exist with data-type="literal"', () => {
            cy.get('[data-type="literal"]').should('exist');
        });

        test('placeholder segments have data-placeholder', () => {
            hourSeg().should('have.attr', 'data-placeholder');
            minuteSeg().should('have.attr', 'data-placeholder');
        });

        test('data-placeholder removed after entering value', () => {
            hourSeg().focus();
            cy.realPress('ArrowUp');
            hourSeg().should('not.have.attr', 'data-placeholder');
        });

        test('data-disabled present when disabled', () => {
            cy.findByLabelText('disabled').click();
            hourSeg().should('have.attr', 'data-disabled');
            minuteSeg().should('have.attr', 'data-disabled');
            periodSeg().should('have.attr', 'data-disabled');
        });

        test('data-readonly present when read-only', () => {
            cy.findByLabelText('read only').click();
            hourSeg().should('have.attr', 'data-readonly');
        });
    });

    // ── 3. Keyboard — Arrow Up/Down (Increment/Decrement) ───

    describe('increment/decrement', () => {
        test('ArrowUp on hour sets value from placeholder', () => {
            hourSeg().should('have.attr', 'data-placeholder');
            hourSeg().focus();
            cy.realPress('ArrowUp');
            hourSeg().should('not.have.attr', 'data-placeholder');
        });

        test('ArrowUp increments hour', () => {
            hourSeg().focus();
            cy.realPress('ArrowUp'); // 1 (from min)
            hourSeg().should('have.attr', 'aria-valuenow', '1');
            cy.realPress('ArrowUp'); // 2
            hourSeg().should('have.attr', 'aria-valuenow', '2');
        });

        test('ArrowDown decrements hour', () => {
            hourSeg().focus();
            cy.realPress('ArrowUp'); // 1
            cy.realPress('ArrowUp'); // 2
            cy.realPress('ArrowUp'); // 3
            cy.realPress('ArrowDown'); // 2
            hourSeg().should('have.attr', 'aria-valuenow', '2');
        });

        test('hour wraps 12→1 on ArrowUp (H12)', () => {
            hourSeg().focus();
            // Set to 12 via End key
            cy.realPress('End');
            hourSeg().should('have.attr', 'aria-valuenow', '12');
            cy.realPress('ArrowUp');
            hourSeg().should('have.attr', 'aria-valuenow', '1');
        });

        test('hour wraps 1→12 on ArrowDown (H12)', () => {
            hourSeg().focus();
            cy.realPress('Home');
            hourSeg().should('have.attr', 'aria-valuenow', '1');
            cy.realPress('ArrowDown');
            hourSeg().should('have.attr', 'aria-valuenow', '12');
        });

        test('minute wraps 59→0 on ArrowUp', () => {
            minuteSeg().focus();
            cy.realPress('End');
            minuteSeg().should('have.attr', 'aria-valuenow', '59');
            cy.realPress('ArrowUp');
            minuteSeg().should('have.attr', 'aria-valuenow', '0');
        });

        test('minute wraps 0→59 on ArrowDown', () => {
            minuteSeg().focus();
            cy.realPress('Home');
            minuteSeg().should('have.attr', 'aria-valuenow', '0');
            cy.realPress('ArrowDown');
            minuteSeg().should('have.attr', 'aria-valuenow', '59');
        });

        test('ArrowUp/Down toggles AM/PM', () => {
            periodSeg().focus();
            cy.realPress('ArrowUp');  // From placeholder → AM (min)
            periodSeg().should('have.attr', 'aria-valuetext', 'AM');
            cy.realPress('ArrowUp');  // AM → PM
            periodSeg().should('have.attr', 'aria-valuetext', 'PM');
            cy.realPress('ArrowDown'); // PM → AM
            periodSeg().should('have.attr', 'aria-valuetext', 'AM');
            cy.realPress('ArrowDown'); // AM → PM
            periodSeg().should('have.attr', 'aria-valuetext', 'PM');
        });
    });

    // ── 4. Keyboard — Arrow Left/Right (Segment Navigation) ─

    describe('segment navigation', () => {
        test('ArrowRight moves from hour to minute', () => {
            hourSeg().focus();
            cy.realPress('ArrowRight');
            minuteSeg().should('be.focused');
        });

        test('ArrowRight moves from minute to day-period', () => {
            minuteSeg().focus();
            cy.realPress('ArrowRight');
            periodSeg().should('be.focused');
        });

        test('ArrowLeft moves from day-period to minute', () => {
            periodSeg().focus();
            cy.realPress('ArrowLeft');
            minuteSeg().should('be.focused');
        });

        test('ArrowLeft moves from minute to hour', () => {
            minuteSeg().focus();
            cy.realPress('ArrowLeft');
            hourSeg().should('be.focused');
        });

        test('ArrowLeft on first segment stays on first', () => {
            hourSeg().focus();
            cy.realPress('ArrowLeft');
            hourSeg().should('be.focused');
        });

        test('ArrowRight on last segment stays on last', () => {
            periodSeg().focus();
            cy.realPress('ArrowRight');
            periodSeg().should('be.focused');
        });

        test('Tab moves between segments', () => {
            hourSeg().focus();
            cy.realPress('Tab');
            minuteSeg().should('be.focused');
            cy.realPress('Tab');
            periodSeg().should('be.focused');
        });

        test('Shift+Tab moves backward between segments', () => {
            periodSeg().focus();
            cy.realPress(['Shift', 'Tab']);
            minuteSeg().should('be.focused');
            cy.realPress(['Shift', 'Tab']);
            hourSeg().should('be.focused');
        });
    });

    // ── 5. Keyboard — Digit Entry ───────────────────────────

    describe('digit entry', () => {
        test('typing "5" on hour sets 5 and advances to minute', () => {
            hourSeg().focus();
            cy.realPress('5');
            hourSeg().should('have.attr', 'aria-valuenow', '5');
            minuteSeg().should('be.focused');
        });

        test('typing "1" then "2" on hour sets 12 and advances', () => {
            hourSeg().focus();
            cy.realPress('1');
            // Still on hour — waiting for second digit
            hourSeg().should('be.focused');
            hourSeg().should('have.attr', 'aria-valuenow', '1');
            cy.realPress('2');
            hourSeg().should('have.attr', 'aria-valuenow', '12');
            minuteSeg().should('be.focused');
        });

        test('typing "1" then "0" on hour sets 10 and advances', () => {
            hourSeg().focus();
            cy.realPress('1');
            cy.realPress('0');
            hourSeg().should('have.attr', 'aria-valuenow', '10');
            minuteSeg().should('be.focused');
        });

        test('typing "3" then "0" on minute sets 30 and advances', () => {
            minuteSeg().focus();
            cy.realPress('3');
            minuteSeg().should('be.focused');
            cy.realPress('0');
            minuteSeg().should('have.attr', 'aria-valuenow', '30');
            periodSeg().should('be.focused');
        });

        test('typing "6" on minute sets 6 and advances immediately', () => {
            minuteSeg().focus();
            cy.realPress('6');
            minuteSeg().should('have.attr', 'aria-valuenow', '6');
            periodSeg().should('be.focused');
        });

        test('typing "0" then "5" on minute sets 5 and advances', () => {
            minuteSeg().focus();
            cy.realPress('0');
            minuteSeg().should('be.focused');
            cy.realPress('5');
            minuteSeg().should('have.attr', 'aria-valuenow', '5');
            periodSeg().should('be.focused');
        });

        test('A key sets AM on day-period', () => {
            periodSeg().focus();
            cy.realPress('p'); // set to PM first
            periodSeg().should('have.attr', 'aria-valuetext', 'PM');
            cy.realPress('a');
            periodSeg().should('have.attr', 'aria-valuetext', 'AM');
        });

        test('P key sets PM on day-period', () => {
            periodSeg().focus();
            cy.realPress('p');
            periodSeg().should('have.attr', 'aria-valuetext', 'PM');
        });
    });

    // ── 6. Keyboard — Backspace, Home, End ──────────────────

    describe('clear and bounds', () => {
        test('Backspace clears segment to placeholder', () => {
            hourSeg().focus();
            cy.realPress('ArrowUp');
            hourSeg().should('not.have.attr', 'data-placeholder');
            cy.realPress('Backspace');
            hourSeg().should('have.attr', 'data-placeholder');
            hourSeg().should('have.attr', 'aria-valuetext', 'empty');
        });

        test('Delete clears segment to placeholder', () => {
            minuteSeg().focus();
            cy.realPress('ArrowUp');
            minuteSeg().should('not.have.attr', 'data-placeholder');
            cy.realPress('Delete');
            minuteSeg().should('have.attr', 'data-placeholder');
        });

        test('Home sets hour to minimum (1 for H12)', () => {
            hourSeg().focus();
            cy.realPress('Home');
            hourSeg().should('have.attr', 'aria-valuenow', '1');
        });

        test('End sets hour to maximum (12 for H12)', () => {
            hourSeg().focus();
            cy.realPress('End');
            hourSeg().should('have.attr', 'aria-valuenow', '12');
        });

        test('Home sets minute to 0', () => {
            minuteSeg().focus();
            cy.realPress('Home');
            minuteSeg().should('have.attr', 'aria-valuenow', '0');
        });

        test('End sets minute to 59', () => {
            minuteSeg().focus();
            cy.realPress('End');
            minuteSeg().should('have.attr', 'aria-valuenow', '59');
        });
    });

    // ── 7. Value Callback ───────────────────────────────────

    describe('value callback', () => {
        test('readout shows "none" initially', () => {
            readout().should('have.text', 'none');
        });

        test('filling all segments fires on_value_change', () => {
            hourSeg().focus();
            cy.realPress('9');   // hour = 9, advance
            cy.realPress('3');
            cy.realPress('0');   // minute = 30, advance
            cy.realPress('a');   // AM
            readout().should('have.text', '09:30:00');
        });

        test('partially filled segments do not fire callback', () => {
            hourSeg().focus();
            cy.realPress('5');   // hour = 5, advance to minute
            // minute not filled, period not filled
            readout().should('have.text', 'none');
        });

        test('updating one segment of a complete time updates readout', () => {
            // Fill all segments first
            hourSeg().focus();
            cy.realPress('9');
            cy.realPress('3');
            cy.realPress('0');
            cy.realPress('a');
            readout().should('have.text', '09:30:00');

            // Change hour
            hourSeg().focus();
            cy.realPress('2');
            readout().should('have.text', '02:30:00');
        });
    });

    // ── 8. Disabled Variant ─────────────────────────────────

    describe('disabled', () => {
        beforeEach(() => {
            cy.findByLabelText('disabled').click();
        });

        test('segments have tabindex="-1" when disabled', () => {
            hourSeg().should('have.attr', 'tabindex', '-1');
            minuteSeg().should('have.attr', 'tabindex', '-1');
            periodSeg().should('have.attr', 'tabindex', '-1');
        });

        test('data-disabled is present on all segments', () => {
            hourSeg().should('have.attr', 'data-disabled');
            minuteSeg().should('have.attr', 'data-disabled');
            periodSeg().should('have.attr', 'data-disabled');
        });
    });

    // ── 9. Read-Only Variant ────────────────────────────────

    describe('read-only', () => {
        test('ArrowUp does not change value when read-only', () => {
            // Set a value first
            hourSeg().focus();
            cy.realPress('5');
            // Enable read-only
            cy.findByLabelText('read only').click();
            hourSeg().focus();
            cy.realPress('ArrowUp');
            hourSeg().should('have.attr', 'aria-valuenow', '5');
        });
    });

    // ── 10. H24 Variant ─────────────────────────────────────

    describe('24-hour format', () => {
        beforeEach(() => {
            cy.findByLabelText('24-hour').click();
        });

        test('no day-period segment in H24', () => {
            cy.get('[data-type="day-period"]').should('not.exist');
        });

        test('hour range is 0–23', () => {
            hourSeg().should('have.attr', 'aria-valuemin', '0');
            hourSeg().should('have.attr', 'aria-valuemax', '23');
        });

        test('hour wraps 23→0 on ArrowUp', () => {
            hourSeg().focus();
            cy.realPress('End'); // 23
            hourSeg().should('have.attr', 'aria-valuenow', '23');
            cy.realPress('ArrowUp');
            hourSeg().should('have.attr', 'aria-valuenow', '0');
        });

        test('hour wraps 0→23 on ArrowDown', () => {
            hourSeg().focus();
            cy.realPress('Home'); // 0
            hourSeg().should('have.attr', 'aria-valuenow', '0');
            cy.realPress('ArrowDown');
            hourSeg().should('have.attr', 'aria-valuenow', '23');
        });

        test('filling hour and minute fires callback with 24h time', () => {
            hourSeg().focus();
            cy.realPress('1');
            cy.realPress('4');   // 14
            cy.realPress('3');
            cy.realPress('0');   // 30
            readout().should('have.text', '14:30:00');
        });
    });

    // ── 11. Second Granularity ──────────────────────────────

    describe('second granularity', () => {
        beforeEach(() => {
            cy.findByLabelText('seconds').click();
        });

        test('second segment exists', () => {
            secondSeg().should('exist');
            secondSeg().should('have.attr', 'aria-label', 'second');
        });

        test('second segment has correct bounds', () => {
            secondSeg().should('have.attr', 'aria-valuemin', '0');
            secondSeg().should('have.attr', 'aria-valuemax', '59');
        });

        test('filling all segments including seconds fires callback', () => {
            hourSeg().focus();
            cy.realPress('9');   // hour = 9, advance
            cy.realPress('1');
            cy.realPress('5');   // minute = 15, advance
            cy.realPress('4');
            cy.realPress('5');   // second = 45, advance
            cy.realPress('p');   // PM
            readout().should('have.text', '21:15:45');
        });
    });

    // ── 12. Form Integration ────────────────────────────────

    describe('form integration', () => {
        test('hidden input exists with correct name', () => {
            cy.get('input[type="hidden"][name="meeting_time"]').should('exist');
        });

        test('hidden input value is empty when no time selected', () => {
            cy.get('input[type="hidden"][name="meeting_time"]').should('have.value', '');
        });

        test('hidden input value is ISO format when time is complete', () => {
            hourSeg().focus();
            cy.realPress('9');
            cy.realPress('3');
            cy.realPress('0');
            cy.realPress('a');
            cy.get('input[type="hidden"][name="meeting_time"]').should('have.value', '09:30:00');
        });
    });

    // ── Axe Accessibility Audit ─────────────────────────────

    describe('axe audit', () => {
        test('no violations in default state', () => {
            cy.checkComponentA11y();
        });
    });
});
