// ── Experiment: style: Directives with Option Values ─────────────────────────
//
// Tests whether style: directives with Option values properly:
// - Set the property when Some("value")
// - Omit/remove the property when None
// - Reactively toggle between set and removed
//
// Critical for converting Category 3 (dynamic property names) to style: directives.

describe('style: Directives with Option Values', () => {
    beforeEach(() => {
        cy.visit('/style-option');
    });

    // ── 1. Static Some vs None ──────────────────────────────────────────

    describe('static Some vs None', () => {
        it('Some("10px") sets the left property', () => {
            cy.get('[data-testid="with-some"]').then(($el) => {
                expect($el[0].style.left).to.equal('10px');
            });
        });

        it('None does NOT set the left property', () => {
            cy.get('[data-testid="with-none"]').then(($el) => {
                expect($el[0].style.left).to.equal('');
            });
        });
    });

    // ── 2. Reactive toggle between Some and None ─────────────────────────

    describe('reactive toggle', () => {
        it('initially has left: 20px', () => {
            cy.get('[data-testid="reactive-toggle"]').then(($el) => {
                expect($el[0].style.left).to.equal('20px');
            });
        });

        it('after toggle, left is removed', () => {
            cy.get('[data-testid="fixture-2"] button').click();
            cy.get('[data-testid="reactive-toggle"]').then(($el) => {
                expect($el[0].style.left).to.equal('');
            });
        });

        it('toggle back restores left', () => {
            cy.get('[data-testid="fixture-2"] button').click();
            cy.get('[data-testid="fixture-2"] button').click();
            cy.get('[data-testid="reactive-toggle"]').then(($el) => {
                expect($el[0].style.left).to.equal('20px');
            });
        });
    });

    // ── 3. Bidirectional (LTR/RTL) pattern ──────────────────────────────

    describe('bidirectional LTR/RTL corner', () => {
        it('LTR: right=0, left is empty', () => {
            cy.get('[data-testid="dir-value"]').should('have.text', 'ltr');
            cy.get('[data-testid="corner"]').then(($el) => {
                expect($el[0].style.right).to.equal('0px');
                expect($el[0].style.left).to.equal('');
            });
        });

        it('RTL: left=0, right is empty', () => {
            cy.get('[data-testid="fixture-3"] button').click();
            cy.get('[data-testid="dir-value"]').should('have.text', 'rtl');
            cy.get('[data-testid="corner"]').then(($el) => {
                expect($el[0].style.left).to.equal('0px');
                expect($el[0].style.right).to.equal('');
            });
        });

        it('toggle back to LTR restores right=0', () => {
            cy.get('[data-testid="fixture-3"] button').click();
            cy.get('[data-testid="fixture-3"] button').click();
            cy.get('[data-testid="dir-value"]').should('have.text', 'ltr');
            cy.get('[data-testid="corner"]').then(($el) => {
                expect($el[0].style.right).to.equal('0px');
                expect($el[0].style.left).to.equal('');
            });
        });
    });

    // ── 4. Slider-like orientation-dependent edges ───────────────────────

    describe('slider-like orientation edges', () => {
        it('horizontal: left and right are set, top and bottom are empty', () => {
            cy.get('[data-testid="orient-value"]').should('have.text', 'horizontal');
            cy.get('[data-testid="range"]').then(($el) => {
                expect($el[0].style.left).to.equal('20%');
                expect($el[0].style.right).to.equal('30%');
                expect($el[0].style.top).to.equal('');
                expect($el[0].style.bottom).to.equal('');
                expect($el[0].style.height).to.equal('100%');
                expect($el[0].style.width).to.equal('');
            });
        });

        it('vertical: top and bottom are set, left and right are empty', () => {
            cy.get('[data-testid="fixture-4"] button').click();
            cy.get('[data-testid="orient-value"]').should('have.text', 'vertical');
            cy.get('[data-testid="range"]').then(($el) => {
                expect($el[0].style.top).to.equal('30%');
                expect($el[0].style.bottom).to.equal('20%');
                expect($el[0].style.left).to.equal('');
                expect($el[0].style.right).to.equal('');
                expect($el[0].style.width).to.equal('100%');
                expect($el[0].style.height).to.equal('');
            });
        });

        it('toggle back to horizontal restores left/right', () => {
            cy.get('[data-testid="fixture-4"] button').click();
            cy.get('[data-testid="fixture-4"] button').click();
            cy.get('[data-testid="orient-value"]').should('have.text', 'horizontal');
            cy.get('[data-testid="range"]').then(($el) => {
                expect($el[0].style.left).to.equal('20%');
                expect($el[0].style.right).to.equal('30%');
                expect($el[0].style.top).to.equal('');
                expect($el[0].style.bottom).to.equal('');
            });
        });
    });

    // ── 5. Option style through ForwardedAttrs ──────────────────────────

    describe('Option style through ForwardedAttrs', () => {
        it('initially left=10px, right is empty', () => {
            cy.get('[data-testid="spread-target"]').then(($el) => {
                expect($el[0].style.left).to.equal('10px');
                expect($el[0].style.right).to.equal('');
                expect($el[0].style.position).to.equal('absolute');
            });
        });

        it('after toggle: right=10px, left is empty', () => {
            cy.get('[data-testid="fixture-5"] button').click();
            cy.get('[data-testid="spread-target"]').then(($el) => {
                expect($el[0].style.right).to.equal('10px');
                expect($el[0].style.left).to.equal('');
            });
        });
    });

    // ── 6. class:/style: directives instead of string props ──────────────

    describe('component with class: and style: directives (no string props)', () => {
        it('class:menu-content is applied', () => {
            cy.get('[data-testid="menu-content-inner"]')
                .should('have.class', 'menu-content');
        });

        it('class:menu-animated is initially absent', () => {
            cy.get('[data-testid="menu-content-inner"]')
                .should('not.have.class', 'menu-animated');
        });

        it('after toggle, class:menu-animated is applied', () => {
            cy.get('[data-testid="fixture-6"] button').click();
            cy.get('[data-testid="menu-content-inner"]')
                .should('have.class', 'menu-animated');
        });

        it('style: CSS var alias is on inner', () => {
            cy.get('[data-testid="menu-content-inner"]').then(($el) => {
                const val = $el[0].style.getPropertyValue('--radix-menu-transform-origin');
                expect(val.trim()).to.equal('var(--radix-popper-transform-origin)');
            });
        });

        it('internal style:outline=none is preserved alongside user style:', () => {
            cy.get('[data-testid="menu-content-inner"]').then(($el) => {
                expect($el[0].style.outline).to.equal('none');
            });
        });

        it('data attrs and role are on inner', () => {
            cy.get('[data-testid="menu-content-inner"]')
                .should('have.attr', 'data-state', 'open')
                .should('have.attr', 'role', 'menu');
        });
    });
});
