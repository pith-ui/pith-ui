describe('Popper', () => {
    beforeEach(() => {
        cy.visit('/popper');
    });

    // ── Helpers ──────────────────────────────────────────────

    function getWrapper(testId) {
        return cy.findByTestId(testId).closest('[data-radix-popper-content-wrapper]');
    }

    // ── 1. LTR start alignment ──────────────────────────────

    describe('LTR start alignment', () => {
        it('content left edge aligns with anchor left edge', () => {
            cy.findByTestId('ltr-anchor').then(($anchor) => {
                const anchorRect = $anchor[0].getBoundingClientRect();
                getWrapper('ltr-content').then(($wrapper) => {
                    const wrapperRect = $wrapper[0].getBoundingClientRect();
                    expect(wrapperRect.left).to.be.closeTo(
                        anchorRect.left,
                        1,
                        `wrapper.left=${wrapperRect.left} anchor.left=${anchorRect.left}`
                    );
                });
            });
        });
    });

    // ── 2. RTL start alignment ──────────────────────────────

    describe('RTL start alignment', () => {
        it('content right edge aligns with anchor right edge', () => {
            cy.findByTestId('rtl-anchor').then(($anchor) => {
                const anchorRect = $anchor[0].getBoundingClientRect();
                getWrapper('rtl-content').then(($wrapper) => {
                    const wrapperRect = $wrapper[0].getBoundingClientRect();
                    expect(wrapperRect.right).to.be.closeTo(
                        anchorRect.right,
                        1,
                        `wrapper.right=${wrapperRect.right} anchor.right=${anchorRect.right}`
                    );
                });
            });
        });
    });
});
