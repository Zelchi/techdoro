import type { Accessor } from 'solid-js';

type StepFieldProps = {
    label: string;
    value: Accessor<number>;
    onDec: () => void;
    onInc: () => void;
};

export default function StepField(props: StepFieldProps) {
    return (
        <fieldset class="config-item">
            <span>{props.label}</span>
            <div class="step-row" onWheel={(event) => event.preventDefault()}>
                <button class="step-button" type="button" onClick={props.onDec}>
                    -
                </button>
                <input
                    type="number"
                    min="1"
                    value={props.value()}
                    readonly
                    inputmode="none"
                    onWheel={(event) => event.preventDefault()}
                    onKeyDown={(event) => {
                        if (event.key === 'ArrowUp' || event.key === 'ArrowDown') event.preventDefault();
                    }}
                />
                <button class="step-button" type="button" onClick={props.onInc}>
                    +
                </button>
            </div>
        </fieldset>
    );
}
