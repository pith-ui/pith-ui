use leptos::prelude::*;
use radix_leptos_primitives::accordion::*;
use radix_leptos_primitives::alert_dialog::*;
use radix_leptos_primitives::checkbox::*;
use radix_leptos_primitives::collapsible::*;
use radix_leptos_primitives::context_menu::*;
use radix_leptos_primitives::dialog::*;
use radix_leptos_primitives::dropdown_menu::*;
use radix_leptos_primitives::hover_card::*;
use radix_leptos_primitives::label::*;
use radix_leptos_primitives::menubar::*;
use radix_leptos_primitives::navigation_menu::*;
use radix_leptos_primitives::popover::*;
use radix_leptos_primitives::progress::*;
use radix_leptos_primitives::radio_group::*;
use radix_leptos_primitives::scroll_area::{
    self, ScrollArea, ScrollAreaScrollbar, ScrollAreaThumb, ScrollAreaViewport,
};
use radix_leptos_primitives::select::*;
use radix_leptos_primitives::separator::*;
use radix_leptos_primitives::slider::*;
use radix_leptos_primitives::switch::*;
use radix_leptos_primitives::tabs::*;
use radix_leptos_primitives::toggle::*;
use radix_leptos_primitives::toolbar::*;
use radix_leptos_primitives::tooltip::*;

