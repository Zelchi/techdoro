import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/plugin-notification';

let permissionRequest: Promise<boolean> | undefined;

const hasTauriRuntime = (): boolean => typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;

const ensurePermission = async (): Promise<boolean> => {
    if (!hasTauriRuntime()) return false;
    if (!permissionRequest) {
        permissionRequest = (async () => {
            if (await isPermissionGranted()) return true;
            return (await requestPermission()) === 'granted';
        })().catch(() => false);
    }
    return permissionRequest;
};

export const notifyTimerFinished = async (isFocus: boolean): Promise<void> => {
    if (!(await ensurePermission())) return;

    sendNotification({
        title: isFocus ? 'Tempo acabou!' : 'Intervalo acabou!',
        body: isFocus ? 'Vai dar uma esticada nas pernas!' : 'Retome os estudos imediatamente!!!',
    });
};
