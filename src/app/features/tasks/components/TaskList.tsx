import { type Accessor, createSignal, For, onCleanup, onMount } from 'solid-js';
import Icon from '../../../components/Icon';
import type { Task } from '../model';
import TaskCheckbox from './TaskCheckbox';

type TaskListProps = {
    tasks: Accessor<Task[]>;
    addTask: (text: string) => boolean;
    deleteTask: (id: number) => void;
    toggleTask: (id: number) => void;
    updateTask: (id: number, text: string) => void;
    click: () => void;
};

export default function TaskList(props: TaskListProps) {
    const [newTask, setNewTask] = createSignal('');
    const [editingId, setEditingId] = createSignal<number | null>(null);
    const [editingText, setEditingText] = createSignal('');
    const [containerWidth, setContainerWidth] = createSignal(0);
    let containerRef: HTMLDivElement | undefined;

    onMount(() => {
        const updateWidth = (): void => {
            setContainerWidth(containerRef?.clientWidth ?? 0);
        };

        updateWidth();
        window.addEventListener('resize', updateWidth);

        let observer: ResizeObserver | undefined;
        if (containerRef && 'ResizeObserver' in window) {
            observer = new ResizeObserver(updateWidth);
            observer.observe(containerRef);
        }

        onCleanup(() => {
            window.removeEventListener('resize', updateWidth);
            observer?.disconnect();
        });
    });

    const addTask = (event: SubmitEvent): void => {
        event.preventDefault();
        if (!props.addTask(newTask())) return;

        setNewTask('');
        props.click();
    };

    const startEditing = (task: Task): void => {
        setEditingId(task.id);
        setEditingText(task.text);
        props.click();
    };

    const cancelEditing = (): void => {
        setEditingId(null);
        setEditingText('');
    };

    const saveEditing = (id: number): void => {
        props.updateTask(id, editingText());
        cancelEditing();
        props.click();
    };

    const shouldScroll = (text: string): boolean => {
        const canvas = document.createElement('canvas');
        const context = canvas.getContext('2d');
        if (!context) return false;

        context.font = '16px "Press Start 2P"';
        return context.measureText(text).width + 100 > containerWidth();
    };

    const taskTextClass = (task: Task): string =>
        'task-text' +
        (task.completed ? ' task-text--completed' : '') +
        (shouldScroll(task.text) ? ' task-text--scroll' : '');

    return (
        <section class="taskbar">
            <form class="task-form" onSubmit={addTask}>
                <input
                    class="task-input"
                    type="text"
                    value={newTask()}
                    onInput={(event) => setNewTask(event.currentTarget.value)}
                />
                <button class="task-add-button" type="submit">
                    <Icon name="plus" size={20} />
                </button>
            </form>
            <div
                class="task-list"
                ref={(element) => {
                    containerRef = element;
                }}
            >
                <For each={props.tasks()}>
                    {(task) => (
                        <div class="task-row">
                            {editingId() === task.id ? (
                                <>
                                    <input
                                        class="task-input task-input--editing"
                                        type="text"
                                        value={editingText()}
                                        autofocus
                                        onInput={(event) => setEditingText(event.currentTarget.value)}
                                        onBlur={() => saveEditing(task.id)}
                                        onKeyDown={(event) => {
                                            if (event.key === 'Enter') {
                                                event.preventDefault();
                                                saveEditing(task.id);
                                            }
                                            if (event.key === 'Escape') cancelEditing();
                                        }}
                                    />
                                    <button
                                        class="task-save-button"
                                        type="button"
                                        onMouseDown={(event) => event.preventDefault()}
                                        onClick={() => saveEditing(task.id)}
                                    >
                                        <Icon name="check" size={16} />
                                    </button>
                                </>
                            ) : (
                                <div class="task-text-container">
                                    <button
                                        class={taskTextClass(task)}
                                        type="button"
                                        onClick={() => startEditing(task)}
                                    >
                                        {task.text}
                                    </button>
                                </div>
                            )}
                            {editingId() !== task.id && (
                                <div class="task-actions">
                                    <TaskCheckbox
                                        task={task}
                                        onToggle={(id) => {
                                            props.toggleTask(id);
                                            props.click();
                                        }}
                                    />
                                    <button
                                        class="task-delete-button"
                                        type="button"
                                        onClick={() => {
                                            props.deleteTask(task.id);
                                            props.click();
                                        }}
                                    >
                                        <Icon name="trash" size={16} />
                                    </button>
                                </div>
                            )}
                        </div>
                    )}
                </For>
            </div>
        </section>
    );
}
