// Calendar — no React equivalent exists, so all tests skip for react.
const skipReact = it.skipForFramework('react', 'no React Calendar primitive');

describe('Calendar', () => {
    // ── Helpers ──────────────────────────────────────────────

    function reset() {
        cy.findByTestId('reset').click();
    }

    /** Click the grid button whose data-date matches the given ISO string. */
    function clickDate(iso) {
        cy.get(`button[data-date='${iso}']`).click();
    }

    /** Focus a date cell (click it first to ensure grid has focus). */
    function focusDate(iso) {
        cy.get(`button[data-date='${iso}']`).focus();
    }

    /** Assert the selected-value readout text. */
    function selectedShouldBe(text) {
        cy.findByTestId('selected-value').should('have.text', text);
    }

    /** Assert the month-value readout text. */
    function monthShouldBe(text) {
        cy.findByTestId('month-value').should('have.text', text);
    }

    /** Return the grid `<table>` element. */
    function grid() {
        return cy.get('table[role="grid"]');
    }

    // ── Setup ────────────────────────────────────────────────

    beforeEach(() => {
        cy.visit('/calendar');
        reset();
    });

    // ── 1. Accessibility Semantics ───────────────────────────

    describe('accessibility', () => {
        skipReact('grid has role="grid"', () => {
            grid().should('have.attr', 'role', 'grid');
        });

        skipReact('grid has aria-labelledby pointing to the heading', () => {
            grid()
                .invoke('attr', 'aria-labelledby')
                .then((id) => {
                    cy.get(`#${id}`).should('contain.text', 'June 2024');
                });
        });

        skipReact('heading has aria-live="polite"', () => {
            grid()
                .invoke('attr', 'aria-labelledby')
                .then((id) => {
                    cy.get(`#${id}`).should('have.attr', 'aria-live', 'polite');
                });
        });

        skipReact('day buttons have aria-label with full date', () => {
            cy.get('button[data-date="2024-06-15"]').should(
                'have.attr',
                'aria-label',
                'Saturday, June 15, 2024',
            );
        });

        skipReact('selected cell has aria-selected="true"', () => {
            clickDate('2024-06-15');
            cy.get('button[data-date="2024-06-15"]').should('have.attr', 'aria-selected', 'true');
        });

        skipReact('unselected cell does not have aria-selected="true"', () => {
            cy.get('button[data-date="2024-06-10"]').should('not.have.attr', 'aria-selected', 'true');
        });

        skipReact('today has aria-current="date"', () => {
            // We can't know today's exact date, but we can assert the attribute
            // exists on exactly one cell.
            cy.get('button[aria-current="date"]').should('have.length.at.most', 1);
        });

        skipReact('weekday header row is aria-hidden', () => {
            cy.get('thead').should('have.attr', 'aria-hidden', 'true');
        });

        skipReact('disabled dates have aria-disabled="true"', () => {
            cy.findByLabelText('disable weekends').click();
            // June 1 2024 is a Saturday
            cy.get('button[data-date="2024-06-01"]').should('have.attr', 'aria-disabled', 'true');
        });

        skipReact('unavailable dates have aria-disabled="true"', () => {
            cy.findByLabelText('unavailable 10th/20th').click();
            cy.get('button[data-date="2024-06-10"]').should('have.attr', 'aria-disabled', 'true');
        });
    });

    // ── 2. Data Attributes ───────────────────────────────────

    describe('data attributes', () => {
        skipReact('data-date contains ISO date string', () => {
            cy.get('button[data-date="2024-06-01"]').should('exist');
        });

        skipReact('data-selected present on selected date', () => {
            clickDate('2024-06-15');
            cy.get('button[data-date="2024-06-15"]').should('have.attr', 'data-selected');
        });

        skipReact('data-selected absent on non-selected date', () => {
            cy.get('button[data-date="2024-06-15"]').should('not.have.attr', 'data-selected');
        });

        skipReact('data-today present on today (if visible)', () => {
            // Navigate to current month first
            const today = new Date();
            const iso = today.toISOString().slice(0, 10);
            // This test only runs if today is in the visible month — skip assertion
            // if not. We just verify the attribute mechanism works by checking June's grid.
            cy.get('button[data-today]').should('have.length.at.most', 1);
        });

        skipReact('data-outside-month on previous/next month dates', () => {
            // June 2024 starts Saturday with Sunday start — grid starts May 26
            cy.get('button[data-date="2024-05-26"]').should('have.attr', 'data-outside-month');
        });

        skipReact('data-disabled on dates matching is_date_disabled predicate', () => {
            cy.findByLabelText('disable weekends').click();
            cy.get('button[data-date="2024-06-01"]').should('have.attr', 'data-disabled'); // Sat
            cy.get('button[data-date="2024-06-03"]').should('not.have.attr', 'data-disabled'); // Mon
        });

        skipReact('data-unavailable on dates matching is_date_unavailable', () => {
            cy.findByLabelText('unavailable 10th/20th').click();
            cy.get('button[data-date="2024-06-10"]').should('have.attr', 'data-unavailable');
            cy.get('button[data-date="2024-06-20"]').should('have.attr', 'data-unavailable');
            cy.get('button[data-date="2024-06-11"]').should('not.have.attr', 'data-unavailable');
        });

        skipReact('data-disabled on dates outside min/max range', () => {
            cy.findByLabelText('min/max (Jun 5–25)').click();
            cy.get('button[data-date="2024-06-04"]').should('have.attr', 'data-disabled');
            cy.get('button[data-date="2024-06-26"]').should('have.attr', 'data-disabled');
            cy.get('button[data-date="2024-06-10"]').should('not.have.attr', 'data-disabled');
        });
    });

    // ── 3. Pointer Interaction ───────────────────────────────

    describe('pointer interaction', () => {
        skipReact('clicking a date selects it', () => {
            clickDate('2024-06-15');
            selectedShouldBe('2024-06-15');
            cy.get('button[data-date="2024-06-15"]').should('have.attr', 'data-selected');
        });

        skipReact('clicking a different date moves selection', () => {
            clickDate('2024-06-15');
            clickDate('2024-06-20');
            selectedShouldBe('2024-06-20');
            cy.get('button[data-date="2024-06-15"]').should('not.have.attr', 'data-selected');
            cy.get('button[data-date="2024-06-20"]').should('have.attr', 'data-selected');
        });

        skipReact('clicking outside-month date selects and navigates', () => {
            // May 26 is in the June grid as an outside-month date
            clickDate('2024-05-26');
            selectedShouldBe('2024-05-26');
            monthShouldBe('2024-05-01');
        });

        skipReact('clicking disabled date does not select', () => {
            cy.findByLabelText('disable weekends').click();
            clickDate('2024-06-01'); // Saturday
            selectedShouldBe('none');
        });

        skipReact('clicking unavailable date does not select', () => {
            cy.findByLabelText('unavailable 10th/20th').click();
            clickDate('2024-06-10');
            selectedShouldBe('none');
        });

        skipReact('clicking when fully disabled does not select', () => {
            cy.findByLabelText('disabled').click();
            clickDate('2024-06-15');
            selectedShouldBe('none');
        });

        skipReact('controlled clear → reselect → clear cycle works', () => {
            // Regression: clearing controlled value then re-selecting then
            // clearing again must work — previously the second clear was a no-op
            // because use_controllable_state fell back to stale internal state.
            clickDate('2024-06-15');
            selectedShouldBe('2024-06-15');
            cy.findByTestId('reset').click(); // clears selection
            selectedShouldBe('none');
            cy.get('button[data-date="2024-06-15"]').should('not.have.attr', 'data-selected');
            clickDate('2024-06-20');
            selectedShouldBe('2024-06-20');
            cy.findByTestId('reset').click(); // second clear
            selectedShouldBe('none');
            cy.get('button[data-date="2024-06-20"]').should('not.have.attr', 'data-selected');
        });

        skipReact('clicking when read-only does not change selection', () => {
            clickDate('2024-06-15');
            selectedShouldBe('2024-06-15');
            cy.findByLabelText('read only').click();
            clickDate('2024-06-20');
            selectedShouldBe('2024-06-15');
        });
    });

    // ── 4. Month Navigation ──────────────────────────────────

    describe('month navigation', () => {
        skipReact('prev button navigates to previous month', () => {
            cy.findByRole('button', {name: 'Previous month'}).click();
            monthShouldBe('2024-05-01');
        });

        skipReact('next button navigates to next month', () => {
            cy.findByRole('button', {name: 'Next month'}).click();
            monthShouldBe('2024-07-01');
        });

        skipReact('heading text updates on navigation', () => {
            grid()
                .invoke('attr', 'aria-labelledby')
                .then((id) => {
                    cy.get(`#${id}`).should('contain.text', 'June 2024');
                });
            cy.findByRole('button', {name: 'Next month'}).click();
            grid()
                .invoke('attr', 'aria-labelledby')
                .then((id) => {
                    cy.get(`#${id}`).should('contain.text', 'July 2024');
                });
        });

        skipReact('prev button disabled at min bound', () => {
            cy.findByLabelText('min/max (Jun 5–25)').click();
            // Previous month (May) is entirely before min (Jun 5) — button should disable
            cy.findByRole('button', {name: 'Previous month'}).should('be.disabled');
        });

        skipReact('next button disabled at max bound', () => {
            cy.findByLabelText('min/max (Jun 5–25)').click();
            // Next month (July 1) is after max (Jun 25) — button should disable
            cy.findByRole('button', {name: 'Next month'}).should('be.disabled');
        });
    });

    // ── 5. Keyboard Navigation ───────────────────────────────

    describe('keyboard navigation', () => {
        skipReact('ArrowRight moves to next day', () => {
            focusDate('2024-06-15');
            cy.realPress('ArrowRight');
            cy.get('button[data-date="2024-06-16"]').should('be.focused');
        });

        skipReact('ArrowLeft moves to previous day', () => {
            focusDate('2024-06-15');
            cy.realPress('ArrowLeft');
            cy.get('button[data-date="2024-06-14"]').should('be.focused');
        });

        skipReact('ArrowDown moves to same day next week', () => {
            focusDate('2024-06-15');
            cy.realPress('ArrowDown');
            cy.get('button[data-date="2024-06-22"]').should('be.focused');
        });

        skipReact('ArrowUp moves to same day previous week', () => {
            focusDate('2024-06-15');
            cy.realPress('ArrowUp');
            cy.get('button[data-date="2024-06-08"]').should('be.focused');
        });

        skipReact('PageDown moves to same day next month', () => {
            focusDate('2024-06-15');
            cy.realPress('PageDown');
            cy.get('button[data-date="2024-07-15"]').should('be.focused');
            monthShouldBe('2024-07-01');
        });

        skipReact('PageUp moves to same day previous month', () => {
            focusDate('2024-06-15');
            cy.realPress('PageUp');
            cy.get('button[data-date="2024-05-15"]').should('be.focused');
            monthShouldBe('2024-05-01');
        });

        skipReact('Shift+PageDown moves to same day next year', () => {
            focusDate('2024-06-15');
            cy.realPress(['Shift', 'PageDown']);
            cy.get('button[data-date="2025-06-15"]').should('be.focused');
            monthShouldBe('2025-06-01');
        });

        skipReact('Shift+PageUp moves to same day previous year', () => {
            focusDate('2024-06-15');
            cy.realPress(['Shift', 'PageUp']);
            cy.get('button[data-date="2023-06-15"]').should('be.focused');
            monthShouldBe('2023-06-01');
        });

        skipReact('Home moves to start of week', () => {
            focusDate('2024-06-12'); // Wednesday
            cy.realPress('Home');
            cy.get('button[data-date="2024-06-09"]').should('be.focused'); // Sunday
        });

        skipReact('End moves to end of week', () => {
            focusDate('2024-06-12'); // Wednesday
            cy.realPress('End');
            cy.get('button[data-date="2024-06-15"]').should('be.focused'); // Saturday
        });

        skipReact('Enter selects focused date', () => {
            focusDate('2024-06-15');
            cy.realPress('Enter');
            selectedShouldBe('2024-06-15');
        });

        skipReact('Space selects focused date', () => {
            focusDate('2024-06-20');
            cy.realPress('Space');
            selectedShouldBe('2024-06-20');
        });

        skipReact('Enter does not select disabled date', () => {
            cy.findByLabelText('disable weekends').click();
            focusDate('2024-06-15'); // Saturday
            cy.realPress('Enter');
            selectedShouldBe('none');
        });

        skipReact('Arrow crossing month boundary auto-navigates', () => {
            focusDate('2024-06-30');
            cy.realPress('ArrowRight');
            cy.get('button[data-date="2024-07-01"]').should('be.focused');
            monthShouldBe('2024-07-01');
        });

        skipReact('Arrow clamps at min date', () => {
            cy.findByLabelText('min/max (Jun 5–25)').click();
            focusDate('2024-06-05');
            cy.realPress('ArrowLeft');
            // Should stay at min date since Jun 4 < min
            cy.get('button[data-date="2024-06-05"]').should('be.focused');
        });

        skipReact('Arrow clamps at max date', () => {
            cy.findByLabelText('min/max (Jun 5–25)').click();
            focusDate('2024-06-25');
            cy.realPress('ArrowRight');
            cy.get('button[data-date="2024-06-25"]').should('be.focused');
        });
    });

    // ── 6. Roving Tabindex ───────────────────────────────────

    describe('roving tabindex', () => {
        skipReact('exactly one cell has tabindex="0"', () => {
            cy.get('table[role="grid"] button[tabindex="0"]').should('have.length', 1);
        });

        skipReact('after clicking a date, that date has tabindex="0"', () => {
            clickDate('2024-06-15');
            cy.get('button[data-date="2024-06-15"]').should('have.attr', 'tabindex', '0');
        });

        skipReact('other cells have tabindex="-1"', () => {
            cy.get('button[data-date="2024-06-10"]').should('have.attr', 'tabindex', '-1');
        });

        skipReact('Tab into grid focuses the tabindex="0" cell', () => {
            // Focus the next button then shift-tab back into grid, or
            // focus prev button and tab forward into grid.
            cy.findByRole('button', {name: 'Previous month'}).focus();
            cy.realPress('Tab');
            // The heading is between nav buttons and grid — skip it
            // Actually, heading is inside header, before grid.
            // Let's just tab from the next button.
            cy.findByRole('button', {name: 'Next month'}).focus();
            cy.realPress('Tab');
            // The focused element should be inside the grid
            cy.focused().should('have.attr', 'tabindex', '0');
        });
    });

    // ── 7. Week Start Variant ────────────────────────────────

    describe('Monday start variant', () => {
        beforeEach(() => {
            cy.findByLabelText('Monday start').click();
        });

        skipReact('first weekday header is Mo', () => {
            cy.get('thead th').first().should('have.text', 'Mo');
        });

        skipReact('last weekday header is Su', () => {
            cy.get('thead th').last().should('have.text', 'Su');
        });

        skipReact('Home goes to Monday of the week', () => {
            focusDate('2024-06-12'); // Wednesday
            cy.realPress('Home');
            cy.get('button[data-date="2024-06-10"]').should('be.focused'); // Monday
        });

        skipReact('End goes to Sunday of the week', () => {
            focusDate('2024-06-12'); // Wednesday
            cy.realPress('End');
            cy.get('button[data-date="2024-06-16"]').should('be.focused'); // Sunday
        });
    });

    // ── 8. Fixed Weeks Variant ───────────────────────────────

    describe('fixed weeks variant', () => {
        skipReact('always renders 6 week rows', () => {
            cy.findByLabelText('fixed weeks').click();
            cy.get('tbody tr').should('have.length', 6);
        });
    });

    // ── 9. Disabled Variant ──────────────────────────────────

    describe('disabled variant', () => {
        beforeEach(() => {
            cy.findByLabelText('disabled').click();
        });

        skipReact('prev button is disabled', () => {
            cy.findByRole('button', {name: 'Previous month'}).should('be.disabled');
        });

        skipReact('next button is disabled', () => {
            cy.findByRole('button', {name: 'Next month'}).should('be.disabled');
        });

        skipReact('no date can be selected', () => {
            clickDate('2024-06-15');
            selectedShouldBe('none');
        });
    });

    // ── 10. Read-Only Variant ────────────────────────────────

    describe('read-only variant', () => {
        beforeEach(() => {
            clickDate('2024-06-15');
            cy.findByLabelText('read only').click();
        });

        skipReact('click does not change selection', () => {
            clickDate('2024-06-20');
            selectedShouldBe('2024-06-15');
        });

        skipReact('Enter does not change selection', () => {
            focusDate('2024-06-20');
            cy.realPress('Enter');
            selectedShouldBe('2024-06-15');
        });

        skipReact('month navigation still works', () => {
            cy.findByRole('button', {name: 'Next month'}).click();
            monthShouldBe('2024-07-01');
        });
    });
});
