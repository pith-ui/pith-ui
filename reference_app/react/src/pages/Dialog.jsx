import {useState} from 'react';
import * as Dialog from '@radix-ui/react-dialog';
import '../../../shared/dialog.css';

export default function DialogPage() {
    const [modal, setModal] = useState(true);
    const [animated, setAnimated] = useState(false);
    const [count, setCount] = useState(0);
    const [hasDestroyButton, setHasDestroyButton] = useState(true);

    return (
        <>
            <Dialog.Root modal={modal}>
                <Dialog.Trigger>open</Dialog.Trigger>
                <Dialog.Portal>
                    <Dialog.Overlay
                        data-testid="overlay"
                        className={[
                            'dialog-overlay',
                            animated && 'dialog-animated-overlay',
                            animated && 'dialog-duration-50',
                        ]
                            .filter(Boolean)
                            .join(' ')}
                    />
                    <Dialog.Content
                        className={[
                            'dialog-content',
                            animated && 'dialog-animated-content',
                            animated && 'dialog-duration-50',
                        ]
                            .filter(Boolean)
                            .join(' ')}
                    >
                        <Dialog.Title>title</Dialog.Title>
                        <Dialog.Description>description</Dialog.Description>
                        <Dialog.Close>close</Dialog.Close>
                        {hasDestroyButton && (
                            <div>
                                <button type="button" onClick={() => setHasDestroyButton(false)}>
                                    destroy me
                                </button>
                            </div>
                        )}
                    </Dialog.Content>
                </Dialog.Portal>
            </Dialog.Root>

            {/* Second dialog for internal styles testing */}
            <Dialog.Root>
                <Dialog.Trigger data-testid="styled-dialog-trigger">open styled</Dialog.Trigger>
                <Dialog.Portal>
                    <Dialog.Overlay
                        data-testid="styled-overlay"
                        className="dialog-overlay"
                        style={{background: 'tomato'}}
                    />
                    <Dialog.Content className="dialog-content">
                        <Dialog.Title>styled title</Dialog.Title>
                        <Dialog.Close>close styled</Dialog.Close>
                    </Dialog.Content>
                </Dialog.Portal>
            </Dialog.Root>

            <br />
            <br />

            <label>
                <input
                    type="checkbox"
                    checked={modal}
                    onChange={(event) => setModal(event.target.checked)}
                />{' '}
                modal
            </label>

            <br />

            <label>
                <input
                    type="checkbox"
                    checked={animated}
                    onChange={(event) => setAnimated(event.target.checked)}
                />{' '}
                animated
            </label>

            <br />

            <label>
                count up{' '}
                <button type="button" onClick={() => setCount((c) => c + 1)}>
                    {count}
                </button>
            </label>

            <br />

            <label>
                name: <input type="text" placeholder="name" />
            </label>
        </>
    );
}
