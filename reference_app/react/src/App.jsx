import {Routes, Route, Link} from 'react-router-dom';
import Accordion from './pages/Accordion';
import AlertDialog from './pages/AlertDialog';
import Avatar from './pages/Avatar';
import Checkbox from './pages/Checkbox';
import Collapsible from './pages/Collapsible';
import ContextMenu from './pages/ContextMenu';
import Dialog from './pages/Dialog';
import DropdownMenu from './pages/DropdownMenu';
import Form from './pages/Form';
import Menu from './pages/Menu';
import HoverCard from './pages/HoverCard';
import NavigationMenu from './pages/NavigationMenu';
import Popover from './pages/Popover';
import Progress from './pages/Progress';
import RadioGroup from './pages/RadioGroup';
import ScrollArea from './pages/ScrollArea';
import Select, {SelectForcedOpenPage} from './pages/Select';
import Separator from './pages/Separator';
import Slider from './pages/Slider';
import Switch from './pages/Switch';
import Tabs from './pages/Tabs';
import Toast from './pages/Toast';
import Toggle from './pages/Toggle';
import ToggleGroup from './pages/ToggleGroup';
import Toolbar from './pages/Toolbar';
import Tooltip from './pages/Tooltip';

function Index() {
    return (
        <div>
            <h1>Radix Reference App (React)</h1>
            <p>Add component pages as needed. Each route maps to a Radix primitive test fixture.</p>
            <nav>
                <ul>
                    <li>
                        <Link to="/accordion">Accordion</Link>
                    </li>
                    <li>
                        <Link to="/alert-dialog">Alert Dialog</Link>
                    </li>
                    <li>
                        <Link to="/avatar">Avatar</Link>
                    </li>
                    <li>
                        <Link to="/checkbox">Checkbox</Link>
                    </li>
                    <li>
                        <Link to="/collapsible">Collapsible</Link>
                    </li>
                    <li>
                        <Link to="/context-menu">Context Menu</Link>
                    </li>
                    <li>
                        <Link to="/dialog">Dialog</Link>
                    </li>
                    <li>
                        <Link to="/dropdown-menu">Dropdown Menu</Link>
                    </li>
                    <li>
                        <Link to="/form">Form</Link>
                    </li>
                    <li>
                        <Link to="/hover-card">Hover Card</Link>
                    </li>
                    <li>
                        <Link to="/menu">Menu</Link>
                    </li>
                    <li>
                        <Link to="/navigation-menu">Navigation Menu</Link>
                    </li>
                    <li>
                        <Link to="/popover">Popover</Link>
                    </li>
                    <li>
                        <Link to="/progress">Progress</Link>
                    </li>
                    <li>
                        <Link to="/radio-group">Radio Group</Link>
                    </li>
                    <li>
                        <Link to="/scroll-area">Scroll Area</Link>
                    </li>
                    <li>
                        <Link to="/select">Select</Link>
                    </li>
                    <li>
                        <Link to="/separator">Separator</Link>
                    </li>
                    <li>
                        <Link to="/slider">Slider</Link>
                    </li>
                    <li>
                        <Link to="/switch">Switch</Link>
                    </li>
                    <li>
                        <Link to="/tabs">Tabs</Link>
                    </li>
                    <li>
                        <Link to="/toast">Toast</Link>
                    </li>
                    <li>
                        <Link to="/toggle">Toggle</Link>
                    </li>
                    <li>
                        <Link to="/toggle-group">Toggle Group</Link>
                    </li>
                    <li>
                        <Link to="/toolbar">Toolbar</Link>
                    </li>
                    <li>
                        <Link to="/tooltip">Tooltip</Link>
                    </li>
                </ul>
            </nav>
        </div>
    );
}

export default function App() {
    return (
        <Routes>
            <Route path="/" element={<Index />} />
            <Route path="/accordion" element={<Accordion />} />
            <Route path="/alert-dialog" element={<AlertDialog />} />
            <Route path="/avatar" element={<Avatar />} />
            <Route path="/checkbox" element={<Checkbox />} />
            <Route path="/collapsible" element={<Collapsible />} />
            <Route path="/context-menu" element={<ContextMenu />} />
            <Route path="/dialog" element={<Dialog />} />
            <Route path="/dropdown-menu" element={<DropdownMenu />} />
            <Route path="/form" element={<Form />} />
            <Route path="/menu" element={<Menu />} />
            <Route path="/hover-card" element={<HoverCard />} />
            <Route path="/navigation-menu" element={<NavigationMenu />} />
            <Route path="/popover" element={<Popover />} />
            <Route path="/progress" element={<Progress />} />
            <Route path="/radio-group" element={<RadioGroup />} />
            <Route path="/scroll-area" element={<ScrollArea />} />
            <Route path="/select" element={<Select />} />
            <Route path="/select/forced-open" element={<SelectForcedOpenPage />} />
            <Route path="/separator" element={<Separator />} />
            <Route path="/slider" element={<Slider />} />
            <Route path="/switch" element={<Switch />} />
            <Route path="/tabs" element={<Tabs />} />
            <Route path="/toast" element={<Toast />} />
            <Route path="/toggle" element={<Toggle />} />
            <Route path="/toggle-group" element={<ToggleGroup />} />
            <Route path="/toolbar" element={<Toolbar />} />
            <Route path="/tooltip" element={<Tooltip />} />
        </Routes>
    );
}
