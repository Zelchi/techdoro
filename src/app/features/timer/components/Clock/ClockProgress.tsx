type ProgressBarProps = {
    timeNow: number;
    timeMax: number;
};

export default function ClockProgress(props: ProgressBarProps) {
    const progress = (): number => {
        if (props.timeMax <= 0) return 100;
        return Math.max(0, Math.min(100, 100 - (props.timeNow / props.timeMax) * 100));
    };

    return (
        <div
            class="progress-track"
            role="progressbar"
            aria-valuemin="0"
            aria-valuemax={props.timeMax}
            aria-valuenow={props.timeNow}
        >
            <div class="progress-value" style={{ width: `${progress()}%` }} />
        </div>
    );
}
