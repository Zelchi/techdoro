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

    return {
        tasks,
        addTask,
        deleteTask,
        toggleTask,
        updateTask,
    };
};