#[component]
pub fn AttrForwardingPage() -> impl IntoView {
    view! {
        <div style="padding: 20px;">
            <h2>"Attribute Forwarding Test Fixture"</h2>

            // ── Simple Components ──────────────────────────
            <section>
                <h3>"Simple"</h3>
                <Separator attr:data-attr-test="separator" />
                <Label attr:data-attr-test="label">"Label"</Label>
                <Progress attr:data-attr-test="progress-root" value=50.0>
                    <ProgressIndicator attr:data-attr-test="progress-indicator" />
                </Progress>
                <Toggle attr:data-attr-test="toggle">"Toggle"</Toggle>
                <Switch attr:data-attr-test="switch-root">{""}</Switch>
                <Checkbox attr:data-attr-test="checkbox-root">{""}</Checkbox>
            </section>

            // ── Accordion ──────────────────────────────────
            <section>
                <h3>"Accordion"</h3>
                <Accordion r#type=AccordionType::Single default_value="item1" attr:data-attr-test="accordion-root">
                    <AccordionItem value="item1".to_string() attr:data-attr-test="accordion-item">
                        <AccordionHeader attr:data-attr-test="accordion-header">
                            <AccordionTrigger attr:data-attr-test="accordion-trigger">"Open"</AccordionTrigger>
                        </AccordionHeader>
                        <AccordionContent attr:data-attr-test="accordion-content" force_mount=true>
                            "Content"
                        </AccordionContent>
                    </AccordionItem>
                </Accordion>
            </section>

            // ── Collapsible ────────────────────────────────
            <section>
                <h3>"Collapsible"</h3>
                <Collapsible default_open=true attr:data-attr-test="collapsible-root">
                    <CollapsibleTrigger attr:data-attr-test="collapsible-trigger">"Toggle"</CollapsibleTrigger>
                    <CollapsibleContent attr:data-attr-test="collapsible-content">"Content"</CollapsibleContent>
                </Collapsible>
            </section>

            // ── Tabs ───────────────────────────────────────
            <section>
                <h3>"Tabs"</h3>
                <Tabs default_value="tab1".to_string() attr:data-attr-test="tabs-root">
                    <TabsList attr:data-attr-test="tabs-list">
                        <TabsTrigger value="tab1".to_string() attr:data-attr-test="tabs-trigger">"Tab 1"</TabsTrigger>
                    </TabsList>
                    <TabsContent value="tab1".to_string() attr:data-attr-test="tabs-content">"Content"</TabsContent>
                </Tabs>
            </section>

            // ── RadioGroup ─────────────────────────────────
            <section>
                <h3>"RadioGroup"</h3>
                <RadioGroup default_value="a" attr:data-attr-test="radio-group-root">
                    <RadioGroupItem value="a" attr:data-attr-test="radio-group-item">
                        <RadioGroupIndicator attr:data-attr-test="radio-group-indicator" />
                    </RadioGroupItem>
                </RadioGroup>
            </section>

            // ── Slider ─────────────────────────────────────
            <section>
                <h3>"Slider"</h3>
                <Slider default_value=vec![50.0] attr:data-attr-test="slider-root">
                    <SliderTrack attr:data-attr-test="slider-track">
                        <SliderRange attr:data-attr-test="slider-range" />
                    </SliderTrack>
                    <SliderThumb attr:data-attr-test="slider-thumb" />
                </Slider>
            </section>

            // ── Toolbar ────────────────────────────────────
            <section>
                <h3>"Toolbar"</h3>
                <Toolbar attr:data-attr-test="toolbar-root">
                    <ToolbarButton attr:data-attr-test="toolbar-button">"Btn"</ToolbarButton>
                    <ToolbarSeparator attr:data-attr-test="toolbar-separator" />
                    <ToolbarLink attr:data-attr-test="toolbar-link" attr:href="#">"Link"</ToolbarLink>
                </Toolbar>
            </section>

            // ── ScrollArea ─────────────────────────────────
            <section>
                <h3>"ScrollArea"</h3>
                <ScrollArea attr:data-attr-test="scroll-area-root" style:height="50px" style:overflow="hidden">
                    <ScrollAreaViewport attr:data-attr-test="scroll-area-viewport">
                        <div style="height: 200px;">"Tall content"</div>
                    </ScrollAreaViewport>
                    <ScrollAreaScrollbar orientation=scroll_area::Orientation::Vertical attr:data-attr-test="scroll-area-scrollbar">
                        <ScrollAreaThumb attr:data-attr-test="scroll-area-thumb">{""}</ScrollAreaThumb>
                    </ScrollAreaScrollbar>
                </ScrollArea>
            </section>

            // ── Dialog (open) ──────────────────────────────
            <section>
                <h3>"Dialog"</h3>
                <Dialog default_open=true>
                    <DialogTrigger attr:data-attr-test="dialog-trigger">"Open"</DialogTrigger>
                    <DialogPortal>
                        <DialogOverlay attr:data-attr-test="dialog-overlay" />
                        <DialogContent attr:data-attr-test="dialog-content">
                            <DialogTitle attr:data-attr-test="dialog-title">"Title"</DialogTitle>
                            <DialogDescription attr:data-attr-test="dialog-description">"Desc"</DialogDescription>
                            <DialogClose attr:data-attr-test="dialog-close">"Close"</DialogClose>
                        </DialogContent>
                    </DialogPortal>
                </Dialog>
            </section>

            // ── AlertDialog (open) ─────────────────────────
            <section>
                <h3>"AlertDialog"</h3>
                <AlertDialog default_open=true>
                    <AlertDialogTrigger attr:data-attr-test="alert-dialog-trigger">"Open"</AlertDialogTrigger>
                    <AlertDialogPortal>
                        <AlertDialogOverlay attr:data-attr-test="alert-dialog-overlay" />
                        <AlertDialogContent attr:data-attr-test="alert-dialog-content">
                            <AlertDialogTitle attr:data-attr-test="alert-dialog-title">"Title"</AlertDialogTitle>
                            <AlertDialogDescription attr:data-attr-test="alert-dialog-description">"Desc"</AlertDialogDescription>
                            <AlertDialogCancel attr:data-attr-test="alert-dialog-cancel">"Cancel"</AlertDialogCancel>
                            <AlertDialogAction attr:data-attr-test="alert-dialog-action">"Ok"</AlertDialogAction>
                        </AlertDialogContent>
                    </AlertDialogPortal>
                </AlertDialog>
            </section>

            // ── Popover (open) ─────────────────────────────
            <section>
                <h3>"Popover"</h3>
                <Popover default_open=true>
                    <PopoverTrigger attr:data-attr-test="popover-trigger">"Open"</PopoverTrigger>
                    <PopoverPortal>
                        <PopoverContent attr:data-attr-test="popover-content" side_offset=5.0>
                            "Content"
                            <PopoverClose attr:data-attr-test="popover-close">"X"</PopoverClose>
                            <PopoverArrow attr:data-attr-test="popover-arrow" />
                        </PopoverContent>
                    </PopoverPortal>
                </Popover>
            </section>

            // ── Tooltip (open) ─────────────────────────────
            <section>
                <h3>"Tooltip"</h3>
                <TooltipProvider>
                    <Tooltip default_open=true>
                        <TooltipTrigger attr:data-attr-test="tooltip-trigger">"Hover"</TooltipTrigger>
                        <TooltipPortal>
                            <TooltipContent attr:data-attr-test="tooltip-content" side_offset=5.0>
                                "Tip"
                                <TooltipArrow attr:data-attr-test="tooltip-arrow" />
                            </TooltipContent>
                        </TooltipPortal>
                    </Tooltip>
                </TooltipProvider>
            </section>

            // ── HoverCard (open) ───────────────────────────
            <section>
                <h3>"HoverCard"</h3>
                <HoverCard default_open=true>
                    <HoverCardTrigger attr:data-attr-test="hover-card-trigger">"Hover"</HoverCardTrigger>
                    <HoverCardPortal>
                        <HoverCardContent attr:data-attr-test="hover-card-content" side_offset=5.0>
                            "Card"
                            <HoverCardArrow attr:data-attr-test="hover-card-arrow" />
                        </HoverCardContent>
                    </HoverCardPortal>
                </HoverCard>
            </section>

            // ── DropdownMenu (open) ────────────────────────
            <section>
                <h3>"DropdownMenu"</h3>
                <DropdownMenu default_open=true>
                    <DropdownMenuTrigger attr:data-attr-test="dropdown-menu-trigger">"Open"</DropdownMenuTrigger>
                    <DropdownMenuPortal>
                        <DropdownMenuContent attr:data-attr-test="dropdown-menu-content" side_offset=5.0>
                            <DropdownMenuLabel attr:data-attr-test="dropdown-menu-label">"Actions"</DropdownMenuLabel>
                            <DropdownMenuItem attr:data-attr-test="dropdown-menu-item">"Item"</DropdownMenuItem>
                            <DropdownMenuSeparator attr:data-attr-test="dropdown-menu-separator" />
                            <DropdownMenuArrow attr:data-attr-test="dropdown-menu-arrow" />
                        </DropdownMenuContent>
                    </DropdownMenuPortal>
                </DropdownMenu>
            </section>

            // ── ContextMenu ────────────────────────────────
            <section>
                <h3>"ContextMenu"</h3>
                <ContextMenu>
                    <ContextMenuTrigger attr:data-attr-test="context-menu-trigger">
                        <div style="padding: 20px; border: 1px dashed gray;">"Right-click here"</div>
                    </ContextMenuTrigger>
                    <ContextMenuPortal>
                        <ContextMenuContent attr:data-attr-test="context-menu-content" force_mount=true style:visibility="hidden" style:position="fixed">
                            <ContextMenuItem attr:data-attr-test="context-menu-item">"Item"</ContextMenuItem>
                        </ContextMenuContent>
                    </ContextMenuPortal>
                </ContextMenu>
            </section>

            // ── Select (open) ──────────────────────────────
            <section>
                <h3>"Select"</h3>
                <Select default_open=true default_value="a">
                    <SelectTrigger attr:data-attr-test="select-trigger">
                        <SelectValue attr:data-attr-test="select-value" />
                    </SelectTrigger>
                    <SelectPortal>
                        <SelectContent attr:data-attr-test="select-content" position="popper" side_offset=5.0>
                            <SelectViewport attr:data-attr-test="select-viewport">
                                <SelectItem value="a" attr:data-attr-test="select-item">
                                    <SelectItemText attr:data-attr-test="select-item-text">"Option A"</SelectItemText>
                                </SelectItem>
                                <SelectItem value="b">
                                    <SelectItemText>"Option B"</SelectItemText>
                                </SelectItem>
                            </SelectViewport>
                        </SelectContent>
                    </SelectPortal>
                </Select>
            </section>

            // ── NavigationMenu ─────────────────────────────
            <section>
                <h3>"NavigationMenu"</h3>
                <NavigationMenu attr:data-attr-test="nav-menu-root" delay_duration=0.0 skip_delay_duration=0.0>
                    <NavigationMenuList attr:data-attr-test="nav-menu-list">
                        <NavigationMenuItem attr:data-attr-test="nav-menu-item" value="products".to_string()>
                            <NavigationMenuTrigger attr:data-attr-test="nav-menu-trigger">"Products"</NavigationMenuTrigger>
                            <NavigationMenuContent attr:data-attr-test="nav-menu-content">
                                <NavigationMenuLink attr:data-attr-test="nav-menu-link" attr:href="#">"Link"</NavigationMenuLink>
                            </NavigationMenuContent>
                        </NavigationMenuItem>
                    </NavigationMenuList>
                    <NavigationMenuViewport attr:data-attr-test="nav-menu-viewport" />
                </NavigationMenu>
            </section>

            // ── Menubar ────────────────────────────────────
            <section>
                <h3>"Menubar"</h3>
                <Menubar attr:data-attr-test="menubar-root">
                    <MenubarMenu>
                        <MenubarTrigger attr:data-attr-test="menubar-trigger">"File"</MenubarTrigger>
                        <MenubarPortal>
                            <MenubarContent attr:data-attr-test="menubar-content" side_offset=5.0>
                                <MenubarItem attr:data-attr-test="menubar-item">"New"</MenubarItem>
                            </MenubarContent>
                        </MenubarPortal>
                    </MenubarMenu>
                </Menubar>
            </section>
        </div>
    }
}
