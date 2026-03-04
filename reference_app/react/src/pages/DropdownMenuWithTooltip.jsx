import * as DropdownMenu from '@radix-ui/react-dropdown-menu';
import * as Tooltip from '@radix-ui/react-tooltip';
import '../../../shared/dropdown-menu.css';

export default function DropdownMenuWithTooltipPage() {
    return (
        <>
            <DropdownMenu.Root>
                <Tooltip.Provider>
                    <Tooltip.Root>
                        <Tooltip.Trigger asChild>
                            <DropdownMenu.Trigger className="dropdown-trigger">open</DropdownMenu.Trigger>
                        </Tooltip.Trigger>
                        <Tooltip.Content>Tooltip content</Tooltip.Content>
                    </Tooltip.Root>
                </Tooltip.Provider>
                <DropdownMenu.Portal>
                    <DropdownMenu.Content className="dropdown-content" sideOffset={5}>
                        <DropdownMenu.Item className="dropdown-item" onSelect={() => {}}>
                            Item 1
                        </DropdownMenu.Item>
                        <DropdownMenu.Item className="dropdown-item" onSelect={() => {}}>
                            Item 2
                        </DropdownMenu.Item>
                        <DropdownMenu.Item className="dropdown-item" onSelect={() => {}}>
                            Item 3
                        </DropdownMenu.Item>
                    </DropdownMenu.Content>
                </DropdownMenu.Portal>
            </DropdownMenu.Root>

            <br /><br />
            <button data-testid="outside-button">outside</button>
        </>
    );
}
