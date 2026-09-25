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
    reorderTask: (id: number, targetId: number, position: 'before' | 'after') => void;
    click: () => void;
};

type DropTarget = {
    id: number;
    position: 'before' | 'after';
};

export default function TaskList(props: TaskListProps) {
    const [newTask, setNewTask] = createSignal('');
    const [editingId, setEditingId] = createSignal<number | null>(null);
    const [editingText, setEditingText] = createSignal('');
    const [containerWidth, setContainerWidth] = createSignal(0);
    const [draggingId, setDraggingId] = createSignal<number | null>(null);
    const [dropTarget, setDropTarget] = createSignal<DropTarget | null>(null);
    let containerRef: HTMLUListElement | undefined;

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

    const getDropPosition = (event: DragEvent): 'before' | 'after' => {
        const element = event.currentTarget as HTMLLIElement;
        const bounds = element.getBoundingClientRect();
        return event.clientY < bounds.top + bounds.height / 2 ? 'before' : 'after';
    };

    const startDragging = (event: DragEvent, task: Task): void => {
        event.dataTransfer?.setData('text/plain', String(task.id));
        if (event.dataTransfer) event.dataTransfer.effectAllowed = 'move';
        setDraggingId(task.id);
        setDropTarget(null);
    };

    const updateDropTarget = (event: DragEvent, task: Task): void => {
        const currentId = draggingId();
        if (currentId === null || currentId === task.id) {
            setDropTarget(null);
            return;
        }

        event.preventDefault();
        if (event.dataTransfer) event.dataTransfer.dropEffect = 'move';
        setDropTarget({ id: task.id, position: getDropPosition(event) });
    };

    const finishDragging = (): void => {
        setDraggingId(null);
        setDropTarget(null);
    };

    const dropTask = (event: DragEvent): void => {
        event.preventDefault();

        const target = dropTarget();
        const draggedId = draggingId() ?? Number(event.dataTransfer?.getData('text/plain'));
        if (target && Number.isFinite(draggedId)) {
            props.reorderTask(draggedId, target.id, target.position);
            props.click();
        }

        finishDragging();
    };

    const allowListDrop = (event: DragEvent): void => {
        if (draggingId() === null) return;

        event.preventDefault();
        if (event.dataTransfer) event.dataTransfer.dropEffect = 'move';
    };

    const taskRowClass = (task: Task): string => {
        const target = dropTarget();
        let className = 'task-row';
        if (draggingId() === task.id) className += ' task-row--dragging';
        if (target?.id === task.id) className += ` task-row--drop-${target.position}`;
        return className;
    };

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
            <ul
                class="task-list"
                ref={(element) => {
                    containerRef = element;
                }}
                onDragOver={allowListDrop}
                onDrop={dropTask}
            >
                <For each={props.tasks()}>
                    {(task) => (
                        <li
                            class={taskRowClass(task)}
                            onDragOver={(event) => updateDropTarget(event, task)}
                            onDrop={dropTask}
                        >
                            {editingId() !== task.id && (
                                <button
                                    type="button"
                                    class="task-drag-handle"
                                    draggable="true"
                                    title="Arrastar tarefa"
                                    aria-label="Arrastar tarefa"
                                    onDragStart={(event) => startDragging(event, task)}
                                    onDragEnd={finishDragging}
                                >
                                    <Icon name="grip" size={18} />
                                </button>
                            )}
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
                                    <TaskCheckbox
                                        task={task}
                                        onToggle={(id) => {
                                            props.toggleTask(id);
                                            props.click();
                                        }}
                                    />
                                </div>
                            )}
                        </li>
                    )}
                </For>
            </ul>
        </section>
    );
}
