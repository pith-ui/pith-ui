import * as NavigationMenu from '@radix-ui/react-navigation-menu';
import '../../../shared/navigation-menu.css';

export default function NavigationMenuPage() {
    return (
        <>
            <NavigationMenu.Root className="nav-root" data-testid="nav-root" delayDuration={0} skipDelayDuration={0}>
                <NavigationMenu.List className="nav-list">
                    <NavigationMenu.Item className="nav-item" value="products">
                        <NavigationMenu.Trigger className="nav-trigger">Products</NavigationMenu.Trigger>
                        <NavigationMenu.Content className="nav-content nav-content-products" data-testid="products-content" style={{gridTemplateColumns: '1fr 1fr'}}>
                            <div className="nav-content-group" data-testid="products-featured">
                                <h3 className="nav-group-heading">Featured</h3>
                                <ul className="nav-content-list">
                                    <li>
                                        <NavigationMenu.Link className="nav-content-link" href="#">
                                            Product A
                                        </NavigationMenu.Link>
                                    </li>
                                    <li>
                                        <NavigationMenu.Link className="nav-content-link" href="#">
                                            Product B
                                        </NavigationMenu.Link>
                                    </li>
                                </ul>
                            </div>
                            <div className="nav-content-group" data-testid="products-all">
                                <h3 className="nav-group-heading">All Products</h3>
                                <ul className="nav-content-list">
                                    <li>
                                        <NavigationMenu.Link className="nav-content-link" href="#">
                                            Product C
                                        </NavigationMenu.Link>
                                    </li>
                                </ul>
                            </div>
                        </NavigationMenu.Content>
                    </NavigationMenu.Item>

                    <NavigationMenu.Item className="nav-item" value="resources">
                        <NavigationMenu.Trigger className="nav-trigger">Resources</NavigationMenu.Trigger>
                        <NavigationMenu.Content className="nav-content nav-content-resources" data-testid="resources-content">
                            <ul className="nav-content-list">
                                <li>
                                    <NavigationMenu.Link className="nav-content-link" href="#">
                                        Blog
                                    </NavigationMenu.Link>
                                </li>
                                <li>
                                    <NavigationMenu.Link className="nav-content-link" href="#">
                                        Docs
                                    </NavigationMenu.Link>
                                </li>
                            </ul>
                        </NavigationMenu.Content>
                    </NavigationMenu.Item>

                    <NavigationMenu.Item className="nav-item">
                        <NavigationMenu.Link className="nav-link-direct" href="#" active>
                            About
                        </NavigationMenu.Link>
                    </NavigationMenu.Item>

                    <NavigationMenu.Indicator className="nav-indicator" data-testid="nav-indicator" />
                </NavigationMenu.List>

                <NavigationMenu.Viewport className="nav-viewport" data-testid="nav-viewport" />
            </NavigationMenu.Root>

            <br />
            <br />

            <button data-testid="outside-element">outside</button>
        </>
    );
}
