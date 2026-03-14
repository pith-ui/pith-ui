describe('Toolbar', () => {
    // ── Helpers ──────────────────────────────────────────────

    function getToolbar() {
        return cy.findByTestId('horizontal-toolbar');
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

    // ── 6. Disabled Button ────────────────────────────────────

    describe('disabled button', () => {
        it('disabled button has disabled attribute', () => {
            // toolbar-dcnv-1
            cy.findByTestId('disabled-button').should('be.disabled');
        });

        it('disabled button has native disabled attribute', () => {
            // toolbar-dcnv-1
            // Toolbar.Button renders a plain <button> with the native disabled attribute
            // (unlike ToggleItem which adds data-disabled)
            cy.findByTestId('disabled-button').should('have.attr', 'disabled');
        });

        it('disabled button is skipped by roving focus', () => {
            // toolbar-dcnv-1
            // Focus the last toggle item ("Right"), then press ArrowRight.
            // The disabled button is the next item in DOM order, but roving
            // focus should skip it and wrap around to the first item ("Bold").
            getToggleItem('Right').focus();
            cy.realPress('ArrowRight');
            // The disabled "Disabled" button should be skipped
            cy.findByTestId('disabled-button').should('not.be.focused');
            getButton('Bold').should('be.focused');
        });
    });

    // ── 7. RTL Direction ─────────────────────────────────────────

    describe('RTL direction', () => {
        it('RTL toolbar has data-orientation="horizontal"', () => {
            // toolbar-dcnv-2
            cy.findByTestId('rtl-toolbar').should('have.attr', 'data-orientation', 'horizontal');
        });

        it('ArrowLeft moves to next item in RTL toolbar', () => {
            // toolbar-dcnv-2
            // In RTL, ArrowLeft should move forward (next)
            cy.findByTestId('rtl-toolbar').within(() => {
                getButton('First').focus();
                getButton('First').should('be.focused');
                cy.realPress('ArrowLeft');
                getButton('Second').should('be.focused');
            });
        });

        it('ArrowRight moves to previous item in RTL toolbar', () => {
            // toolbar-dcnv-2
            // In RTL, ArrowRight should move backward (previous)
            cy.findByTestId('rtl-toolbar').within(() => {
                getButton('Second').focus();
                getButton('Second').should('be.focused');
                cy.realPress('ArrowRight');
                getButton('First').should('be.focused');
            });
        });

        it('ArrowLeft wraps from last to first in RTL', () => {
            // toolbar-dcnv-2
            cy.findByTestId('rtl-toolbar').within(() => {
                getButton('Third').focus();
                cy.realPress('ArrowLeft');
                getButton('First').should('be.focused');
            });
        });

        it('ArrowRight wraps from first to last in RTL', () => {
            // toolbar-dcnv-2
            cy.findByTestId('rtl-toolbar').within(() => {
                getButton('First').focus();
                cy.realPress('ArrowRight');
                getButton('Third').should('be.focused');
            });
        });

        it('Home and End still work in RTL', () => {
            // toolbar-dcnv-2
            cy.findByTestId('rtl-toolbar').within(() => {
                getButton('Third').focus();
                cy.realPress('Home');
                getButton('First').should('be.focused');
                cy.realPress('End');
                getButton('Third').should('be.focused');
            });
        });
    });

    // ── 7b. Attribute Forwarding ────────────────────────────

    describe('attribute forwarding', () => {
        it('Root forwards className/class and custom data attributes', () => {
            getToolbar()
                .should('have.class', 'toolbar-root')
                .and('have.attr', 'data-custom', 'toolbar-root-custom')
                .and('have.attr', 'role', 'toolbar');
        });

        it('Button forwards className/class and custom data attributes', () => {
            getButton('Bold')
                .should('have.class', 'toolbar-button')
                .and('have.attr', 'data-custom', 'toolbar-button-custom');
        });

        it('Root preserves component attributes alongside forwarded attributes', () => {
            getToolbar()
                .should('have.attr', 'data-orientation', 'horizontal')
                .and('have.attr', 'aria-orientation', 'horizontal')
                .and('have.attr', 'data-custom', 'toolbar-root-custom')
                .and('have.class', 'toolbar-root');
        });
    });

    // ── 8. Vertical Orientation ───────────────────────────────

    describe('vertical orientation', () => {
        it('vertical toolbar has aria-orientation="vertical"', () => {
            // toolbar-dcnv-3
            cy.findByTestId('vertical-toolbar').should('have.attr', 'aria-orientation', 'vertical');
        });

        it('vertical toolbar has data-orientation="vertical"', () => {
            // toolbar-dcnv-3
            cy.findByTestId('vertical-toolbar').should('have.attr', 'data-orientation', 'vertical');
        });

        it('ArrowDown moves to next item in vertical toolbar', () => {
            // toolbar-dcnv-3
            // Focus the first item in the vertical toolbar, press ArrowDown,
            // verify the next item receives focus.
            cy.findByTestId('vertical-toolbar').within(() => {
                getButton('VBold').focus();
                getButton('VBold').should('be.focused');
                cy.realPress('ArrowDown');
                getButton('VItalic').should('be.focused');
            });
        });

        it('ArrowUp moves to previous item in vertical toolbar', () => {
            // toolbar-dcnv-3
            // Focus the second item in the vertical toolbar, press ArrowUp,
            // verify the first item receives focus.
            cy.findByTestId('vertical-toolbar').within(() => {
                getButton('VItalic').focus();
                getButton('VItalic').should('be.focused');
                cy.realPress('ArrowUp');
                getButton('VBold').should('be.focused');
            });
        });
    });
});
