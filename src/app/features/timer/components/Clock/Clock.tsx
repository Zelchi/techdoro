import type { Accessor } from 'solid-js';
import Icon from '../../../../components/Icon';
import ClockProgress from './ClockProgress';

type ClockProps = {
    timeNow: Accessor<number>;
    timeMax: Accessor<number>;
    isRunning: Accessor<boolean>;
    onToggle: () => void;
    onReset: () => void;
    onClick: () => void;
};

const formatTime = (time: number): string => {
    const total = Math.max(0, Math.floor(time));
    const minutes = Math.floor(total / 60);
    const seconds = total - minutes * 60;
    return `${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`;
};

export default function Clock(props: ClockProps) {
    return (
        <section class="clock-area">
            <div class="clock-face">
                <h1>{formatTime(props.timeNow())}</h1>
                <ClockProgress timeNow={props.timeNow()} timeMax={props.timeMax()} />
            </div>
            <div class="clock-actions">
                {!props.isRunning() && props.timeNow() !== props.timeMax() && <span class="clock-action-spacer" />}
                <button
                    class="primary-button"
                    type="button"
                    onClick={() => {
                        props.onClick();
                        props.onToggle();
                    }}
                >
                    {props.isRunning() ? 'Pause' : 'Start'}
                </button>
                {!props.isRunning() && props.timeNow() !== props.timeMax() && (
                    <button
                        class="icon-button icon-button--large"
                        type="button"
                        onClick={() => {
                            props.onClick();
                            props.onReset();
                        }}
                    >
                        <Icon name="reset" size={22} />
                    </button>
                )}
            </div>
        </section>
    );
}
