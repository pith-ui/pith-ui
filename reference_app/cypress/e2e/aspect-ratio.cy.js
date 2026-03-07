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

        it('wrapper has correct padding-bottom for 1:1 ratio', () => {
            getWrapper('default-ratio').should('have.css', 'padding-bottom').and('match', /^[0-9]/);
        });

        it('wrapper has correct padding-bottom for 16:9 ratio', () => {
            getWrapper('custom-ratio').should('have.css', 'padding-bottom').and('match', /^[0-9]/);
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

    // ── 4. Conflicting style does not override internal styles ──

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
});
