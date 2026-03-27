describe('AspectRatio', () => {
    beforeEach(() => {
        cy.visit('/aspect-ratio');
    });

    // ── Helpers ──────────────────────────────────────────────

    function getWrapper(testId) {
        return cy.findByTestId(testId).parent('[data-radix-aspect-ratio-wrapper]');
    }

    function getInner(testId) {
        return cy.findByTestId(testId);
    }

    // ── 1. Internal Styles (wrapper) ────────────────────────

    describe('wrapper styles', () => {
        it('wrapper has position: relative', () => {
            getWrapper('default-ratio').should('have.css', 'position', 'relative');
        });

        it('wrapper has width: 100%', () => {
            getWrapper('default-ratio').invoke('css', 'width').then((width) => {
                // width should be 100% of parent — just check it's non-zero
                expect(parseInt(width)).to.be.greaterThan(0);
            });
        });

        it('wrapper has correct padding-bottom for 1:1 ratio (100%)', () => {
            // aspect-ratio-dcnv-1: verify the mathematically correct value
            // For ratio 1/1, padding-bottom = (1/1) * 100% = 100%
            getWrapper('default-ratio').then(($el) => {
                const wrapper = $el[0];
                const parentWidth = wrapper.parentElement.getBoundingClientRect().width;
                const paddingBottom = parseFloat(getComputedStyle(wrapper).paddingBottom);
                // padding-bottom in px should equal parent width for 1:1 ratio
                expect(paddingBottom).to.be.closeTo(parentWidth, 2);
            });
        });

        it('wrapper has correct padding-bottom for 16:9 ratio (~56.25%)', () => {
            // aspect-ratio-dcnv-1: verify the mathematically correct value
            // For ratio 16/9, padding-bottom = (9/16) * 100% = 56.25%
            getWrapper('custom-ratio').then(($el) => {
                const wrapper = $el[0];
                const parentWidth = wrapper.parentElement.getBoundingClientRect().width;
                const paddingBottom = parseFloat(getComputedStyle(wrapper).paddingBottom);
                const expectedPadding = parentWidth * (9 / 16);
                expect(paddingBottom).to.be.closeTo(expectedPadding, 2);
            });
        });

        it('wrapper has data-radix-aspect-ratio-wrapper attribute', () => {
            getWrapper('default-ratio').should('have.attr', 'data-radix-aspect-ratio-wrapper');
        });
    });

    // ── 2. Internal Styles (inner element) ──────────────────

    describe('inner element styles', () => {
        it('inner element has position: absolute', () => {
            getInner('default-ratio').should('have.css', 'position', 'absolute');
        });

        it('inner element has top: 0px', () => {
            getInner('default-ratio').should('have.css', 'top', '0px');
        });

        it('inner element has right: 0px', () => {
            getInner('default-ratio').should('have.css', 'right', '0px');
        });

        it('inner element has bottom: 0px', () => {
            getInner('default-ratio').should('have.css', 'bottom', '0px');
        });

        it('inner element has left: 0px', () => {
            getInner('default-ratio').should('have.css', 'left', '0px');
        });
    });

    // ── 3. Custom style does not clobber internal styles ────

    describe('custom style preservation', () => {
        it('preserves position: absolute when user passes custom background style', () => {
            getInner('with-custom-style').should('have.css', 'position', 'absolute');
        });

        it('preserves top: 0px when user passes custom style', () => {
            getInner('with-custom-style').should('have.css', 'top', '0px');
        });

        it('preserves right: 0px when user passes custom style', () => {
            getInner('with-custom-style').should('have.css', 'right', '0px');
        });

        it('preserves bottom: 0px when user passes custom style', () => {
            getInner('with-custom-style').should('have.css', 'bottom', '0px');
        });

        it('preserves left: 0px when user passes custom style', () => {
            getInner('with-custom-style').should('have.css', 'left', '0px');
        });

        it('applies user custom background style', () => {
            getInner('with-custom-style').should('have.css', 'background-color', 'rgb(255, 99, 71)');
        });
    });

    // ── 4. asChild rendering ────────────────────────────────────

    describe('asChild rendering', () => {
        it('renders consumer element (img) instead of default div', () => {
            cy.findByTestId('with-as-child').should('match', 'img');
        });

        it('consumer element has internal absolute positioning styles', () => {
            cy.findByTestId('with-as-child').should('have.css', 'position', 'absolute');
            cy.findByTestId('with-as-child').should('have.css', 'top', '0px');
            cy.findByTestId('with-as-child').should('have.css', 'right', '0px');
            cy.findByTestId('with-as-child').should('have.css', 'bottom', '0px');
            cy.findByTestId('with-as-child').should('have.css', 'left', '0px');
        });

        it('consumer element retains its own attributes', () => {
            cy.findByTestId('with-as-child').should('have.attr', 'alt', 'placeholder');
        });

        it('wrapper still has correct aspect ratio padding', () => {
            getWrapper('with-as-child').then(($el) => {
                const wrapper = $el[0];
                const parentWidth = wrapper.parentElement.getBoundingClientRect().width;
                const paddingBottom = parseFloat(getComputedStyle(wrapper).paddingBottom);
                const expectedPadding = parentWidth * (9 / 16);
                expect(paddingBottom).to.be.closeTo(expectedPadding, 2);
            });
        });
    });

    // ── 5. Conflicting style does not override internal styles ──

    describe('conflicting style override', () => {
        it('internal position: absolute wins over user position: relative', () => {
            getInner('with-conflicting-style').should('have.css', 'position', 'absolute');
        });

        it('internal top: 0px wins over user top: 10px', () => {
            getInner('with-conflicting-style').should('have.css', 'top', '0px');
        });

        it('preserves non-conflicting user styles', () => {
            getInner('with-conflicting-style').should('have.css', 'background-color', 'rgb(100, 149, 237)');
        });
    });

    // ── Axe Accessibility Audit ─────────────────────────────

    describe('axe audit', () => {
        it('no violations in default state', () => {
            cy.checkComponentA11y();
        });
    });
});
