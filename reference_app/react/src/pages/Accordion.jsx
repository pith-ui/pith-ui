import {useState} from 'react';
import * as Accordion from '@radix-ui/react-accordion';
import '../../../shared/accordion.css';

export default function AccordionPage() {
    const [type, setType] = useState('single');
    const [collapsible, setCollapsible] = useState(false);

    return (
        <>
            {type === 'single' ? (
                <Accordion.Root
                    type="single"
                    collapsible={collapsible}
                    className="accordion-root"
                    data-testid="accordion-root"
                >
                    <AccordionItems />
                </Accordion.Root>
            ) : (
                <Accordion.Root type="multiple" className="accordion-root" data-testid="accordion-root">
                    <AccordionItems />
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
        </>
    );
}

function AccordionItems() {
    return (
        <>
            <Accordion.Item value="item-1" className="accordion-item" data-testid="item-1">
                <Accordion.Header className="accordion-header">
                    <Accordion.Trigger className="accordion-trigger">Item 1</Accordion.Trigger>
                </Accordion.Header>
                <Accordion.Content className="accordion-content">Content 1</Accordion.Content>
            </Accordion.Item>
            <Accordion.Item value="item-2" className="accordion-item" data-testid="item-2" disabled>
                <Accordion.Header className="accordion-header">
                    <Accordion.Trigger className="accordion-trigger">Item 2</Accordion.Trigger>
                </Accordion.Header>
                <Accordion.Content className="accordion-content">Content 2</Accordion.Content>
            </Accordion.Item>
            <Accordion.Item value="item-3" className="accordion-item" data-testid="item-3">
                <Accordion.Header className="accordion-header">
                    <Accordion.Trigger className="accordion-trigger">Item 3</Accordion.Trigger>
                </Accordion.Header>
                <Accordion.Content className="accordion-content">Content 3</Accordion.Content>
            </Accordion.Item>
        </>
    );
}
