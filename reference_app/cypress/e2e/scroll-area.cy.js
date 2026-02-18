describe('Scroll Area', () => {
    // ── Helpers ──────────────────────────────────────────────

    function getRoot() {
        return cy.findByTestId('scroll-area-root');
    }

    function getViewport() {
        return cy.findByTestId('scroll-area-viewport');
    }

    function getVerticalScrollbar() {
        return cy.findByTestId('scrollbar-vertical');
    }

    function getVerticalThumb() {
        return cy.findByTestId('thumb-vertical');
    }

    function getHorizontalScrollbar() {
        return cy.findByTestId('scrollbar-horizontal');
    }

    function getHorizontalThumb() {
        return cy.findByTestId('thumb-horizontal');
    }

    beforeEach(() => {
        cy.visit('/scroll-area');
    });

    // ── 1. Structure ────────────────────────────────────────

    describe('structure', () => {
        it('root element exists', () => {
            getRoot().should('exist');
        });

        it('viewport element exists', () => {
            getViewport().should('exist');
        });

        it('content items are rendered inside viewport', () => {
            getViewport().should('contain.text', 'Item 1');
            getViewport().should('contain.text', 'Item 50');
        });

        it('viewport contains scrollable content', () => {
            // The internal scrollable element (with data-radix-scroll-area-viewport)
            // should have scrollHeight > clientHeight when content overflows
            cy.get('[data-radix-scroll-area-viewport]').then(($el) => {
                expect($el[0].scrollHeight).to.be.greaterThan($el[0].clientHeight);
            });
        });
    });

    // ── 2. Data Attributes ──────────────────────────────────

    describe('data attributes', () => {
        describe('always type', () => {
            beforeEach(() => {
                cy.findByLabelText('always').click();
            });

            it('vertical scrollbar has data-orientation="vertical"', () => {
                getVerticalScrollbar().should('have.attr', 'data-orientation', 'vertical');
            });

            it('vertical scrollbar has data-state attribute', () => {
                getVerticalScrollbar().should('have.attr', 'data-state');
            });

            it('vertical scrollbar data-state is "visible" when content overflows', () => {
                getVerticalScrollbar().should('have.attr', 'data-state', 'visible');
            });

            it('vertical thumb has data-state attribute', () => {
                getVerticalThumb().should('have.attr', 'data-state');
            });

            it('vertical thumb data-state is "visible" when content overflows', () => {
                getVerticalThumb().should('have.attr', 'data-state', 'visible');
            });
        });

        describe('horizontal scrollbar', () => {
            beforeEach(() => {
                cy.findByLabelText('always').click();
                cy.findByLabelText('show horizontal scrollbar').click();
            });

            it('horizontal scrollbar has data-orientation="horizontal"', () => {
                getHorizontalScrollbar().should('have.attr', 'data-orientation', 'horizontal');
            });

            it('horizontal scrollbar has data-state attribute', () => {
                getHorizontalScrollbar().should('have.attr', 'data-state');
            });

            it('horizontal thumb has data-state attribute', () => {
                getHorizontalThumb().should('have.attr', 'data-state');
            });

            it('both scrollbars coexist', () => {
                getVerticalScrollbar().should('exist');
                getHorizontalScrollbar().should('exist');
            });
        });
    });

    // ── 3. Scrollbar Behavior ───────────────────────────────

    describe('scrollbar behavior', () => {
        describe('always type', () => {
            beforeEach(() => {
                cy.findByLabelText('always').click();
            });

            it('scrollbar is visible without interaction', () => {
                getVerticalScrollbar().should('be.visible');
            });

            it('thumb is visible without interaction', () => {
                getVerticalThumb().should('be.visible');
            });
        });

        describe('scrolling', () => {
            beforeEach(() => {
                cy.findByLabelText('always').click();
            });

            it('content can be scrolled programmatically', () => {
                cy.findByTestId('scroll-to-bottom').click();
                cy.get('[data-radix-scroll-area-viewport]').then(($el) => {
                    expect($el[0].scrollTop).to.be.greaterThan(0);
                });
            });

            it('scroll to top resets scroll position', () => {
                cy.findByTestId('scroll-to-bottom').click();
                cy.get('[data-radix-scroll-area-viewport]').should(($el) => {
                    expect($el[0].scrollTop).to.be.greaterThan(0);
                });
                cy.findByTestId('scroll-to-top').click();
                cy.get('[data-radix-scroll-area-viewport]').should(($el) => {
                    expect($el[0].scrollTop).to.equal(0);
                });
            });

            it('last item is visible after scrolling to bottom', () => {
                cy.findByTestId('scroll-to-bottom').click();
                // After scrolling to bottom, Item 50 should be in the viewport
                cy.get('[data-radix-scroll-area-viewport]').should(($el) => {
                    const el = $el[0];
                    const maxScroll = el.scrollHeight - el.clientHeight;
                    expect(el.scrollTop).to.be.closeTo(maxScroll, 1);
                });
            });
        });
    });

    // ── 4. Accessibility ────────────────────────────────────

    describe('accessibility', () => {
        it('viewport is a scrollable container', () => {
            // The scrollable element should have overflow set to allow scrolling
            cy.get('[data-radix-scroll-area-viewport]').should('have.css', 'overflow-y', 'scroll');
        });

        it('native scrollbar is hidden via CSS', () => {
            // Radix injects a style tag to hide native scrollbars
            cy.get('style').then(($styles) => {
                const styleTexts = Array.from($styles).map((s) => s.textContent);
                const hasScrollbarHiding = styleTexts.some(
                    (text) => text && text.includes('[data-radix-scroll-area-viewport]')
                );
                expect(hasScrollbarHiding).to.be.true;
            });
        });

        it('content items within viewport are accessible', () => {
            getViewport().findByText('Item 1').should('exist');
            getViewport().findByText('Item 25').should('exist');
        });
    });

    // ── 5. Variants ─────────────────────────────────────────

    describe('variants', () => {
        describe('type="auto"', () => {
            beforeEach(() => {
                cy.findByLabelText('auto').click();
            });

            it('scrollbar appears when content overflows', () => {
                // In auto mode, scrollbar should appear when there is overflow
                getVerticalScrollbar().should('exist');
            });
        });

        describe('type="always"', () => {
            beforeEach(() => {
                cy.findByLabelText('always').click();
            });

            it('scrollbar is always visible', () => {
                getVerticalScrollbar().should('be.visible');
                getVerticalThumb().should('be.visible');
            });
        });

        describe('horizontal scrollbar toggle', () => {
            beforeEach(() => {
                cy.findByLabelText('always').click();
            });

            it('horizontal scrollbar appears when enabled and content is wide', () => {
                getHorizontalScrollbar().should('not.exist');
                cy.findByLabelText('show horizontal scrollbar').click();
                getHorizontalScrollbar().should('exist');
                getHorizontalScrollbar().should('have.attr', 'data-orientation', 'horizontal');
            });

            it('horizontal scrollbar disappears when disabled', () => {
                cy.findByLabelText('show horizontal scrollbar').click();
                getHorizontalScrollbar().should('exist');
                cy.findByLabelText('show horizontal scrollbar').click();
                getHorizontalScrollbar().should('not.exist');
            });
        });
    });
});
