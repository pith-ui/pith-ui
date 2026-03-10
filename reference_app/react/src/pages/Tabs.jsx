import {useState} from 'react';
import * as Tabs from '@radix-ui/react-tabs';
import '../../../shared/tabs.css';

export default function TabsPage() {
    const [orientation, setOrientation] = useState('horizontal');
    const [activationMode, setActivationMode] = useState('automatic');
    const [controlledValue, setControlledValue] = useState('ctab1');

    return (
        <>
            <div data-testid="uncontrolled-tabs-section">
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
            </div>

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

            <hr />

            {/* Force-mounted tabs for hidden attribute test */}
            <div data-testid="force-mount-tabs-section" aria-hidden="true">
                <Tabs.Root defaultValue="fm1" className="tabs-root">
                    <Tabs.List aria-label="force mount tabs" className="tabs-list">
                        <Tabs.Trigger value="fm1" className="tabs-trigger" data-testid="fm-trigger-1">FM 1</Tabs.Trigger>
                        <Tabs.Trigger value="fm2" className="tabs-trigger" data-testid="fm-trigger-2">FM 2</Tabs.Trigger>
                    </Tabs.List>
                    <Tabs.Content value="fm1" className="tabs-content" data-testid="fm-content-1" forceMount>FM Content 1</Tabs.Content>
                    <Tabs.Content value="fm2" className="tabs-content" data-testid="fm-content-2" forceMount>FM Content 2</Tabs.Content>
                </Tabs.Root>
            </div>

            <hr />

            <div data-testid="controlled-tabs-section" aria-hidden="true">
                <h3>Controlled Tabs</h3>
                <fieldset>
                    <legend>External Tab Control</legend>
                    <button data-testid="controlled-select-tab1" onClick={() => setControlledValue('ctab1')}>Select Tab 1</button>
                    <button data-testid="controlled-select-tab2" onClick={() => setControlledValue('ctab2')}>Select Tab 2</button>
                    <button data-testid="controlled-select-tab3" onClick={() => setControlledValue('ctab3')}>Select Tab 3</button>
                </fieldset>
                <span data-testid="controlled-value-display">{controlledValue}</span>
                <Tabs.Root
                    value={controlledValue}
                    onValueChange={setControlledValue}
                    className="tabs-root"
                    data-testid="controlled-tabs"
                >
                    <Tabs.List aria-label="controlled tabs example" className="tabs-list">
                        <Tabs.Trigger value="ctab1" className="tabs-trigger" data-testid="controlled-tab-trigger-1">CTab 1</Tabs.Trigger>
                        <Tabs.Trigger value="ctab2" className="tabs-trigger" data-testid="controlled-tab-trigger-2">CTab 2</Tabs.Trigger>
                        <Tabs.Trigger value="ctab3" className="tabs-trigger" data-testid="controlled-tab-trigger-3">CTab 3</Tabs.Trigger>
                    </Tabs.List>
                    <Tabs.Content value="ctab1" className="tabs-content" data-testid="controlled-tab-content-1">Controlled Content 1</Tabs.Content>
                    <Tabs.Content value="ctab2" className="tabs-content" data-testid="controlled-tab-content-2">Controlled Content 2</Tabs.Content>
                    <Tabs.Content value="ctab3" className="tabs-content" data-testid="controlled-tab-content-3">Controlled Content 3</Tabs.Content>
                </Tabs.Root>
            </div>
        </>
    );
}
