import { createSignal, onCleanup } from 'solid-js';
import { loadSettings, mergeSettings, type Settings, saveSettings } from './model';

export const useSettings = () => {
    const [settings, setSettings] = createSignal<Settings>(loadSettings());
    let saveTimer: number | undefined;

    const scheduleSave = (nextSettings: Settings): void => {
        if (saveTimer !== undefined) window.clearTimeout(saveTimer);

        saveTimer = window.setTimeout(() => {
            saveTimer = undefined;
            saveSettings(nextSettings);
        }, 200);
    };

    const updateSettings = (changes: Partial<Settings>): void => {
        const nextSettings = mergeSettings(settings(), changes);
        setSettings(nextSettings);
        scheduleSave(nextSettings);
    };

    onCleanup(() => {
        if (saveTimer === undefined) return;

        window.clearTimeout(saveTimer);
        saveSettings(settings());
        saveTimer = undefined;
    });

    return {
        settings,
        updateSettings,
    };
};
