import type { Accessor } from 'solid-js';
import Icon, { type IconName } from '../../../components/Icon';

type VolumeBarProps = {
    value: Accessor<number>;
    onChange: (value: number) => void;
    onClick: () => void;
};

export default function VolumeBar(props: VolumeBarProps) {
    const iconName = (): IconName => {
        if (props.value() === 0) return 'volume-mute';
        if (props.value() <= 30) return 'volume-down';
        return 'volume-up';
    };

    const toggleMute = (): void => {
        props.onChange(props.value() === 0 ? 50 : 0);
        props.onClick();
    };

    return (
        <div class="volume-bar">
            <div class="volume-group">
                <button class="volume-icon" type="button" onClick={toggleMute}>
                    <Icon name={iconName()} size={16} />
                </button>
                <input
                    class="volume-slider"
                    type="range"
                    min="0"
                    max="100"
                    step="1"
                    value={props.value()}
                    style={{ '--volume': `${props.value()}%` }}
                    onInput={(event) => props.onChange(Number(event.currentTarget.value))}
                />
                <span class="volume-value">{props.value()}%</span>
            </div>
        </div>
    );
}
