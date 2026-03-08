# Headless UI Component Principles

Guiding principles for building a headless UI component library — components that own behavior, state, and accessibility while leaving rendering entirely to the consumer.

---

## 1. Separate Logic from Presentation

A headless component encapsulates *what a thing does*, never *how it looks*. It manages state, keyboard interactions, focus behavior, and ARIA semantics. It produces no visual output of its own. The consumer provides all markup, styling, and layout.

This separation is the foundational contract of the library. Every design decision should reinforce it.

## 2. Own Accessibility Completely

Accessibility is the primary value a headless component delivers. Each component must handle the full scope of its WAI-ARIA pattern: roles, states, properties, keyboard navigation, focus management, and screen reader announcements. The consumer should never need to manually wire `aria-*` attributes or key handlers for baseline compliance.

If a component cannot guarantee accessible defaults, it is incomplete.

## 3. Invert Control to the Consumer

The consumer decides what renders. The component provides the props, state, and event handlers needed to make that rendering correct and accessible. Expose this contract through a clear mechanism — render props, slot functions, or hook return values — and keep it consistent across the library.

Consumers should feel like they are *assembling* behavior, not fighting an abstraction.

## 4. Make State Controllable

Every stateful value should support both uncontrolled (internal default) and controlled (consumer-driven) modes. A disclosure can manage its own open/closed state, but must also accept that state from the outside when the consumer needs coordination across components.

Default to sensible uncontrolled behavior. Upgrade to controlled seamlessly.

## 5. Compose, Don't Configure

Prefer composition over configuration props. Rather than a single monolithic component with dozens of options, expose smaller primitives — trigger, content, item, label — that compose into the full pattern. This keeps the API surface narrow for each piece and gives consumers structural flexibility without escape hatches.

A well-composed API rarely needs an `as`, `component`, or `render` override prop.

## 6. Minimize Opinions

Avoid shipping default styles, wrapper elements, class names, or animation behavior. Every opinion the library introduces is a potential conflict with the consumer's system. Where structural DOM is unavoidable, document it explicitly and keep it minimal.

The goal is zero friction when integrating into any design system.

## 7. Keep the API Surface Small

Each component should expose the fewest possible props, callbacks, and sub-components needed to fulfill its pattern. Resist adding convenience props that serve narrow use cases — they expand the maintenance surface and create ambiguity. Anything not essential to the core interaction contract belongs in userland.

A small API is easier to learn, harder to misuse, and cheaper to maintain.

## 8. Be Predictable

State transitions, event timing, and focus behavior should be deterministic and consistent across the library. If one component calls `onChange` after state updates, all components should. If one component traps focus, it should follow the same focus-trap rules as every other.

Establish conventions early, document them, and enforce them in review.

## 9. Design for Server Rendering

Components should produce correct initial HTML without requiring client-side JavaScript to hydrate into a valid state. Avoid layout shifts, missing ARIA attributes on first paint, or reliance on effects for initial rendering. Treat SSR and progressive enhancement as first-class constraints, not afterthoughts.

## 10. Document the Contract, Not the Visuals

Documentation should focus on the behavioral contract: what state is managed, what props are passed through, what keyboard interactions are handled, and what ARIA semantics are applied. Visual examples are helpful for context but must never imply that a particular rendering is "correct." The whole point is that many renderings are correct.

---

## Summary

A headless component succeeds when the consumer can build any visual expression of an interaction pattern — with full accessibility, predictable state, and no styling conflicts — using only the primitives the library provides. Ship behavior. Ship accessibility. Ship nothing else.