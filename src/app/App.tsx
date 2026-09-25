import { createSignal, onMount } from 'solid-js';
import WindowBar from './components/WindowBar';
import SettingsPanel from './features/settings/components/SettingsPanel';
import { useSettings } from './features/settings/useSettings';
import TaskList from './features/tasks/components/TaskList';
import { useTasks } from './features/tasks/useTasks';
import TimerFrame from './features/timer/components/TimerFrame';
import { useTimer } from './features/timer/useTimer';
import { notifyTimerFinished } from './services/notifications';
import { useSound } from './services/sound';
import { checkForUpdates } from './services/updater';

export default function App() {
    onMount(() => {
        void checkForUpdates();
    });

    const settings = useSettings();
    const tasks = useTasks();
    const [settingsOpen, setSettingsOpen] = createSignal(false);
    const click = useSound('click', () => settings.settings().volume);
    const alarm = useSound('alarm', () => settings.settings().volume);
    const timer = useTimer({
        settings: settings.settings,
        onSessionFinished: (session) => {
            alarm();
            void notifyTimerFinished(session === 1);
        },
    });

    return (
        <main class="app-shell">
            <WindowBar />
            <TimerFrame timer={timer} onToggleSettings={() => setSettingsOpen((open) => !open)} click={click} />
            <SettingsPanel
                settings={settings.settings}
                updateSettings={settings.updateSettings}
                onClose={() => setSettingsOpen(false)}
                click={click}
                isOpen={settingsOpen()}
            />
            <TaskList
                tasks={tasks.tasks}
                addTask={tasks.addTask}
                deleteTask={tasks.deleteTask}
                toggleTask={tasks.toggleTask}
                updateTask={tasks.updateTask}
                reorderTask={tasks.reorderTask}
                click={click}
            />
        </main>
    );
}
