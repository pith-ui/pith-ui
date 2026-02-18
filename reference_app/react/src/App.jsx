import {Routes, Route, Link} from 'react-router-dom';
import Dialog from './pages/Dialog';

function Index() {
    return (
        <div>
            <h1>Radix Reference App (React)</h1>
            <p>Add component pages as needed. Each route maps to a Radix primitive test fixture.</p>
            <nav>
                <ul>
                    <li>
                        <Link to="/dialog">Dialog</Link>
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
            <Route path="/dialog" element={<Dialog />} />
        </Routes>
    );
}
