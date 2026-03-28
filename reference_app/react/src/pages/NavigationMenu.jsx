import {useState} from 'react';
import * as NavigationMenu from '@radix-ui/react-navigation-menu';
import '../../../shared/navigation-menu.css';

export default function NavigationMenuPage() {
    const [controlledValue, setControlledValue] = useState('');

    return (
        <>
            <NavigationMenu.Root className="nav-root" data-testid="nav-root" aria-label="Main" delayDuration={0} skipDelayDuration={0}>
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

            <hr />

            <h3>Controlled</h3>
            <button data-testid="set-products" onClick={() => setControlledValue('c-products')}>
                open products
            </button>
            <button data-testid="set-resources" onClick={() => setControlledValue('c-resources')}>
                open resources
            </button>
            <button data-testid="close-all" onClick={() => setControlledValue('')}>
                close all
            </button>
            <span data-testid="controlled-nav-value">{controlledValue || '(none)'}</span>

            <NavigationMenu.Root
                className="nav-root"
                data-testid="controlled-nav-root"
                aria-label="Controlled"
                value={controlledValue}
                onValueChange={setControlledValue}
                delayDuration={0}
                skipDelayDuration={0}
            >
                <NavigationMenu.List className="nav-list">
                    <NavigationMenu.Item className="nav-item" value="c-products">
                        <NavigationMenu.Trigger className="nav-trigger" data-testid="controlled-products-trigger">
                            CProducts
                        </NavigationMenu.Trigger>
                        <NavigationMenu.Content className="nav-content" data-testid="controlled-products-content">
                            <ul className="nav-content-list">
                                <li>
                                    <NavigationMenu.Link className="nav-content-link" href="#">
                                        CProduct A
                                    </NavigationMenu.Link>
                                </li>
                            </ul>
                        </NavigationMenu.Content>
                    </NavigationMenu.Item>

                    <NavigationMenu.Item className="nav-item" value="c-resources">
                        <NavigationMenu.Trigger className="nav-trigger" data-testid="controlled-resources-trigger">
                            CResources
                        </NavigationMenu.Trigger>
                        <NavigationMenu.Content className="nav-content" data-testid="controlled-resources-content">
                            <ul className="nav-content-list">
                                <li>
                                    <NavigationMenu.Link className="nav-content-link" href="#">
                                        CBlog
                                    </NavigationMenu.Link>
                                </li>
                            </ul>
                        </NavigationMenu.Content>
                    </NavigationMenu.Item>
                </NavigationMenu.List>

                <NavigationMenu.Viewport className="nav-viewport" data-testid="controlled-nav-viewport" />
            </NavigationMenu.Root>
        </>
    );
}
