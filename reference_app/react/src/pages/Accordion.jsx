import {useState} from 'react';
import * as Accordion from '@radix-ui/react-accordion';
import '../../../shared/accordion.css';

export default function AccordionPage() {
    const [type, setType] = useState('single');
    const [collapsible, setCollapsible] = useState(false);
    const [animated, setAnimated] = useState(false);

    return (
        <>
            {type === 'single' ? (
                <Accordion.Root
                    type="single"
                    collapsible={collapsible}
                    className="accordion-root"
                    data-testid="accordion-root"
                >
                    <AccordionItems animated={animated} />
                </Accordion.Root>
            ) : (
                <Accordion.Root type="multiple" className="accordion-root" data-testid="accordion-root">
                    <AccordionItems animated={animated} />
                </Accordion.Root>
            )}

            <br />
            <br />

            <fieldset>
                <legend>type</legend>
                <label>
                    <input
                        type="radio"
                        name="type"
                        value="single"
                        checked={type === 'single'}
                        onChange={() => setType('single')}
                    />{' '}
                    single
                </label>
                <br />
                <label>
                    <input
                        type="radio"
                        name="type"
                        value="multiple"
                        checked={type === 'multiple'}
                        onChange={() => setType('multiple')}
                    />{' '}
                    multiple
                </label>
            </fieldset>

            <br />

            <label>
                <input
                    type="checkbox"
                    checked={collapsible}
                    onChange={(event) => setCollapsible(event.target.checked)}
                />{' '}
                collapsible
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
            <br />

            <ControlledAccordion />
        </>
    );
}

function ControlledAccordion() {
    const [controlledValue, setControlledValue] = useState('');
    return (
        <>
            <h2 data-testid="controlled-heading">Controlled Accordion</h2>
            <div data-testid="controlled-value">{controlledValue}</div>
            <button data-testid="controlled-open-item-1" onClick={() => setControlledValue('ctrl-item-1')}>Open Item 1</button>
            <button data-testid="controlled-open-item-2" onClick={() => setControlledValue('ctrl-item-2')}>Open Item 2</button>
            <button data-testid="controlled-close-all" onClick={() => setControlledValue('')}>Close All</button>
            <Accordion.Root
                type="single"
                collapsible
                value={controlledValue}
                onValueChange={setControlledValue}
                className="accordion-root"
                data-testid="controlled-accordion-root"
            >
                <Accordion.Item value="ctrl-item-1" className="accordion-item" data-testid="ctrl-item-1">
                    <Accordion.Header className="accordion-header">
                        <Accordion.Trigger className="accordion-trigger">Ctrl Item 1</Accordion.Trigger>
                    </Accordion.Header>
                    <Accordion.Content className="accordion-content">Controlled Content 1</Accordion.Content>
                </Accordion.Item>
                <Accordion.Item value="ctrl-item-2" className="accordion-item" data-testid="ctrl-item-2">
                    <Accordion.Header className="accordion-header">
                        <Accordion.Trigger className="accordion-trigger">Ctrl Item 2</Accordion.Trigger>
                    </Accordion.Header>
                    <Accordion.Content className="accordion-content">Controlled Content 2</Accordion.Content>
                </Accordion.Item>
            </Accordion.Root>
        </>
    );
}

function AccordionItems({animated}) {
    const contentClass = animated ? 'accordion-content accordion-content-animated' : 'accordion-content';
    return (
        <>
            <Accordion.Item value="item-1" className="accordion-item" data-testid="item-1" data-custom="accordion-item-custom">
                <Accordion.Header className="accordion-header" data-custom="accordion-header-custom">
                    <Accordion.Trigger className="accordion-trigger" data-custom="accordion-trigger-custom">Item 1</Accordion.Trigger>
                </Accordion.Header>
                <Accordion.Content className={contentClass} data-custom="accordion-content-custom">Content 1</Accordion.Content>
            </Accordion.Item>
            <Accordion.Item value="item-2" className="accordion-item" data-testid="item-2" disabled>
                <Accordion.Header className="accordion-header">
                    <Accordion.Trigger className="accordion-trigger">Item 2</Accordion.Trigger>
                </Accordion.Header>
                <Accordion.Content className={contentClass}>Content 2</Accordion.Content>
            </Accordion.Item>
            <Accordion.Item value="item-3" className="accordion-item" data-testid="item-3">
                <Accordion.Header className="accordion-header">
                    <Accordion.Trigger className="accordion-trigger">Item 3</Accordion.Trigger>
                </Accordion.Header>
                <Accordion.Content className={contentClass}>Content 3</Accordion.Content>
            </Accordion.Item>
            <Accordion.Item value="item-styled" className="accordion-item" data-testid="item-styled">
                <Accordion.Header className="accordion-header">
                    <Accordion.Trigger className="accordion-trigger">Styled Item</Accordion.Trigger>
                </Accordion.Header>
                <Accordion.Content
                    className={contentClass}
                    data-testid="styled-content"
                    style={{background: 'tomato', '--radix-accordion-content-height': '999px'}}
                >
                    Styled content
                </Accordion.Content>
            </Accordion.Item>
        </>
    );
}
