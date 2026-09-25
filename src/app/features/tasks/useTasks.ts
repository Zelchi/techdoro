import { createEffect, createSignal, on } from 'solid-js';
import { getNextTaskId, loadTasks, saveTasks, type Task } from './model';

export const useTasks = () => {
    const [tasks, setTasks] = createSignal<Task[]>(loadTasks());

    createEffect(on(tasks, (currentTasks) => saveTasks(currentTasks)));

    const addTask = (text: string): boolean => {
        const normalizedText = text.trim();
        if (!normalizedText) return false;

        setTasks((previous) => [
            ...previous,
            {
                id: getNextTaskId(previous),
                text: normalizedText,
                completed: false,
            },
        ]);
        return true;
    };

    const deleteTask = (id: number): void => {
        setTasks((previous) => previous.filter((task) => task.id !== id));
    };

    const toggleTask = (id: number): void => {
        setTasks((previous) =>
            previous.map((task) => (task.id === id ? { ...task, completed: !task.completed } : task)),
        );
    };

    const updateTask = (id: number, text: string): void => {
        const normalizedText = text.trim();
        if (!normalizedText) return;

        setTasks((previous) => previous.map((task) => (task.id === id ? { ...task, text: normalizedText } : task)));
    };

    const reorderTask = (id: number, targetId: number, position: 'before' | 'after'): void => {
        if (id === targetId) return;

        setTasks((previous) => {
            const movedTask = previous.find((task) => task.id === id);
            if (!movedTask || !previous.some((task) => task.id === targetId)) return previous;

            const remainingTasks = previous.filter((task) => task.id !== id);
            const targetIndex = remainingTasks.findIndex((task) => task.id === targetId);
            const insertionIndex = position === 'after' ? targetIndex + 1 : targetIndex;

            remainingTasks.splice(insertionIndex, 0, movedTask);
            return remainingTasks;
        });
    };

    return {
        tasks,
        addTask,
        deleteTask,
        toggleTask,
        updateTask,
        reorderTask,
    };
};
