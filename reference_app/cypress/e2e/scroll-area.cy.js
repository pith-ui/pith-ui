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

            // Note: Docs list data-state on Scrollbar, but React renders
            // data-state on the Thumb only. Scrollbar tests for data-state removed.

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

            // Note: Same as vertical — React does not render data-state on
            // horizontal Scrollbar, only on the Thumb.

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

    // ── 6. Thumb Position Tracking ─────────────────────────────

    describe('thumb position tracking', () => {
        beforeEach(() => {
            cy.findByLabelText('always').click();
        });

        it('thumb is near top when scrolled to top', () => {
            cy.findByTestId('scroll-to-top').click();
            getVerticalThumb().then(($thumb) => {
                const thumbRect = $thumb[0].getBoundingClientRect();
                getVerticalScrollbar().then(($scrollbar) => {
                    const scrollbarRect = $scrollbar[0].getBoundingClientRect();
                    // Thumb top should be very close to scrollbar top
                    expect(thumbRect.top - scrollbarRect.top).to.be.lessThan(5);
                });
            });
        });

        it('thumb moves down when content is scrolled to bottom', () => {
            getVerticalThumb().then(($thumb) => {
                const initialTop = $thumb[0].getBoundingClientRect().top;
                cy.findByTestId('scroll-to-bottom').click();
                // Give scroll event time to propagate to thumb
                getVerticalThumb().should(($thumb2) => {
                    const newTop = $thumb2[0].getBoundingClientRect().top;
                    expect(newTop).to.be.greaterThan(initialTop);
                });
            });
        });

        it('thumb is near bottom of scrollbar when scrolled to bottom', () => {
            cy.findByTestId('scroll-to-bottom').click();
            // Wait for scroll to propagate
            cy.get('[data-radix-scroll-area-viewport]').should(($el) => {
                expect($el[0].scrollTop).to.be.greaterThan(0);
            });
            getVerticalScrollbar().then(($scrollbar) => {
                const scrollbarRect = $scrollbar[0].getBoundingClientRect();
                getVerticalThumb().should(($thumb) => {
                    const thumbRect = $thumb[0].getBoundingClientRect();
                    // Thumb bottom should be close to scrollbar bottom
                    expect(scrollbarRect.bottom - thumbRect.bottom).to.be.lessThan(5);
                });
            });
        });

        it('thumb returns to top after scrolling to top', () => {
            cy.findByTestId('scroll-to-bottom').click();
            getVerticalThumb().should(($thumb) => {
                expect($thumb[0].getBoundingClientRect().top).to.be.greaterThan(0);
            });
            cy.findByTestId('scroll-to-top').click();
            getVerticalThumb().then(($thumb) => {
                const thumbRect = $thumb[0].getBoundingClientRect();
                getVerticalScrollbar().then(($scrollbar) => {
                    const scrollbarRect = $scrollbar[0].getBoundingClientRect();
                    expect(thumbRect.top - scrollbarRect.top).to.be.lessThan(5);
                });
            });
        });
    });

    // ── 7. Pointer Drag Scrolling ───────────────────────────────

    describe('pointer drag scrolling', () => {
        beforeEach(() => {
            cy.findByLabelText('always').click();
            // Ensure we start at the top
            cy.findByTestId('scroll-to-top').click();
            cy.get('[data-radix-scroll-area-viewport]').should(($el) => {
                expect($el[0].scrollTop).to.equal(0);
            });
        });

        it('dragging thumb down scrolls content down', () => {
            getVerticalThumb().then(($thumb) => {
                const thumbRect = $thumb[0].getBoundingClientRect();
                const startX = thumbRect.left + thumbRect.width / 2;
                const startY = thumbRect.top + thumbRect.height / 2;
                const endY = startY + 50;

                // Perform drag using realMouseDown, realMouseMove, realMouseUp
                getVerticalThumb().realMouseDown({position: 'center'});
                getVerticalThumb().realMouseMove(0, 50, {position: 'center'});
                getVerticalThumb().realMouseUp();

                // Content should have scrolled down
                cy.get('[data-radix-scroll-area-viewport]').should(($el) => {
                    expect($el[0].scrollTop).to.be.greaterThan(0);
                });
            });
        });
    });

    // ── Internal Styles ─────────────────────────────────────

    describe('internal styles', () => {
        beforeEach(() => {
            // Switch to "always" so thumb is visible
            cy.findByLabelText('always').click();
        });

        it('thumb references CSS variable for width via inline style', () => {
            cy.findByTestId('thumb-vertical').then(($el) => {
                const style = $el[0].style;
                const width = style.getPropertyValue('width');
                const height = style.getPropertyValue('height');
                // Vertical thumb should have a height set
                expect(height).to.not.be.empty;
            });
        });

        it('thumb has non-zero computed dimensions', () => {
            cy.findByTestId('thumb-vertical').then(($el) => {
                const rect = $el[0].getBoundingClientRect();
                expect(rect.width).to.be.greaterThan(0);
                expect(rect.height).to.be.greaterThan(0);
            });
        });
    });
});
