import {useState, useRef} from 'react';
import * as Toast from '@radix-ui/react-toast';
import '../../../shared/toast.css';

export default function ToastPage() {
    const [open, setOpen] = useState(false);
    const [autoDismiss, setAutoDismiss] = useState(false);
    const [count, setCount] = useState(0);
    const timerRef = useRef(0);
    const [showUncontrolled, setShowUncontrolled] = useState(false);
    const [controlledOpen, setControlledOpen] = useState(false);
    const [multiCount, setMultiCount] = useState(0);

    function addToast() {
        setOpen(false);
        window.clearTimeout(timerRef.current);
        timerRef.current = window.setTimeout(() => {
            setCount((c) => c + 1);
            setOpen(true);
        }, 100);
    }

    return (
        <Toast.Provider duration={autoDismiss ? 2000 : 1000000} swipeDirection="right">
            <h2>Controlled</h2>
            <button onClick={addToast} data-testid="add-toast">
                Add toast
            </button>

            <br />
            <br />

            <label>
                <input
                    type="checkbox"
                    checked={autoDismiss}
                    onChange={(e) => setAutoDismiss(e.target.checked)}
                />{' '}
                auto-dismiss
            </label>

            <br />
            <br />

            <span data-testid="toast-count">{count}</span>

            <br />
            <br />

            <button data-testid="outside-button">outside</button>

            <Toast.Root className="toast-root" open={open} onOpenChange={setOpen}>
                <Toast.Title className="toast-title">Toast title</Toast.Title>
                <Toast.Description className="toast-description">Toast description</Toast.Description>
                <Toast.Action className="toast-action" altText="Undo the action" asChild>
                    <button>Undo</button>
                </Toast.Action>
                <Toast.Close className="toast-close" asChild>
                    <button>×</button>
                </Toast.Close>
            </Toast.Root>

            <h2>Uncontrolled</h2>
            <button
                data-testid="show-uncontrolled"
                onClick={() => setShowUncontrolled((v) => !v)}
            >
                {showUncontrolled ? 'Hide uncontrolled' : 'Show uncontrolled'}
            </button>

            {showUncontrolled && <UncontrolledToast />}

            <h2>Controlled Mode</h2>
            <label>
                <input
                    type="checkbox"
                    checked={controlledOpen}
                    onChange={(e) => setControlledOpen(e.target.checked)}
                />{' '}
                open controlled toast
            </label>

            <Toast.Root className="toast-root" open={controlledOpen} onOpenChange={setControlledOpen} duration={1000000}>
                <Toast.Title className="toast-title">Controlled toast title</Toast.Title>
                <Toast.Description className="toast-description">Controlled toast description</Toast.Description>
                <Toast.Close className="toast-close" asChild>
                    <button data-testid="controlled-toast-close">Close controlled</button>
                </Toast.Close>
            </Toast.Root>

            <h2>Multi-toast tab order</h2>
            <button data-testid="add-multi-toast" onClick={() => setMultiCount((c) => c + 1)}>
                Add multi toast
            </button>
            <button data-testid="before-viewport">Before viewport</button>

            {[...Array(multiCount)].map((_, index) => {
                const id = index + 1;
                return (
                    <Toast.Root key={index} className="toast-root" open duration={1000000} data-testid={`multi-toast-${id}`}>
                        <Toast.Title className="toast-title">Multi toast {id}</Toast.Title>
                        <Toast.Description className="toast-description">Description {id}</Toast.Description>
                        <Toast.Action altText={`Action for toast ${id}`} asChild>
                            <button data-testid={`multi-action-${id}`}>Action {id}</button>
                        </Toast.Action>
                        <Toast.Close asChild>
                            <button data-testid={`multi-close-${id}`}>Close {id}</button>
                        </Toast.Close>
                    </Toast.Root>
                );
            })}

            <Toast.Viewport className="toast-viewport" data-testid="toast-viewport" label="Notifications" />
            <button data-testid="after-viewport">After viewport</button>
        </Toast.Provider>
    );
}

function UncontrolledToast() {
    return (
        <Toast.Root className="toast-root" duration={1000000}>
            <Toast.Title className="toast-title">Uncontrolled toast</Toast.Title>
            <Toast.Description className="toast-description">
                This toast has no open prop
            </Toast.Description>
            <Toast.Close className="toast-close" asChild>
                <button data-testid="uncontrolled-close">Close uncontrolled</button>
            </Toast.Close>
        </Toast.Root>
    );
}
