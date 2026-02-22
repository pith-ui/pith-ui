describe('Debug Accordion Header', () => {
    it('check value context directly', () => {
        cy.visit('/accordion');
        cy.findByRole('button', {name: 'Item 1'}).click();
        cy.findByRole('button', {name: 'Item 1'}).should('have.attr', 'data-state', 'open');
        cy.wait(1000);
        cy.findByRole('button', {name: 'Item 1'}).closest('h3').then(($h3) => {
            const attrs = {};
            for (let i = 0; i < $h3[0].attributes.length; i++) {
                const attr = $h3[0].attributes[i];
                attrs[attr.name] = attr.value;
            }
            expect('H3_ATTRS: ' + JSON.stringify(attrs)).to.equal('STOP');
        });
    });
});
