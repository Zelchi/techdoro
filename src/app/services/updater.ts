import { relaunch } from '@tauri-apps/plugin-process';
import { check } from '@tauri-apps/plugin-updater';

export async function checkForUpdates(): Promise<void> {
    if (!import.meta.env.PROD) return;

    try {
        const update = await check();

        if (!update) return;

        await update.downloadAndInstall();
        await relaunch();
    } catch (error) {
        console.warn('Não foi possível atualizar o Techdoro.', error);
    }
}
