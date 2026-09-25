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

type PointerDrag = {
    id: number;
    pointerId: number;
    active: boolean;
};

export default function TaskList(props: TaskListProps) {
    const [newTask, setNewTask] = createSignal('');
    const [editingId, setEditingId] = createSignal<number | null>(null);
    const [editingText, setEditingText] = createSignal('');
    const [containerWidth, setContainerWidth] = createSignal(0);
    const [draggingId, setDraggingId] = createSignal<number | null>(null);
    const [dropTarget, setDropTarget] = createSignal<DropTarget | null>(null);
    let containerRef: HTMLUListElement | undefined;
    let pointerDrag: PointerDrag | null = null;

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

    const getDropTargetAtPoint = (clientX: number, clientY: number): DropTarget | null => {
        if (!containerRef || !pointerDrag) return null;

        const pointElement = document.elementFromPoint(clientX, clientY);
        const pointRow = pointElement?.closest<HTMLElement>('.task-row[data-task-id]');
        const isTaskRow = pointRow?.parentElement === containerRef;

        if (isTaskRow) {
            const targetId = Number(pointRow.dataset.taskId);
            if (targetId === pointerDrag.id) return null;

            const bounds = pointRow.getBoundingClientRect();
            return {
                id: targetId,
                position: clientY < bounds.top + bounds.height / 2 ? 'before' : 'after',
            };
        }

        const rows = Array.from(containerRef.querySelectorAll<HTMLElement>('.task-row[data-task-id]')).filter(
            (row) => Number(row.dataset.taskId) !== pointerDrag?.id,
        );

        for (const row of rows) {
            const bounds = row.getBoundingClientRect();
            if (clientY < bounds.top + bounds.height / 2) {
                return { id: Number(row.dataset.taskId), position: 'before' };
            }
        }

        const lastRow = rows.at(-1);
        return lastRow ? { id: Number(lastRow.dataset.taskId), position: 'after' } : null;
    };

    const startPointerDragging = (event: PointerEvent, task: Task): void => {
        if (event.button !== 0) return;

        event.preventDefault();
        event.stopPropagation();
        const handle = event.currentTarget as HTMLElement;
        handle.setPointerCapture(event.pointerId);
        pointerDrag = { id: task.id, pointerId: event.pointerId, active: false };
        setDraggingId(task.id);
        setDropTarget(null);
    };

    const updatePointerDropTarget = (event: PointerEvent): void => {
        if (!pointerDrag || pointerDrag.pointerId !== event.pointerId) return;

        event.preventDefault();
        pointerDrag.active = true;
        setDropTarget(getDropTargetAtPoint(event.clientX, event.clientY));
    };

    const finishPointerDragging = (event: PointerEvent, shouldDrop: boolean): void => {
        if (!pointerDrag || pointerDrag.pointerId !== event.pointerId) return;

        event.preventDefault();
        const handle = event.currentTarget as HTMLElement;
        const drag = pointerDrag;
        const target = shouldDrop ? getDropTargetAtPoint(event.clientX, event.clientY) : null;
        pointerDrag = null;

        if (handle.hasPointerCapture(event.pointerId)) {
            handle.releasePointerCapture(event.pointerId);
        }

        if (drag.active && target) {
            props.reorderTask(drag.id, target.id, target.position);
            props.click();
        }

        setDraggingId(null);
        setDropTarget(null);
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
            >
                <For each={props.tasks()}>
                    {(task) => (
                        <li
                            class={taskRowClass(task)}
                            data-task-id={task.id}
                        >
                            {editingId() !== task.id && (
                                <button
                                    type="button"
                                    class="task-drag-handle"
                                    title="Arrastar tarefa"
                                    aria-label="Arrastar tarefa"
                                    onPointerDown={(event) => startPointerDragging(event, task)}
                                    onPointerMove={updatePointerDropTarget}
                                    onPointerUp={(event) => finishPointerDragging(event, true)}
                                    onPointerCancel={(event) => finishPointerDragging(event, false)}
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
