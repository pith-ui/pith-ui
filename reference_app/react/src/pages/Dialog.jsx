import {useState} from 'react';
import * as Dialog from '@radix-ui/react-dialog';
import '../../../shared/dialog.css';

export default function DialogPage() {
    const [modal, setModal] = useState(true);
    const [animated, setAnimated] = useState(false);
    const [count, setCount] = useState(0);
    const [hasDestroyButton, setHasDestroyButton] = useState(true);
    const [controlledOpen, setControlledOpen] = useState(false);
    const [eventLog, setEventLog] = useState([]);
    const [preventEscape, setPreventEscape] = useState(false);
    const [preventOutsideClick, setPreventOutsideClick] = useState(false);

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

            <hr />

            <div data-testid="controlled-dialog-section">
                <h3>Controlled Dialog</h3>

                <label>
                    <input
                        type="checkbox"
                        data-testid="controlled-dialog-checkbox"
                        checked={controlledOpen}
                        onChange={(e) => setControlledOpen(e.target.checked)}
                    />{' '}
                    controlled open
                </label>
                <button
                    type="button"
                    data-testid="controlled-dialog-external-close"
                    onClick={() => setControlledOpen(false)}
                >
                    external close
                </button>
                <span data-testid="controlled-dialog-state">{controlledOpen ? 'open' : 'closed'}</span>

                <Dialog.Root open={controlledOpen} onOpenChange={setControlledOpen}>
                    <Dialog.Trigger data-testid="controlled-dialog-trigger">open controlled</Dialog.Trigger>
                    <Dialog.Portal>
                        <Dialog.Overlay className="dialog-overlay" />
                        <Dialog.Content className="dialog-content" data-testid="controlled-dialog-content">
                            <Dialog.Title>controlled title</Dialog.Title>
                            <Dialog.Description>controlled description</Dialog.Description>
                            <Dialog.Close data-testid="controlled-dialog-close">close controlled</Dialog.Close>
                        </Dialog.Content>
                    </Dialog.Portal>
                </Dialog.Root>
            </div>

            <br />
            <br />

            <hr />

            {/* Callback contract dialog */}
            <div data-testid="callback-dialog-section">
                <h3>Callback Dialog</h3>
                <label>
                    <input
                        type="checkbox"
                        data-testid="prevent-escape"
                        checked={preventEscape}
                        onChange={(e) => setPreventEscape(e.target.checked)}
                    />{' '}
                    prevent escape
                </label>
                <label>
                    <input
                        type="checkbox"
                        data-testid="prevent-outside-click"
                        checked={preventOutsideClick}
                        onChange={(e) => setPreventOutsideClick(e.target.checked)}
                    />{' '}
                    prevent outside click
                </label>
                <button type="button" data-testid="clear-event-log" onClick={() => setEventLog([])}>
                    clear log
                </button>
                <span data-testid="event-log">{eventLog.join(',')}</span>

                <Dialog.Root>
                    <Dialog.Trigger data-testid="callback-trigger">open callback</Dialog.Trigger>
                    <Dialog.Portal>
                        <Dialog.Overlay className="dialog-overlay" data-testid="callback-overlay" />
                        <Dialog.Content
                            className="dialog-content"
                            data-testid="callback-content"
                            onOpenAutoFocus={(e) => {
                                setEventLog((prev) => [...prev, 'openAutoFocus']);
                            }}
                            onCloseAutoFocus={(e) => {
                                setEventLog((prev) => [...prev, 'closeAutoFocus']);
                            }}
                            onEscapeKeyDown={(e) => {
                                setEventLog((prev) => [...prev, 'escapeKeyDown']);
                                if (preventEscape) e.preventDefault();
                            }}
                            onPointerDownOutside={(e) => {
                                setEventLog((prev) => [...prev, 'pointerDownOutside']);
                                if (preventOutsideClick) e.preventDefault();
                            }}
                            onInteractOutside={(e) => {
                                setEventLog((prev) => [...prev, 'interactOutside']);
                            }}
                        >
                            <Dialog.Title>callback title</Dialog.Title>
                            <Dialog.Close data-testid="callback-close">close callback</Dialog.Close>
                        </Dialog.Content>
                    </Dialog.Portal>
                </Dialog.Root>
            </div>

            <hr />

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
