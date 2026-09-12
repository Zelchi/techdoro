export type Settings = {
    volume: number;
    focusMinutes: number;
    shortBreakMinutes: number;
    longBreakMinutes: number;
    cyclesBeforeLongBreak: number;
};

export const defaultSettings: Settings = {
    volume: 30,
    focusMinutes: 25,
    shortBreakMinutes: 5,
    longBreakMinutes: 15,
    cyclesBeforeLongBreak: 4,
};

const storageKeys: Record<keyof Settings, string> = {
    volume: 'volume',
    focusMinutes: 'timeLongMax',
    shortBreakMinutes: 'timeShortMax',
    longBreakMinutes: 'timeFinalMax',
    cyclesBeforeLongBreak: 'cyclesBeforeFinal',
};

const getStorage = (): Storage | undefined => (typeof localStorage === 'undefined' ? undefined : localStorage);

const readNumber = (storage: Storage | undefined, key: string, fallback: number, minimum = 1): number => {
    const value = Number.parseInt(storage?.getItem(key) ?? '', 10);
    return Number.isFinite(value) && value >= minimum ? value : fallback;
};

export const loadSettings = (storage = getStorage()): Settings => ({
    volume: Math.min(100, readNumber(storage, storageKeys.volume, defaultSettings.volume, 0)),
    focusMinutes: readNumber(storage, storageKeys.focusMinutes, defaultSettings.focusMinutes),
    shortBreakMinutes: readNumber(storage, storageKeys.shortBreakMinutes, defaultSettings.shortBreakMinutes),
    longBreakMinutes: readNumber(storage, storageKeys.longBreakMinutes, defaultSettings.longBreakMinutes),
    cyclesBeforeLongBreak: readNumber(
        storage,
        storageKeys.cyclesBeforeLongBreak,
        defaultSettings.cyclesBeforeLongBreak,
    ),
});

export const saveSettings = (settings: Settings, storage = getStorage()): void => {
    if (!storage) return;

    storage.setItem(storageKeys.volume, String(settings.volume));
    storage.setItem(storageKeys.focusMinutes, String(settings.focusMinutes));
    storage.setItem(storageKeys.shortBreakMinutes, String(settings.shortBreakMinutes));
    storage.setItem(storageKeys.longBreakMinutes, String(settings.longBreakMinutes));
    storage.setItem(storageKeys.cyclesBeforeLongBreak, String(settings.cyclesBeforeLongBreak));
};

export const mergeSettings = (current: Settings, changes: Partial<Settings>): Settings => ({
    ...current,
    ...changes,
    volume: Math.max(0, Math.min(100, Math.trunc(changes.volume ?? current.volume))),
    focusMinutes: Math.max(1, Math.trunc(changes.focusMinutes ?? current.focusMinutes)),
    shortBreakMinutes: Math.max(1, Math.trunc(changes.shortBreakMinutes ?? current.shortBreakMinutes)),
    longBreakMinutes: Math.max(1, Math.trunc(changes.longBreakMinutes ?? current.longBreakMinutes)),
    cyclesBeforeLongBreak: Math.max(1, Math.trunc(changes.cyclesBeforeLongBreak ?? current.cyclesBeforeLongBreak)),
});
