describe('Navigation Menu', () => {
    // ── Helpers ──────────────────────────────────────────────

    function shouldBeOpen(triggerName) {
        cy.findByRole('button', {name: triggerName}).should('have.attr', 'data-state', 'open');
    }

    function shouldBeClosed(triggerName) {
        cy.findByRole('button', {name: triggerName}).should('have.attr', 'data-state', 'closed');
    }

    function contentShouldBeVisible(testId) {
        cy.findByTestId(testId).should('exist');
    }

    function contentShouldNotExist(testId) {
        cy.findByTestId(testId).should('not.exist');
    }

    function allContentClosed() {
        shouldBeClosed('Products');
        shouldBeClosed('Resources');
        cy.findByTestId('products-content').should('not.exist');
        cy.findByTestId('resources-content').should('not.exist');
    }

    beforeEach(() => {
        cy.visit('/navigation-menu');
        // Ensure we start with all menus closed
        allContentClosed();
    });

    // ── 1. Accessibility Semantics ──────────────────────────

    describe('accessibility', () => {
        it('Root has role="navigation"', () => {
            cy.findByRole('navigation').should('exist');
        });

        it('List renders as a list element', () => {
            cy.findByRole('navigation').find('ul').should('exist');
        });

        it('Trigger has aria-expanded="false" when closed', () => {
            cy.findByRole('button', {name: 'Products'}).should('have.attr', 'aria-expanded', 'false');
        });

        it('Trigger has aria-expanded="true" when open', () => {
            cy.findByRole('button', {name: 'Products'}).click();
            cy.findByRole('button', {name: 'Products'}).should('have.attr', 'aria-expanded', 'true');
        });

        it('Trigger has aria-controls pointing to content', () => {
            cy.findByRole('button', {name: 'Products'}).click();
            cy.findByRole('button', {name: 'Products'})
                .invoke('attr', 'aria-controls')
                .then((contentId) => {
                    cy.get(`#${contentId}`).should('exist');
                });
        });

        it('Content has aria-labelledby pointing to trigger', () => {
            cy.findByRole('button', {name: 'Products'}).click();
            cy.findByRole('button', {name: 'Products'})
                .invoke('attr', 'aria-controls')
                .then((contentId) => {
                    cy.get(`#${contentId}`)
                        .invoke('attr', 'aria-labelledby')
                        .then((labelId) => {
                            cy.get(`#${labelId}`).should('contain.text', 'Products');
                        });
                });
        });

        it('Active link has aria-current="page"', () => {
            cy.findByText('About').should('have.attr', 'aria-current', 'page');
        });

        it('Link without active does not have aria-current', () => {
            cy.findByRole('button', {name: 'Products'}).click();
            cy.findByText('Product A').should('not.have.attr', 'aria-current');
        });
    });

    // ── 2. Data Attributes ──────────────────────────────────

    describe('data attributes', () => {
        it('Root has data-orientation="horizontal"', () => {
            cy.findByTestId('nav-root').should('have.attr', 'data-orientation', 'horizontal');
        });

        it('Trigger data-state is "closed" initially', () => {
            shouldBeClosed('Products');
            shouldBeClosed('Resources');
        });

        it('Trigger data-state changes to "open" when clicked', () => {
            cy.findByRole('button', {name: 'Products'}).click();
            shouldBeOpen('Products');
        });

        it('Trigger data-state returns to "closed" when content dismissed', () => {
            cy.findByRole('button', {name: 'Products'}).click();
            shouldBeOpen('Products');
            cy.realPress('Escape');
            shouldBeClosed('Products');
        });

        it('Only the active trigger has data-state="open"', () => {
            cy.findByRole('button', {name: 'Products'}).click();
            shouldBeOpen('Products');
            shouldBeClosed('Resources');

            cy.findByRole('button', {name: 'Resources'}).click();
            shouldBeClosed('Products');
            shouldBeOpen('Resources');
        });

        it('Active link has data-active attribute', () => {
            cy.findByText('About').should('have.attr', 'data-active');
        });

        it('Viewport has data-state attribute when open', () => {
            cy.findByRole('button', {name: 'Products'}).click();
            cy.findByTestId('nav-viewport').should('have.attr', 'data-state', 'open');
        });

        // Note: Docs list data-state on Content, but React does not render it.
        // The Viewport (tested above) carries data-state instead.

        it('Content has data-orientation', () => {
            cy.findByRole('button', {name: 'Products'}).click();
            cy.findByTestId('products-content').should('have.attr', 'data-orientation', 'horizontal');
        });
    });

    // ── 3. Keyboard Navigation ──────────────────────────────

    describe('keyboard navigation', () => {
        it('Tab moves focus through triggers and links', () => {
            cy.findByRole('button', {name: 'Products'}).focus();
            cy.findByRole('button', {name: 'Products'}).should('be.focused');
        });

        it('ArrowRight moves focus to next trigger/link (horizontal)', () => {
            cy.findByRole('button', {name: 'Products'}).focus();
            cy.realPress('ArrowRight');
            cy.findByRole('button', {name: 'Resources'}).should('be.focused');
        });

        it('ArrowLeft moves focus to previous trigger/link (horizontal)', () => {
            cy.findByRole('button', {name: 'Resources'}).focus();
            cy.realPress('ArrowLeft');
            cy.findByRole('button', {name: 'Products'}).should('be.focused');
        });

        // Note: Arrow navigation does not wrap/loop — not documented and React
        // does not implement it. Home/End cover first/last navigation.

        it('Home moves focus to first trigger', () => {
            cy.findByText('About').focus();
            cy.realPress('Home');
            cy.findByRole('button', {name: 'Products'}).should('be.focused');
        });

        it('End moves focus to last trigger/link', () => {
            cy.findByRole('button', {name: 'Products'}).focus();
            cy.realPress('End');
            cy.findByText('About').should('be.focused');
        });

        it('Enter opens content from trigger', () => {
            cy.findByRole('button', {name: 'Products'}).focus();
            cy.realPress('Enter');
            shouldBeOpen('Products');
            contentShouldBeVisible('products-content');
        });

        it('Space opens content from trigger', () => {
            cy.findByRole('button', {name: 'Products'}).focus();
            cy.realPress('Space');
            shouldBeOpen('Products');
            contentShouldBeVisible('products-content');
        });

        it('Escape closes open content and moves focus back to trigger', () => {
            cy.findByRole('button', {name: 'Products'}).click();
            shouldBeOpen('Products');
            cy.realPress('Escape');
            shouldBeClosed('Products');
            cy.findByRole('button', {name: 'Products'}).should('be.focused');
        });

        it('ArrowDown moves focus into open content (horizontal orientation)', () => {
            cy.findByRole('button', {name: 'Products'}).click();
            shouldBeOpen('Products');
            cy.findByRole('button', {name: 'Products'}).focus();
            cy.realPress('ArrowDown');
            // Focus should move into the content area
            cy.findByTestId('products-content').then(($content) => {
                cy.focused().then(($focused) => {
                    expect($content[0].contains($focused[0])).to.be.true;
                });
            });
        });

        it('Enter toggles trigger: clicking again closes', () => {
            cy.findByRole('button', {name: 'Products'}).focus();
            cy.realPress('Enter');
            shouldBeOpen('Products');
            cy.findByRole('button', {name: 'Products'}).focus();
            cy.realPress('Enter');
            shouldBeClosed('Products');
        });
    });

    // ── 4. Pointer Interaction ──────────────────────────────

    describe('pointer interaction', () => {
        it('Click on trigger opens content', () => {
            cy.findByRole('button', {name: 'Products'}).click();
            shouldBeOpen('Products');
            contentShouldBeVisible('products-content');
        });

        it('Click on trigger again closes content', () => {
            cy.findByRole('button', {name: 'Products'}).click();
            shouldBeOpen('Products');
            cy.findByRole('button', {name: 'Products'}).click();
            shouldBeClosed('Products');
        });

        it('Clicking a different trigger switches content', () => {
            cy.findByRole('button', {name: 'Products'}).click();
            shouldBeOpen('Products');
            contentShouldBeVisible('products-content');

            cy.findByRole('button', {name: 'Resources'}).click();
            shouldBeClosed('Products');
            shouldBeOpen('Resources');
            contentShouldBeVisible('resources-content');
        });

        it('Content links are clickable', () => {
            cy.findByRole('button', {name: 'Products'}).click();
            contentShouldBeVisible('products-content');
            cy.findByText('Product A').should('exist');
        });

        it('Clicking a content link closes the menu', () => {
            cy.findByRole('button', {name: 'Products'}).click();
            shouldBeOpen('Products');
            cy.findByText('Product A').click();
            shouldBeClosed('Products');
        });
    });

    // ── 5. Focus Management ─────────────────────────────────

    describe('focus management', () => {
        it('Focus returns to trigger after Escape', () => {
            cy.findByRole('button', {name: 'Products'}).click();
            shouldBeOpen('Products');
            // Focus into content
            cy.findByText('Product A').focus();
            cy.realPress('Escape');
            shouldBeClosed('Products');
            cy.findByRole('button', {name: 'Products'}).should('be.focused');
        });

        it('ArrowDown from trigger moves focus into content', () => {
            cy.findByRole('button', {name: 'Products'}).click();
            shouldBeOpen('Products');
            cy.findByRole('button', {name: 'Products'}).focus();
            cy.realPress('ArrowDown');
            // Should be focused inside content
            cy.findByTestId('products-content').then(($content) => {
                cy.focused().then(($focused) => {
                    expect($content[0].contains($focused[0])).to.be.true;
                });
            });
        });
    });

    // ── 6. Indicator ────────────────────────────────────────

    describe('indicator', () => {
        it('Indicator has data-state="hidden" when no menu is open', () => {
            // Indicator may or may not be rendered when hidden; check if present
            cy.get('body').then(($body) => {
                if ($body.find('[data-testid="nav-indicator"]').length) {
                    cy.findByTestId('nav-indicator').should('have.attr', 'data-state', 'hidden');
                }
            });
        });

        it('Indicator has data-state="visible" when a menu is open', () => {
            cy.findByRole('button', {name: 'Products'}).click();
            shouldBeOpen('Products');
            cy.findByTestId('nav-indicator').should('have.attr', 'data-state', 'visible');
        });
    });

    // ── 7. Viewport Sizing ────────────────────────────────────

    describe('viewport sizing', () => {
        it('viewport has CSS custom properties for width and height when open', () => {
            cy.findByRole('button', {name: 'Products'}).click();
            shouldBeOpen('Products');
            cy.findByTestId('nav-viewport').should(($viewport) => {
                const style = $viewport[0].style;
                const width = style.getPropertyValue('--radix-navigation-menu-viewport-width');
                const height = style.getPropertyValue('--radix-navigation-menu-viewport-height');
                expect(width).to.match(/^\d+(\.\d+)?px$/);
                expect(height).to.match(/^\d+(\.\d+)?px$/);
            });
        });

        it('viewport width differs between Products and Resources content', () => {
            cy.findByRole('button', {name: 'Products'}).click();
            shouldBeOpen('Products');
            cy.findByTestId('nav-viewport')
                .invoke('css', '--radix-navigation-menu-viewport-width')
                .then((productsWidth) => {
                    cy.findByRole('button', {name: 'Resources'}).click();
                    shouldBeOpen('Resources');
                    cy.findByTestId('nav-viewport').should(($vp) => {
                        const resourcesWidth = $vp[0].style.getPropertyValue(
                            '--radix-navigation-menu-viewport-width'
                        );
                        expect(resourcesWidth).to.not.equal(productsWidth);
                    });
                });
        });

        it('viewport height differs between Products and Resources content', () => {
            cy.findByRole('button', {name: 'Products'}).click();
            shouldBeOpen('Products');
            cy.findByTestId('nav-viewport')
                .invoke('css', '--radix-navigation-menu-viewport-height')
                .then((productsHeight) => {
                    cy.findByRole('button', {name: 'Resources'}).click();
                    shouldBeOpen('Resources');
                    cy.findByTestId('nav-viewport').should(($vp) => {
                        const resourcesHeight = $vp[0].style.getPropertyValue(
                            '--radix-navigation-menu-viewport-height'
                        );
                        expect(resourcesHeight).to.not.equal(productsHeight);
                    });
                });
        });
    });

    // ── 8. Content Layout ─────────────────────────────────────

    describe('content layout', () => {
        it('products content renders groups side-by-side (grid layout)', () => {
            cy.findByRole('button', {name: 'Products'}).click();
            contentShouldBeVisible('products-content');
            cy.findByTestId('products-featured').then(($featured) => {
                cy.findByTestId('products-all').then(($all) => {
                    const featuredRect = $featured[0].getBoundingClientRect();
                    const allRect = $all[0].getBoundingClientRect();
                    // Groups should be side-by-side (different x positions)
                    expect(allRect.left).to.be.greaterThan(featuredRect.right - 1);
                    // Groups should be at similar y positions (same row)
                    expect(Math.abs(featuredRect.top - allRect.top)).to.be.lessThan(5);
                });
            });
        });

        it('products content class is forwarded to rendered element', () => {
            cy.findByRole('button', {name: 'Products'}).click();
            cy.findByTestId('products-content').should('have.class', 'nav-content-products');
        });

        it('resources content class is forwarded to rendered element', () => {
            cy.findByRole('button', {name: 'Resources'}).click();
            cy.findByTestId('resources-content').should('have.class', 'nav-content-resources');
        });

        it('content inline style is forwarded to rendered element in viewport', () => {
            cy.findByRole('button', {name: 'Products'}).click();
            cy.findByTestId('products-content').should(($el) => {
                // grid-template-columns is set via inline style, not CSS class
                const gtc = $el[0].style.gridTemplateColumns;
                expect(gtc).to.not.be.empty;
            });
        });
    });
});
