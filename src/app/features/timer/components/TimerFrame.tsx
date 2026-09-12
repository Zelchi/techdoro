import Icon from '../../../components/Icon';
import type { TimerController } from '../useTimer';
import Clock from './Clock/Clock';

type TimerFrameProps = {
    timer: TimerController;
    onToggleSettings: () => void;
    click: () => void;
};

export default function TimerFrame(props: TimerFrameProps) {
    const isActiveSession = (session: number): boolean => props.timer.session() === session;

    return (
        <div class="timer-frame">
            <div class="timer-toolbar">
                <div class="toolbar-side toolbar-side--left">
                    <button
                        class="icon-button"
                        type="button"
                        onClick={() => {
                            props.timer.next();
                            props.click();
                        }}
                    >
                        <Icon name="next" size={18} />
                    </button>
                </div>
                <div class="cycle-indicator" role="status">
                    <span class={`cycle-dot${isActiveSession(1) ? ' active' : ''}`} />
                    <span class={`cycle-dot${isActiveSession(2) ? ' active' : ''}`} />
                    <span class={`cycle-dot${isActiveSession(3) ? ' active' : ''}`} />
                </div>
                <div class="toolbar-side toolbar-side--right">
                    <button
                        class="icon-button"
                        type="button"
                        onClick={() => {
                            props.onToggleSettings();
                            props.click();
                        }}
                    >
                        <Icon name="settings" size={18} />
                    </button>
                </div>
            </div>
            <Clock
                timeNow={props.timer.timeNow}
                timeMax={props.timer.timeMax}
                isRunning={props.timer.isRunning}
                onToggle={props.timer.toggle}
                onReset={props.timer.reset}
                onClick={props.click}
            />
        </div>
    );
}
