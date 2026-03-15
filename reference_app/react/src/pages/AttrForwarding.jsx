/**
 * Attribute Forwarding Test Fixture
 *
 * Renders every component part with data-attr-test="<part-name>" to verify
 * that user-provided spread attributes reach the final DOM element.
 *
 * Components that need to be "open" to render content parts are opened
 * via defaultOpen or open={true}.
 */
import * as Accordion from '@radix-ui/react-accordion';
import * as AlertDialog from '@radix-ui/react-alert-dialog';
import * as Checkbox from '@radix-ui/react-checkbox';
import * as Collapsible from '@radix-ui/react-collapsible';
import * as ContextMenu from '@radix-ui/react-context-menu';
import * as Dialog from '@radix-ui/react-dialog';
import * as DropdownMenu from '@radix-ui/react-dropdown-menu';
import * as HoverCard from '@radix-ui/react-hover-card';
import * as Label from '@radix-ui/react-label';
import * as Menubar from '@radix-ui/react-menubar';
import * as NavigationMenu from '@radix-ui/react-navigation-menu';
import * as Popover from '@radix-ui/react-popover';
import * as Progress from '@radix-ui/react-progress';
import * as RadioGroup from '@radix-ui/react-radio-group';
import * as ScrollArea from '@radix-ui/react-scroll-area';
import * as Select from '@radix-ui/react-select';
import * as Separator from '@radix-ui/react-separator';
import * as Slider from '@radix-ui/react-slider';
import * as Switch from '@radix-ui/react-switch';
import * as Tabs from '@radix-ui/react-tabs';
import * as Toggle from '@radix-ui/react-toggle';
import * as Toolbar from '@radix-ui/react-toolbar';
import * as Tooltip from '@radix-ui/react-tooltip';

