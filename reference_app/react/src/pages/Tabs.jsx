import {useState} from 'react';
import * as Tabs from '@radix-ui/react-tabs';
import '../../../shared/tabs.css';

export default function TabsPage() {
    const [orientation, setOrientation] = useState('horizontal');
    const [activationMode, setActivationMode] = useState('automatic');

    return (
        <>
            <Tabs.Root
                defaultValue="tab1"
                orientation={orientation}
                activationMode={activationMode}
                className="tabs-root"
            >
                <Tabs.List aria-label="tabs example" className="tabs-list">
                    <Tabs.Trigger value="tab1" className="tabs-trigger">
                        Tab 1
                    </Tabs.Trigger>
                    <Tabs.Trigger value="tab2" disabled className="tabs-trigger">
                        Tab 2
                    </Tabs.Trigger>
                    <Tabs.Trigger value="tab3" className="tabs-trigger">
                        Tab 3
                    </Tabs.Trigger>
                </Tabs.List>
                <Tabs.Content value="tab1" className="tabs-content">
                    Content 1
                </Tabs.Content>
                <Tabs.Content value="tab2" className="tabs-content">
                    Content 2
                </Tabs.Content>
                <Tabs.Content value="tab3" className="tabs-content">
                    Content 3
                </Tabs.Content>
            </Tabs.Root>

            <br />
            <br />

            <fieldset>
                <legend>Orientation</legend>
                <label>
                    <input
                        type="radio"
                        name="orientation"
                        value="horizontal"
                        checked={orientation === 'horizontal'}
                        onChange={() => setOrientation('horizontal')}
                    />{' '}
                    horizontal
                </label>
                <br />
                <label>
                    <input
                        type="radio"
                        name="orientation"
                        value="vertical"
                        checked={orientation === 'vertical'}
                        onChange={() => setOrientation('vertical')}
                    />{' '}
                    vertical
                </label>
            </fieldset>

            <fieldset>
                <legend>Activation Mode</legend>
                <label>
                    <input
                        type="radio"
                        name="activation"
                        value="automatic"
                        checked={activationMode === 'automatic'}
                        onChange={() => setActivationMode('automatic')}
                    />{' '}
                    automatic
                </label>
                <br />
                <label>
                    <input
                        type="radio"
                        name="activation"
                        value="manual"
                        checked={activationMode === 'manual'}
                        onChange={() => setActivationMode('manual')}
                    />{' '}
                    manual
                </label>
            </fieldset>
        </>
    );
}
