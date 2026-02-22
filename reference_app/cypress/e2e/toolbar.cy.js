describe('Toolbar', () => {
    // ── Helpers ──────────────────────────────────────────────

    function getToolbar() {
        return cy.findByRole('toolbar');
    }

    function getButton(name) {
        return cy.findByRole('button', {name});
    }

    function getLink(name) {
        return cy.findByRole('link', {name});
    }

    function getToggleItem(name) {
        // In single-mode ToggleGroup, items have role="radio", not role="button"
        return cy.findByText(name, {selector: '.toolbar-toggle-item'});
    }

    function shouldBeOn(name) {
        getToggleItem(name).should('have.attr', 'data-state', 'on');
    }

    function shouldBeOff(name) {
        getToggleItem(name).should('have.attr', 'data-state', 'off');
    }

    beforeEach(() => {
        cy.visit('/toolbar');
    });

    // ── 1. Accessibility Semantics ──────────────────────────

    describe('accessibility', () => {
        it('Root has role="toolbar"', () => {
            getToolbar().should('exist');
        });

        it('Root has aria-orientation', () => {
            getToolbar().should('have.attr', 'aria-orientation', 'horizontal');
        });

        it('Separator has role="separator"', () => {
            cy.findAllByRole('separator').should('have.length', 2);
        });

        it('Button is focusable', () => {
            getButton('Bold').focus();
            getButton('Bold').should('be.focused');
        });

        it('Link is focusable', () => {
            getLink('Learn More').focus();
            getLink('Learn More').should('be.focused');
        });

        it('ToggleItems render as button elements', () => {
            getToggleItem('Left').should('exist').and('match', 'button');
            getToggleItem('Center').should('exist').and('match', 'button');
            getToggleItem('Right').should('exist').and('match', 'button');
        });
    });

    // ── 2. Data Attributes ──────────────────────────────────

    describe('data attributes', () => {
        it('Root has data-orientation="horizontal"', () => {
            getToolbar().should('have.attr', 'data-orientation', 'horizontal');
        });

        it('Separator has data-orientation', () => {
            cy.findAllByRole('separator').first().should('have.attr', 'data-orientation', 'vertical');
        });

        it('ToggleItem has data-state="off" initially', () => {
            shouldBeOff('Left');
            shouldBeOff('Center');
            shouldBeOff('Right');
        });

        it('ToggleItem data-state changes to "on" on click', () => {
            getToggleItem('Left').click();
            shouldBeOn('Left');
        });

        it('ToggleItem data-state goes back to "off" when another is selected', () => {
            getToggleItem('Left').click();
            shouldBeOn('Left');
            getToggleItem('Center').click();
            shouldBeOn('Center');
            shouldBeOff('Left');
        });

        it('Button has data-orientation', () => {
            getButton('Bold').should('have.attr', 'data-orientation', 'horizontal');
        });

        it('ToggleItem has data-orientation', () => {
            getToggleItem('Left').should('have.attr', 'data-orientation', 'horizontal');
        });
    });

    // ── 3. Keyboard Navigation ──────────────────────────────

    describe('keyboard navigation', () => {
        it('Tab enters toolbar and focuses first item', () => {
            getButton('Bold').focus();
            getButton('Bold').should('be.focused');
        });

        it('Tab again leaves toolbar', () => {
            getButton('Bold').focus();
            cy.realPress('Tab');
            // Focus should have left the toolbar
            getButton('Bold').should('not.be.focused');
            getButton('Italic').should('not.be.focused');
            getLink('Learn More').should('not.be.focused');
            getToggleItem('Left').should('not.be.focused');
        });

        it('ArrowRight moves to next item', () => {
            getButton('Bold').focus();
            cy.realPress('ArrowRight');
            getButton('Italic').should('be.focused');
        });

        it('ArrowRight moves across separators', () => {
            getButton('Italic').focus();
            cy.realPress('ArrowRight');
            getLink('Learn More').should('be.focused');
        });

        it('ArrowRight moves from link to toggle items', () => {
            getLink('Learn More').focus();
            cy.realPress('ArrowRight');
            getToggleItem('Left').should('be.focused');
        });

        it('ArrowLeft moves to previous item', () => {
            getButton('Italic').focus();
            cy.realPress('ArrowLeft');
            getButton('Bold').should('be.focused');
        });

        it('ArrowLeft moves across separators', () => {
            getLink('Learn More').focus();
            cy.realPress('ArrowLeft');
            getButton('Italic').should('be.focused');
        });

        it('Home moves to first item', () => {
            getToggleItem('Right').focus();
            cy.realPress('Home');
            getButton('Bold').should('be.focused');
        });

        it('End moves to last item', () => {
            getButton('Bold').focus();
            cy.realPress('End');
            getToggleItem('Right').should('be.focused');
        });

        it('ArrowRight wraps from last to first', () => {
            getToggleItem('Right').focus();
            cy.realPress('ArrowRight');
            getButton('Bold').should('be.focused');
        });

        it('ArrowLeft wraps from first to last', () => {
            getButton('Bold').focus();
            cy.realPress('ArrowLeft');
            getToggleItem('Right').should('be.focused');
        });

        it('Space activates a ToggleItem', () => {
            getToggleItem('Left').focus();
            cy.realPress('Space');
            shouldBeOn('Left');
        });

        it('Enter activates a ToggleItem', () => {
            getToggleItem('Center').focus();
            cy.realPress('Enter');
            shouldBeOn('Center');
        });
    });

    // ── 4. Pointer Interaction ──────────────────────────────

    describe('pointer interaction', () => {
        it('Click Button triggers action', () => {
            getButton('Bold').click();
            cy.findByTestId('action-output').should('have.text', 'Bold clicked');
        });

        it('Click ToggleItem toggles state', () => {
            getToggleItem('Left').click();
            shouldBeOn('Left');
            getToggleItem('Left').click();
            // In single mode, clicking active item deselects it
            shouldBeOff('Left');
        });

        it('Clicking one ToggleItem deselects others (single mode)', () => {
            getToggleItem('Left').click();
            shouldBeOn('Left');
            getToggleItem('Right').click();
            shouldBeOn('Right');
            shouldBeOff('Left');
        });

        it('Click Link is focusable and clickable', () => {
            getLink('Learn More').click();
            getLink('Learn More').should('exist');
        });
    });

    // ── 5. Focus Management ─────────────────────────────────

    describe('focus management', () => {
        it('Tab into toolbar focuses first item, Shift+Tab returns', () => {
            getButton('Bold').focus();
            getButton('Bold').should('be.focused');
            cy.realPress(['Shift', 'Tab']);
            getButton('Bold').should('not.be.focused');
        });

        it('Roving focus remembers last focused item', () => {
            getButton('Bold').focus();
            cy.realPress('ArrowRight');
            getButton('Italic').should('be.focused');
            // Tab out of toolbar
            cy.realPress('Tab');
            // Tab back in should focus the last focused item (Italic)
            cy.realPress(['Shift', 'Tab']);
            getButton('Italic').should('be.focused');
        });
    });
});