export default function AttrForwardingPage() {
    return (
        <div style={{padding: '20px'}}>
            <h2>Attribute Forwarding Test Fixture</h2>

            {/* ── Simple Components ──────────────────────── */}
            <section>
                <h3>Simple</h3>
                <Separator.Root data-attr-test="separator" />
                <Label.Root data-attr-test="label">Label</Label.Root>
                <Progress.Root data-attr-test="progress-root" value={50}>
                    <Progress.Indicator data-attr-test="progress-indicator" />
                </Progress.Root>
                <Toggle.Root data-attr-test="toggle">Toggle</Toggle.Root>
                <Switch.Root data-attr-test="switch-root" />
                <Checkbox.Root data-attr-test="checkbox-root" />
            </section>

            {/* ── Accordion ──────────────────────────────── */}
            <section>
                <h3>Accordion</h3>
                <Accordion.Root type="single" defaultValue="item1" data-attr-test="accordion-root">
                    <Accordion.Item value="item1" data-attr-test="accordion-item">
                        <Accordion.Header data-attr-test="accordion-header">
                            <Accordion.Trigger data-attr-test="accordion-trigger">Open</Accordion.Trigger>
                        </Accordion.Header>
                        <Accordion.Content data-attr-test="accordion-content" forceMount>
                            Content
                        </Accordion.Content>
                    </Accordion.Item>
                </Accordion.Root>
            </section>

            {/* ── Collapsible ────────────────────────────── */}
            <section>
                <h3>Collapsible</h3>
                <Collapsible.Root defaultOpen data-attr-test="collapsible-root">
                    <Collapsible.Trigger data-attr-test="collapsible-trigger">Toggle</Collapsible.Trigger>
                    <Collapsible.Content data-attr-test="collapsible-content">Content</Collapsible.Content>
                </Collapsible.Root>
            </section>

            {/* ── Tabs ───────────────────────────────────── */}
            <section>
                <h3>Tabs</h3>
                <Tabs.Root defaultValue="tab1" data-attr-test="tabs-root">
                    <Tabs.List data-attr-test="tabs-list">
                        <Tabs.Trigger value="tab1" data-attr-test="tabs-trigger">Tab 1</Tabs.Trigger>
                    </Tabs.List>
                    <Tabs.Content value="tab1" data-attr-test="tabs-content">Content</Tabs.Content>
                </Tabs.Root>
            </section>

            {/* ── RadioGroup ─────────────────────────────── */}
            <section>
                <h3>RadioGroup</h3>
                <RadioGroup.Root defaultValue="a" data-attr-test="radio-group-root">
                    <RadioGroup.Item value="a" data-attr-test="radio-group-item">
                        <RadioGroup.Indicator data-attr-test="radio-group-indicator" />
                    </RadioGroup.Item>
                </RadioGroup.Root>
            </section>

            {/* ── Slider ─────────────────────────────────── */}
            <section>
                <h3>Slider</h3>
                <Slider.Root defaultValue={[50]} data-attr-test="slider-root">
                    <Slider.Track data-attr-test="slider-track">
                        <Slider.Range data-attr-test="slider-range" />
                    </Slider.Track>
                    <Slider.Thumb data-attr-test="slider-thumb" />
                </Slider.Root>
            </section>

            {/* ── Toolbar ────────────────────────────────── */}
            <section>
                <h3>Toolbar</h3>
                <Toolbar.Root data-attr-test="toolbar-root">
                    <Toolbar.Button data-attr-test="toolbar-button">Btn</Toolbar.Button>
                    <Toolbar.Separator data-attr-test="toolbar-separator" />
                    <Toolbar.Link data-attr-test="toolbar-link" href="#">Link</Toolbar.Link>
                </Toolbar.Root>
            </section>

            {/* ── ScrollArea ─────────────────────────────── */}
            <section>
                <h3>ScrollArea</h3>
                <ScrollArea.Root data-attr-test="scroll-area-root" style={{height: 50, width: 200}}>
                    <ScrollArea.Viewport data-attr-test="scroll-area-viewport">
                        <div style={{height: 500, width: 500}}>Tall and wide content to force scrollbars</div>
                    </ScrollArea.Viewport>
                    <ScrollArea.Scrollbar orientation="vertical" data-attr-test="scroll-area-scrollbar" forceMount>
                        <ScrollArea.Thumb data-attr-test="scroll-area-thumb" />
                    </ScrollArea.Scrollbar>
                </ScrollArea.Root>
            </section>

            {/* ── Dialog (forceMount all parts) ───────────── */}
            <section>
                <h3>Dialog</h3>
                <Dialog.Root>
                    <Dialog.Trigger data-attr-test="dialog-trigger">Open</Dialog.Trigger>
                    <Dialog.Portal forceMount>
                        <Dialog.Overlay data-attr-test="dialog-overlay" forceMount />
                        <Dialog.Content data-attr-test="dialog-content" forceMount>
                            <Dialog.Title data-attr-test="dialog-title">Title</Dialog.Title>
                            <Dialog.Description data-attr-test="dialog-description">Desc</Dialog.Description>
                            <Dialog.Close data-attr-test="dialog-close">Close</Dialog.Close>
                        </Dialog.Content>
                    </Dialog.Portal>
                </Dialog.Root>
            </section>

            {/* ── AlertDialog (forceMount all parts) ─────── */}
            <section>
                <h3>AlertDialog</h3>
                <AlertDialog.Root>
                    <AlertDialog.Trigger data-attr-test="alert-dialog-trigger">Open</AlertDialog.Trigger>
                    <AlertDialog.Portal forceMount>
                        <AlertDialog.Overlay data-attr-test="alert-dialog-overlay" forceMount />
                        <AlertDialog.Content data-attr-test="alert-dialog-content" forceMount>
                            <AlertDialog.Title data-attr-test="alert-dialog-title">Title</AlertDialog.Title>
                            <AlertDialog.Description data-attr-test="alert-dialog-description">Desc</AlertDialog.Description>
                            <AlertDialog.Cancel data-attr-test="alert-dialog-cancel">Cancel</AlertDialog.Cancel>
                            <AlertDialog.Action data-attr-test="alert-dialog-action">Ok</AlertDialog.Action>
                        </AlertDialog.Content>
                    </AlertDialog.Portal>
                </AlertDialog.Root>
            </section>

            {/* ── Popover (forceMount to avoid positioning issues) ── */}
            <section>
                <h3>Popover</h3>
                <Popover.Root>
                    <Popover.Trigger data-attr-test="popover-trigger">Open</Popover.Trigger>
                    <Popover.Portal forceMount>
                        <Popover.Content data-attr-test="popover-content" forceMount sideOffset={5}>
                            Content
                            <Popover.Close data-attr-test="popover-close">X</Popover.Close>
                            <Popover.Arrow data-attr-test="popover-arrow" />
                        </Popover.Content>
                    </Popover.Portal>
                </Popover.Root>
            </section>

            {/* ── Tooltip (forceMount) ────────────────────── */}
            <section>
                <h3>Tooltip</h3>
                <Tooltip.Provider>
                    <Tooltip.Root>
                        <Tooltip.Trigger data-attr-test="tooltip-trigger">Hover</Tooltip.Trigger>
                        <Tooltip.Portal forceMount>
                            <Tooltip.Content data-attr-test="tooltip-content" forceMount sideOffset={5}>
                                Tip
                                <Tooltip.Arrow data-attr-test="tooltip-arrow" />
                            </Tooltip.Content>
                        </Tooltip.Portal>
                    </Tooltip.Root>
                </Tooltip.Provider>
            </section>

            {/* ── HoverCard (forceMount) ─────────────────── */}
            <section>
                <h3>HoverCard</h3>
                <HoverCard.Root>
                    <HoverCard.Trigger data-attr-test="hover-card-trigger">Hover</HoverCard.Trigger>
                    <HoverCard.Portal forceMount>
                        <HoverCard.Content data-attr-test="hover-card-content" forceMount sideOffset={5}>
                            Card
                            <HoverCard.Arrow data-attr-test="hover-card-arrow" />
                        </HoverCard.Content>
                    </HoverCard.Portal>
                </HoverCard.Root>
            </section>

            {/* ── DropdownMenu (forceMount) ────────────────── */}
            <section>
                <h3>DropdownMenu</h3>
                <DropdownMenu.Root modal={false}>
                    <DropdownMenu.Trigger data-attr-test="dropdown-menu-trigger">Open</DropdownMenu.Trigger>
                    <DropdownMenu.Portal forceMount>
                        <DropdownMenu.Content data-attr-test="dropdown-menu-content" forceMount sideOffset={5}>
                            <DropdownMenu.Label data-attr-test="dropdown-menu-label">Actions</DropdownMenu.Label>
                            <DropdownMenu.Item data-attr-test="dropdown-menu-item">Item</DropdownMenu.Item>
                            <DropdownMenu.Separator data-attr-test="dropdown-menu-separator" />
                            <DropdownMenu.Arrow data-attr-test="dropdown-menu-arrow" />
                        </DropdownMenu.Content>
                    </DropdownMenu.Portal>
                </DropdownMenu.Root>
            </section>

            {/* ── ContextMenu (forceMount content) ─────────── */}
            <section>
                <h3>ContextMenu</h3>
                <ContextMenu.Root modal={false}>
                    <ContextMenu.Trigger data-attr-test="context-menu-trigger" style={{padding: '20px', border: '1px dashed gray'}}>
                        Right-click here
                    </ContextMenu.Trigger>
                    <ContextMenu.Portal forceMount>
                        <ContextMenu.Content data-attr-test="context-menu-content" forceMount>
                            <ContextMenu.Item data-attr-test="context-menu-item">Item</ContextMenu.Item>
                        </ContextMenu.Content>
                    </ContextMenu.Portal>
                </ContextMenu.Root>
            </section>

            {/* ── Select (open) ──────────────────────────── */}
            <section>
                <h3>Select</h3>
                <Select.Root defaultOpen defaultValue="a">
                    <Select.Trigger data-attr-test="select-trigger">
                        <Select.Value data-attr-test="select-value" />
                    </Select.Trigger>
                    <Select.Portal>
                        <Select.Content data-attr-test="select-content" position="popper" sideOffset={5}>
                            <Select.Viewport data-attr-test="select-viewport">
                                <Select.Item value="a" data-attr-test="select-item">
                                    <Select.ItemText data-attr-test="select-item-text">Option A</Select.ItemText>
                                </Select.Item>
                                <Select.Item value="b">
                                    <Select.ItemText>Option B</Select.ItemText>
                                </Select.Item>
                            </Select.Viewport>
                        </Select.Content>
                    </Select.Portal>
                </Select.Root>
            </section>

            {/* ── NavigationMenu (defaultValue to force open) ── */}
            <section>
                <h3>NavigationMenu</h3>
                <NavigationMenu.Root data-attr-test="nav-menu-root" defaultValue="products" delayDuration={0} skipDelayDuration={0}>
                    <NavigationMenu.List data-attr-test="nav-menu-list">
                        <NavigationMenu.Item data-attr-test="nav-menu-item" value="products">
                            <NavigationMenu.Trigger data-attr-test="nav-menu-trigger">Products</NavigationMenu.Trigger>
                            <NavigationMenu.Content data-attr-test="nav-menu-content" forceMount>
                                <NavigationMenu.Link data-attr-test="nav-menu-link" href="#">Link</NavigationMenu.Link>
                            </NavigationMenu.Content>
                        </NavigationMenu.Item>
                    </NavigationMenu.List>
                    <NavigationMenu.Viewport data-attr-test="nav-menu-viewport" />
                </NavigationMenu.Root>
            </section>

            {/* ── Menubar (forceMount content) ───────────── */}
            <section>
                <h3>Menubar</h3>
                <Menubar.Root data-attr-test="menubar-root">
                    <Menubar.Menu>
                        <Menubar.Trigger data-attr-test="menubar-trigger">File</Menubar.Trigger>
                        <Menubar.Portal forceMount>
                            <Menubar.Content data-attr-test="menubar-content" forceMount sideOffset={5}>
                                <Menubar.Item data-attr-test="menubar-item">New</Menubar.Item>
                            </Menubar.Content>
                        </Menubar.Portal>
                    </Menubar.Menu>
                </Menubar.Root>
            </section>
        </div>
    );
}
