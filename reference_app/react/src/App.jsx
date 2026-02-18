import {Routes, Route, Link} from 'react-router-dom';

function Index() {
    return (
        <div>
            <h1>Radix Reference App (React)</h1>
            <p>Add component pages as needed. Each route maps to a Radix primitive test fixture.</p>
            <nav>
                <ul>{/* Add links here as pages are created */}</ul>
            </nav>
        </div>
    );
}

export default function App() {
    return (
        <Routes>
            <Route path="/" element={<Index />} />
            {/* Add component routes here, e.g.:
                <Route path="/dialog" element={<Dialog />} />
                <Route path="/form" element={<Form />} />
            */}
        </Routes>
    );
}
