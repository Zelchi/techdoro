import { getCurrentWindow } from '@tauri-apps/api/window';

const getAppWindow = () => {
    if (typeof window === 'undefined' || !('__TAURI_INTERNALS__' in window)) return undefined;
    return getCurrentWindow();
};

export const minimizeWindow = (): void => {
    const appWindow = getAppWindow();
    if (appWindow) void appWindow.hide().catch(() => undefined);
};

export const startWindowDrag = (): void => {
    const appWindow = getAppWindow();
    if (appWindow) void appWindow.startDragging().catch(() => undefined);
};
