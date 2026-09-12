import Icon from '../../../components/Icon';
import type { Task } from '../model';

type CheckboxProps = {
    task: Task;
    onToggle: (id: number) => void;
};

export default function TaskCheckbox(props: CheckboxProps) {
    return (
        <button
            class={`task-checkbox${props.task.completed ? ' task-checkbox--checked' : ''}`}
            type="button"
            onClick={() => props.onToggle(props.task.id)}
            aria-pressed={props.task.completed ? 'true' : 'false'}
        >
            {props.task.completed && <Icon name="check" size={18} />}
        </button>
    );
}
