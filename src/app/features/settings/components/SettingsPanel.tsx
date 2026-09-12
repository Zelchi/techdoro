import type { Accessor } from 'solid-js';
import Icon from '../../../components/Icon';
import type { Settings } from '../model';
import StepField from './StepField';
import VolumeBar from './VolumeBar';

type MenuProps = {
    settings: Accessor<Settings>;
    updateSettings: (changes: Partial<Settings>) => void;
    onClose: () => void;
    click: () => void;
    isOpen: boolean;
};

export default function SettingsPanel(props: MenuProps) {
    const settings = (): Settings => props.settings();

    const adjust = (key: keyof Settings, delta: number): void => {
        const current = settings()[key];
        const next = Number(current) + delta;
        props.updateSettings({ [key]: next });
        props.click();
    };

    const fields = [
        { key: 'focusMinutes' as const, label: 'Focus' },
        { key: 'shortBreakMinutes' as const, label: 'Short Break' },
        { key: 'longBreakMinutes' as const, label: 'Long Break' },
        { key: 'cyclesBeforeLongBreak' as const, label: 'Cycles' },
    ];

    return (
        <aside class={`settings-panel${props.isOpen ? ' settings-panel--open' : ''}`} aria-hidden={!props.isOpen}>
            <div class="settings-topbar">
                <VolumeBar
                    value={() => settings().volume}
                    onChange={(value) => props.updateSettings({ volume: value })}
                    onClick={props.click}
                />
                <button
                    class="icon-button settings-close"
                    type="button"
                    onClick={() => {
                        props.onClose();
                        props.click();
                    }}
                >
                    <Icon name="thumbsup" size={18} />
                </button>
            </div>
            <div class="settings-grid">
                {fields.map((field) => (
                    <StepField
                        label={field.label}
                        value={() => settings()[field.key]}
                        onDec={() => adjust(field.key, -1)}
                        onInc={() => adjust(field.key, 1)}
                    />
                ))}
            </div>
        </aside>
    );
}
