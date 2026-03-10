import {useState} from 'react';
import * as AlertDialog from '@radix-ui/react-alert-dialog';
import '../../../shared/alert-dialog.css';

export default function AlertDialogPage() {
    const [count, setCount] = useState(0);
    const [controlledOpen, setControlledOpen] = useState(false);

    return (
        <>
            <AlertDialog.Root>
                <AlertDialog.Trigger>delete</AlertDialog.Trigger>
                <AlertDialog.Portal>
                    <AlertDialog.Overlay className="alert-dialog-overlay" />
                    <AlertDialog.Content className="alert-dialog-content">
                        <AlertDialog.Title>Are you sure?</AlertDialog.Title>
                        <AlertDialog.Description>This action cannot be undone.</AlertDialog.Description>
                        <AlertDialog.Cancel>cancel</AlertDialog.Cancel>
                        <AlertDialog.Action>confirm</AlertDialog.Action>
                    </AlertDialog.Content>
                </AlertDialog.Portal>
            </AlertDialog.Root>

            <br />
            <br />

            <label>
                count up{' '}
                <button type="button" onClick={() => setCount((c) => c + 1)}>
                    {count}
                </button>
            </label>

            <br />
            <br />

            {/* Controlled alert dialog */}
            <AlertDialog.Root open={controlledOpen} onOpenChange={setControlledOpen}>
                <AlertDialog.Trigger data-testid="controlled-trigger">controlled delete</AlertDialog.Trigger>
                <AlertDialog.Portal>
                    <AlertDialog.Overlay className="alert-dialog-overlay" />
                    <AlertDialog.Content className="alert-dialog-content" data-testid="controlled-content">
                        <AlertDialog.Title>Controlled Alert</AlertDialog.Title>
                        <AlertDialog.Description>This is a controlled alert dialog.</AlertDialog.Description>
                        <AlertDialog.Cancel data-testid="controlled-cancel">cancel</AlertDialog.Cancel>
                        <AlertDialog.Action data-testid="controlled-action">confirm</AlertDialog.Action>
                    </AlertDialog.Content>
                </AlertDialog.Portal>
            </AlertDialog.Root>

            <label>
                <input
                    type="checkbox"
                    data-testid="controlled-checkbox"
                    checked={controlledOpen}
                    onChange={(e) => setControlledOpen(e.target.checked)}
                />{' '}
                controlled open
            </label>
            <span data-testid="controlled-state">{controlledOpen ? 'open' : 'closed'}</span>
            <button data-testid="controlled-external-close" onClick={() => setControlledOpen(false)}>
                external close
            </button>
        </>
    );
}
