import {useState} from 'react';
import * as Avatar from '@radix-ui/react-avatar';
import '../../../shared/avatar.css';

const WORKING_SRC = 'https://picsum.photos/id/1005/200/200';
const BROKEN_SRC = 'https://broken.example.com/no-image.png';

export default function AvatarPage() {
    const [workingStatus, setWorkingStatus] = useState('idle');
    const [brokenStatus, setBrokenStatus] = useState('idle');
    const [dynamicSrc, setDynamicSrc] = useState(undefined);

    return (
        <>
            <h2>Avatar</h2>

            {/* No image — immediate fallback */}
            <div className="avatar-container">
                <Avatar.Root className="avatar-root" data-testid="avatar-no-image">
                    <Avatar.Image className="avatar-image" />
                    <Avatar.Fallback className="avatar-fallback">NI</Avatar.Fallback>
                </Avatar.Root>
                <span>No image</span>
            </div>

            {/* Working image */}
            <div className="avatar-container">
                <Avatar.Root className="avatar-root" data-testid="avatar-working">
                    <Avatar.Image
                        className="avatar-image"
                        src={WORKING_SRC}
                        alt="Working avatar"
                        onLoadingStatusChange={setWorkingStatus}
                    />
                    <Avatar.Fallback className="avatar-fallback">WI</Avatar.Fallback>
                </Avatar.Root>
                <span>
                    Working image (status: <span data-testid="status-working">{workingStatus}</span>)
                </span>
            </div>

            {/* Broken image */}
            <div className="avatar-container">
                <Avatar.Root className="avatar-root" data-testid="avatar-broken">
                    <Avatar.Image
                        className="avatar-image"
                        src={BROKEN_SRC}
                        alt="Broken avatar"
                        onLoadingStatusChange={setBrokenStatus}
                    />
                    <Avatar.Fallback className="avatar-fallback">BI</Avatar.Fallback>
                </Avatar.Root>
                <span>
                    Broken image (status: <span data-testid="status-broken">{brokenStatus}</span>)
                </span>
            </div>

            {/* Delayed fallback with broken image */}
            <div className="avatar-container">
                <Avatar.Root className="avatar-root" data-testid="avatar-delayed">
                    <Avatar.Image className="avatar-image" src={BROKEN_SRC} alt="Delayed avatar" />
                    <Avatar.Fallback className="avatar-fallback" delayMs={300}>
                        DI
                    </Avatar.Fallback>
                </Avatar.Root>
                <span>Delayed fallback (300ms)</span>
            </div>

            {/* Dynamic source */}
            <div className="avatar-container">
                <Avatar.Root className="avatar-root" data-testid="avatar-dynamic">
                    <Avatar.Image className="avatar-image" src={dynamicSrc} alt="Dynamic avatar" />
                    <Avatar.Fallback className="avatar-fallback">DY</Avatar.Fallback>
                </Avatar.Root>
                <span>Dynamic source</span>
            </div>

            <div>
                <button data-testid="set-working-src" onClick={() => setDynamicSrc(WORKING_SRC)}>
                    Set working src
                </button>{' '}
                <button data-testid="set-broken-src" onClick={() => setDynamicSrc(BROKEN_SRC)}>
                    Set broken src
                </button>{' '}
                <button data-testid="clear-src" onClick={() => setDynamicSrc(undefined)}>
                    Clear src
                </button>
            </div>
        </>
    );
}
