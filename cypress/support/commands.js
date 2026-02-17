// @ts-check
/// <reference types="cypress" />
/// <reference types="./index.d.ts" />

// Visit a Leptos story in embed mode (no nav shell).
// Usage: cy.visitStory('dialog/cypress')
//   => navigates to /dialog/cypress?embed
Cypress.Commands.add('visitStory', (storyPath, options) => {
    return cy.visit(`/${storyPath}?embed`, options);
});
