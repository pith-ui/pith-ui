/**
 * Attribute Forwarding Test Suite
 *
 * Verifies that user-provided spread attributes (data-attr-test) on each
 * component part reach the final DOM element. This is a systematic audit
 * of Leptos's attribute forwarding through component chains.
 *
 * The React app serves as the ground truth — every attribute must reach
 * the DOM. The Leptos app must match.
 */
describe('Attribute Forwarding', () => {
    beforeEach(() => {
        cy.visit('/attr-forwarding');
    });

    // ── Helper ──────────────────────────────────────────────
    function attrShouldExist(value) {
        cy.get(`[data-attr-test="${value}"]`).should('exist');
    }

    // ── Simple Components (shallow chain) ───────────────────

    describe('simple components', () => {
        it('Separator forwards attributes', () => {
            attrShouldExist('separator');
        });

        it('Label forwards attributes', () => {
            attrShouldExist('label');
        });

        it('Progress.Root forwards attributes', () => {
            attrShouldExist('progress-root');
        });

        it('Progress.Indicator forwards attributes', () => {
            attrShouldExist('progress-indicator');
        });

        it('Toggle forwards attributes', () => {
            attrShouldExist('toggle');
        });

        it('Switch.Root forwards attributes', () => {
            attrShouldExist('switch-root');
        });

        it('Checkbox.Root forwards attributes', () => {
            attrShouldExist('checkbox-root');
        });
    });

    // ── Accordion ───────────────────────────────────────────

    describe('Accordion', () => {
        it('Root forwards attributes', () => {
            attrShouldExist('accordion-root');
        });

        it('Item forwards attributes', () => {
            attrShouldExist('accordion-item');
        });

        it('Trigger forwards attributes', () => {
            attrShouldExist('accordion-trigger');
        });

        it('Content forwards attributes', () => {
            attrShouldExist('accordion-content');
        });

        it('Header forwards attributes', () => {
            attrShouldExist('accordion-header');
        });
    });

    // ── Collapsible ─────────────────────────────────────────

    describe('Collapsible', () => {
        it('Root forwards attributes', () => {
            attrShouldExist('collapsible-root');
        });

        it('Trigger forwards attributes', () => {
            attrShouldExist('collapsible-trigger');
        });

        it('Content forwards attributes', () => {
            attrShouldExist('collapsible-content');
        });
    });

    // ── Tabs ────────────────────────────────────────────────

    describe('Tabs', () => {
        it('Root forwards attributes', () => {
            attrShouldExist('tabs-root');
        });

        it('List forwards attributes', () => {
            attrShouldExist('tabs-list');
        });

        it('Trigger forwards attributes', () => {
            attrShouldExist('tabs-trigger');
        });

        it('Content forwards attributes', () => {
            attrShouldExist('tabs-content');
        });
    });

    // ── RadioGroup ──────────────────────────────────────────

    describe('RadioGroup', () => {
        it('Root forwards attributes', () => {
            attrShouldExist('radio-group-root');
        });

        it('Item forwards attributes', () => {
            attrShouldExist('radio-group-item');
        });

        it('Indicator forwards attributes', () => {
            attrShouldExist('radio-group-indicator');
        });
    });

    // ── Slider ──────────────────────────────────────────────

    describe('Slider', () => {
        it('Root forwards attributes', () => {
            attrShouldExist('slider-root');
        });

        it('Track forwards attributes', () => {
            attrShouldExist('slider-track');
        });

        it('Range forwards attributes', () => {
            attrShouldExist('slider-range');
        });

        it('Thumb forwards attributes', () => {
            attrShouldExist('slider-thumb');
        });
    });

    // ── Toolbar ─────────────────────────────────────────────

    describe('Toolbar', () => {
        it('Root forwards attributes', () => {
            attrShouldExist('toolbar-root');
        });

        it('Button forwards attributes', () => {
            attrShouldExist('toolbar-button');
        });

        it('Separator forwards attributes', () => {
            attrShouldExist('toolbar-separator');
        });

        it('Link forwards attributes', () => {
            attrShouldExist('toolbar-link');
        });
    });

    // ── ScrollArea ──────────────────────────────────────────

    describe('ScrollArea', () => {
        it('Root forwards attributes', () => {
            attrShouldExist('scroll-area-root');
        });

        it('Viewport forwards attributes', () => {
            attrShouldExist('scroll-area-viewport');
        });

        // ScrollArea Scrollbar/Thumb are skipped: React only renders them when
        // content overflows AND the scrollbar type is not "always". Testing these
        // reliably requires scroll-triggering content that works in headless mode.
        it.skip('Scrollbar forwards attributes', () => {
            attrShouldExist('scroll-area-scrollbar');
        });

        it.skip('Thumb forwards attributes', () => {
            attrShouldExist('scroll-area-thumb');
        });
    });

    // ── Dialog (portaled, medium chain) ─────────────────────

    describe('Dialog', () => {
        it('Trigger forwards attributes', () => {
            attrShouldExist('dialog-trigger');
        });

        it('Overlay forwards attributes', () => {
            attrShouldExist('dialog-overlay');
        });

        it('Content forwards attributes', () => {
            attrShouldExist('dialog-content');
        });

        it('Title forwards attributes', () => {
            attrShouldExist('dialog-title');
        });

        it('Description forwards attributes', () => {
            attrShouldExist('dialog-description');
        });

        it('Close forwards attributes', () => {
            attrShouldExist('dialog-close');
        });
    });

    // ── AlertDialog (portaled, medium chain) ────────────────

    describe('AlertDialog', () => {
        it('Trigger forwards attributes', () => {
            attrShouldExist('alert-dialog-trigger');
        });

        it('Overlay forwards attributes', () => {
            attrShouldExist('alert-dialog-overlay');
        });

        it('Content forwards attributes', () => {
            attrShouldExist('alert-dialog-content');
        });

        it('Title forwards attributes', () => {
            attrShouldExist('alert-dialog-title');
        });

        it('Description forwards attributes', () => {
            attrShouldExist('alert-dialog-description');
        });

        it('Cancel forwards attributes', () => {
            attrShouldExist('alert-dialog-cancel');
        });

        it('Action forwards attributes', () => {
            attrShouldExist('alert-dialog-action');
        });
    });

    // ── Popover (portaled + popper, deep chain) ─────────────

    describe('Popover', () => {
        it('Trigger forwards attributes', () => {
            attrShouldExist('popover-trigger');
        });

        it('Content forwards attributes', () => {
            attrShouldExist('popover-content');
        });

        it('Close forwards attributes', () => {
            attrShouldExist('popover-close');
        });

        it('Arrow forwards attributes', () => {
            attrShouldExist('popover-arrow');
        });
    });

    // ── Tooltip (portaled + popper, deep chain) ─────────────

    describe('Tooltip', () => {
        it('Trigger forwards attributes', () => {
            attrShouldExist('tooltip-trigger');
        });

        it('Content forwards attributes', () => {
            attrShouldExist('tooltip-content');
        });

        it('Arrow forwards attributes', () => {
            attrShouldExist('tooltip-arrow');
        });
    });

    // ── HoverCard (portaled + popper, deep chain) ───────────

    describe('HoverCard', () => {
        it('Trigger forwards attributes', () => {
            attrShouldExist('hover-card-trigger');
        });

        it('Content forwards attributes', () => {
            attrShouldExist('hover-card-content');
        });

        it('Arrow forwards attributes', () => {
            attrShouldExist('hover-card-arrow');
        });
    });

    // ── DropdownMenu (portaled + popper, deep chain) ────────

    describe('DropdownMenu', () => {
        it('Trigger forwards attributes', () => {
            attrShouldExist('dropdown-menu-trigger');
        });

        it('Content forwards attributes', () => {
            attrShouldExist('dropdown-menu-content');
        });

        it('Item forwards attributes', () => {
            attrShouldExist('dropdown-menu-item');
        });

        it('Separator forwards attributes', () => {
            attrShouldExist('dropdown-menu-separator');
        });

        it('Label forwards attributes', () => {
            attrShouldExist('dropdown-menu-label');
        });

        it('Arrow forwards attributes', () => {
            attrShouldExist('dropdown-menu-arrow');
        });
    });

    // ── ContextMenu (portaled + popper, deep chain) ─────────

    describe('ContextMenu', () => {
        it('Trigger forwards attributes', () => {
            attrShouldExist('context-menu-trigger');
        });

        it('Content forwards attributes', () => {
            attrShouldExist('context-menu-content');
        });

        it('Item forwards attributes', () => {
            attrShouldExist('context-menu-item');
        });
    });

    // ── Select (portaled + popper, deep chain) ──────────────

    describe('Select', () => {
        it('Trigger forwards attributes', () => {
            attrShouldExist('select-trigger');
        });

        it('Value forwards attributes', () => {
            attrShouldExist('select-value');
        });

        it('Content forwards attributes', () => {
            attrShouldExist('select-content');
        });

        it('Viewport forwards attributes', () => {
            attrShouldExist('select-viewport');
        });

        it('Item forwards attributes', () => {
            attrShouldExist('select-item');
        });

        it('ItemText forwards attributes', () => {
            attrShouldExist('select-item-text');
        });
    });

    // ── NavigationMenu (viewport rendering, deep chain) ─────

    describe('NavigationMenu', () => {
        it('Root forwards attributes', () => {
            attrShouldExist('nav-menu-root');
        });

        it('List forwards attributes', () => {
            attrShouldExist('nav-menu-list');
        });

        it('Item forwards attributes', () => {
            attrShouldExist('nav-menu-item');
        });

        it('Trigger forwards attributes', () => {
            attrShouldExist('nav-menu-trigger');
        });

        it('Content forwards attributes', () => {
            attrShouldExist('nav-menu-content');
        });

        it('Link forwards attributes', () => {
            attrShouldExist('nav-menu-link');
        });

        // NavigationMenu.Viewport is empty when no content is active —
        // its data-attr-test appears once content opens via the viewport.
        it.skip('Viewport forwards attributes', () => {
            attrShouldExist('nav-menu-viewport');
        });
    });

    // ── Menubar (portaled + popper, deep chain) ─────────────

    describe('Menubar', () => {
        it('Root forwards attributes', () => {
            attrShouldExist('menubar-root');
        });

        it('Trigger forwards attributes', () => {
            attrShouldExist('menubar-trigger');
        });

        it('Content forwards attributes', () => {
            attrShouldExist('menubar-content');
        });

        it('Item forwards attributes', () => {
            attrShouldExist('menubar-item');
        });
    });
});
