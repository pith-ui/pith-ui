import {useState} from 'react';
import * as AlertDialog from '@radix-ui/react-alert-dialog';
import '../../../shared/alert-dialog.css';

export default function AlertDialogPage() {
    const [count, setCount] = useState(0);

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
        </>
    );
}
