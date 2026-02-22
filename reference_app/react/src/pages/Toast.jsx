import {useState, useRef} from 'react';
import * as Toast from '@radix-ui/react-toast';
import '../../../shared/toast.css';

export default function ToastPage() {
    const [open, setOpen] = useState(false);
    const [autoDismiss, setAutoDismiss] = useState(false);
    const [count, setCount] = useState(0);
    const timerRef = useRef(0);

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

            <Toast.Viewport className="toast-viewport" data-testid="toast-viewport" label="Notifications" />
        </Toast.Provider>
    );
}
