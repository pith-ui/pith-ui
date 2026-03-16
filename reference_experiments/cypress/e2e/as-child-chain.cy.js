// ── Experiment: asChild Chain — Multi-Level AttributeInterceptor Forwarding ──
//
// Tests whether attrs survive through multiple layers of AttributeInterceptor
// + ForwardedAttrs.spread(), simulating the real Radix component chain:
//   MenuContent → DismissableLayer → FocusScope → RovingFocusGroup → PopperContent
//
// Key questions:
// 1. Do parent attrs reach the innermost element through N interceptor layers?
// 2. Does each layer's own attrs coexist with forwarded parent attrs?
// 3. Do style: directives from multiple layers survive on the final element?
// 4. Does the PopperContent two-element pattern (wrapper + inner) work correctly?

describe('asChild Chain — Multi-Level Attribute Forwarding', () => {
    beforeEach(() => {
        cy.visit('/as-child-chain');
    });

    // ── 1. Single transparent layer ──────────────────────────────────────

    describe('single transparent layer', () => {
        it('parent data attr reaches inner element', () => {
            cy.get('[data-testid="single-inner"]')
                .should('have.attr', 'data-from-parent', 'parent-value');
        });

        it('parent class reaches inner element', () => {
            cy.get('[data-testid="single-inner"]')
                .should('have.class', 'parent-class');
        });

        it('layer own attr is present', () => {
            cy.get('[data-testid="single-inner"]')
                .should('have.attr', 'data-layer', 'single');
        });
    });

    // ── 2. Two transparent layers ────────────────────────────────────────

    describe('two transparent layers', () => {
        it('parent data attr reaches innermost element', () => {
            cy.get('[data-testid="two-layer-inner"]')
                .should('have.attr', 'data-from-parent', 'parent-value');
        });

        it('parent class reaches innermost element', () => {
            cy.get('[data-testid="two-layer-inner"]')
                .should('have.class', 'parent-class');
        });

        it('outer layer attr reaches innermost element', () => {
            cy.get('[data-testid="two-layer-inner"]')
                .should('have.attr', 'data-layer-outer', 'was-here');
        });

        it('inner layer own attr is present', () => {
            cy.get('[data-testid="two-layer-inner"]')
                .should('have.attr', 'data-layer', 'inner');
        });
    });

    // ── 3. Three transparent layers ──────────────────────────────────────

    describe('three transparent layers', () => {
        it('parent data attr reaches innermost element', () => {
            cy.get('[data-testid="three-layer-innermost"]')
                .should('have.attr', 'data-from-parent', 'parent-value');
        });

        it('parent class reaches innermost element', () => {
            cy.get('[data-testid="three-layer-innermost"]')
                .should('have.class', 'parent-class');
        });

        it('outer layer attr reaches innermost element', () => {
            cy.get('[data-testid="three-layer-innermost"]')
                .should('have.attr', 'data-outer', 'was-here');
        });

        it('middle layer attr reaches innermost element', () => {
            cy.get('[data-testid="three-layer-innermost"]')
                .should('have.attr', 'data-middle', 'was-here');
        });

        it('innermost layer own attr is present', () => {
            cy.get('[data-testid="three-layer-innermost"]')
                .should('have.attr', 'data-layer', 'innermost');
        });
    });

    // ── 4. Chain with style: directives at each level ────────────────────

    describe('style: directives through chain', () => {
        it('parent style var reaches innermost', () => {
            cy.get('[data-testid="styled-innermost"]').then(($el) => {
                const val = $el[0].style.getPropertyValue('--parent-var');
                cy.log(`--parent-var: "${val}"`);
                expect(val.trim()).to.equal('parent-value');
            });
        });

        it('outer style var reaches innermost', () => {
            cy.get('[data-testid="styled-innermost"]').then(($el) => {
                const val = $el[0].style.getPropertyValue('--outer-var');
                cy.log(`--outer-var: "${val}"`);
                expect(val.trim()).to.equal('outer-value');
            });
        });

        it('middle style var reaches innermost', () => {
            cy.get('[data-testid="styled-innermost"]').then(($el) => {
                const val = $el[0].style.getPropertyValue('--middle-var');
                cy.log(`--middle-var: "${val}"`);
                expect(val.trim()).to.equal('middle-value');
            });
        });

        it('innermost own style var is present', () => {
            cy.get('[data-testid="styled-innermost"]').then(($el) => {
                const val = $el[0].style.getPropertyValue('--innermost-var');
                cy.log(`--innermost-var: "${val}"`);
                expect(val.trim()).to.equal('innermost-value');
            });
        });
    });

    // ── 5. Chain ending with wrapper + inner (PopperContent pattern) ─────

    describe('PopperContent-like terminal (wrapper + inner)', () => {
        it('parent data attr is on inner, NOT wrapper', () => {
            cy.get('[data-testid="popper-inner"]')
                .should('have.attr', 'data-from-parent', 'parent-value');
            cy.get('[data-testid="popper-wrapper"]')
                .should('not.have.attr', 'data-from-parent');
        });

        it('parent class is on inner, NOT wrapper', () => {
            cy.get('[data-testid="popper-inner"]')
                .should('have.class', 'parent-class');
            cy.get('[data-testid="popper-wrapper"]')
                .should('not.have.class', 'parent-class');
        });

        it('chain attrs (focus-scope, dismissable) are on inner', () => {
            cy.get('[data-testid="popper-inner"]')
                .should('have.attr', 'data-focus-scope', 'true');
            cy.get('[data-testid="popper-inner"]')
                .should('have.attr', 'data-dismissable', 'true');
        });

        it('wrapper has only positioning styles', () => {
            cy.get('[data-testid="popper-wrapper"]').then(($el) => {
                expect($el[0].style.position).to.equal('fixed');
                // Should NOT have user vars
                const userVar = $el[0].style.getPropertyValue('--user-var');
                expect(userVar.trim()).to.equal('');
            });
        });

        it('inner has internal popper style var', () => {
            cy.get('[data-testid="popper-inner"]').then(($el) => {
                const val = $el[0].style.getPropertyValue('--popper-transform-origin');
                cy.log(`--popper-transform-origin: "${val}"`);
                expect(val.trim()).to.equal('center');
            });
        });

        it('user style var reaches inner', () => {
            cy.get('[data-testid="popper-inner"]').then(($el) => {
                const val = $el[0].style.getPropertyValue('--user-var');
                cy.log(`--user-var: "${val}"`);
                expect(val.trim()).to.equal('user-value');
            });
        });
    });

    // ── 6. Children passthrough (no interceptor) ─────────────────────────

    // NOTE: ChildrenPassthrough renders {children()} — Leptos applies add_any_attr
    // to the first DOM element it encounters in children. The attrs applied to
    // ChildrenPassthrough land on the children's root element.

    describe('children passthrough (no interceptor)', () => {
        it('parent data attr reaches inner via children passthrough', () => {
            cy.get('[data-testid="children-inner"]')
                .should('have.attr', 'data-from-parent', 'parent-value');
        });

        it('parent class reaches inner via children passthrough', () => {
            cy.get('[data-testid="children-inner"]')
                .should('have.class', 'parent-class');
        });
    });

    // ── 7. Mixed — interceptor + children passthrough ────────────────────

    describe('mixed chain (interceptor + children passthrough)', () => {
        it('parent data attr reaches inner through mixed chain', () => {
            cy.get('[data-testid="mixed-inner"]')
                .should('have.attr', 'data-from-parent', 'parent-value');
        });

        it('parent class reaches inner through mixed chain', () => {
            cy.get('[data-testid="mixed-inner"]')
                .should('have.class', 'parent-class');
        });

        it('inner layer own attr is present', () => {
            cy.get('[data-testid="mixed-inner"]')
                .should('have.attr', 'data-layer', 'inner');
        });
    });
});
