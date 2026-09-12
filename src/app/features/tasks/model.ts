export type Task = {
    id: number;
    text: string;
    completed: boolean;
};

const STORAGE_KEY = 'tarefas';

const getStorage = (): Storage | undefined => (typeof localStorage === 'undefined' ? undefined : localStorage);

const isTask = (value: unknown): value is Task => {
    if (typeof value !== 'object' || value === null) return false;

    const task = value as Partial<Task>;
    return typeof task.id === 'number' && typeof task.text === 'string' && typeof task.completed === 'boolean';
};

export const loadTasks = (storage = getStorage()): Task[] => {
    if (!storage) return [];

    try {
        const saved = storage.getItem(STORAGE_KEY);
        const parsed: unknown = saved ? JSON.parse(saved) : [];
        return Array.isArray(parsed) ? parsed.filter(isTask) : [];
    } catch {
        return [];
    }
};

export const saveTasks = (tasks: Task[], storage = getStorage()): void => {
    storage?.setItem(STORAGE_KEY, JSON.stringify(tasks));
};

export const getNextTaskId = (tasks: Task[]): number =>
    tasks.reduce((highest, task) => Math.max(highest, task.id), 0) + 1;
