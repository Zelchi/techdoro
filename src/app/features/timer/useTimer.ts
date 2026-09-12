import { type Accessor, createEffect, createSignal, on, onCleanup, untrack } from 'solid-js';
import type { Settings } from '../settings/model';
import { type ClockMap, createClockMap, createDurations, getNextSession, type Session } from './model';

type TimerOptions = {
    settings: Accessor<Settings>;
    onSessionFinished: (session: Session) => void;
};

export type TimerController = {
    session: Accessor<Session>;
    isRunning: Accessor<boolean>;
    timeNow: Accessor<number>;
    timeMax: Accessor<number>;
    toggle: () => void;
    reset: () => void;
    next: () => void;
};

export const useTimer = (options: TimerOptions): TimerController => {
    const [session, setSession] = createSignal<Session>(1);
    const [isRunning, setIsRunning] = createSignal(false);
    const [focusCyclesDone, setFocusCyclesDone] = createSignal(0);
    const [now, setNow] = createSignal(Date.now());
    const initialDurations = createDurations(options.settings());
    const [clocks, setClocks] = createSignal<ClockMap>(createClockMap(initialDurations));

    let startedAt: number | null = null;
    let interval: number | undefined;

    const stopTicker = (): void => {
        if (interval === undefined) return;
        window.clearInterval(interval);
        interval = undefined;
    };

    const remainingFor = (target: Session, timestamp = now()): number => {
        const state = clocks()[target];
        if (target === session() && isRunning() && startedAt !== null) {
            return Math.max(state.timeNow - Math.max(0, timestamp - startedAt) / 1000, 0);
        }
        return state.timeNow;
    };

    const commitElapsed = (timestamp = Date.now()): void => {
        if (startedAt === null) return;

        const target = session();
        const elapsed = Math.max(0, timestamp - startedAt) / 1000;
        setClocks((previous) => ({
            ...previous,
            [target]: {
                ...previous[target],
                timeNow: Math.max(previous[target].timeNow - elapsed, 0),
            },
        }));
        startedAt = timestamp;
    };

    const reset = (): void => {
        const target = session();
        const maximum = createDurations(options.settings())[target];
        setIsRunning(false);
        startedAt = null;
        stopTicker();
        setClocks((previous) => ({
            ...previous,
            [target]: { timeNow: maximum, timeMax: maximum },
        }));
    };

    const next = (): void => {
        if (startedAt !== null) commitElapsed();
        setIsRunning(false);
        startedAt = null;
        stopTicker();
        setSession(getNextSession);
    };

    const finish = (timestamp: number): void => {
        if (!isRunning() || startedAt === null) return;

        const current = session();
        const nextCycle = focusCyclesDone() + 1;
        let nextSession: Session;

        if (current === 1) {
            if (nextCycle >= options.settings().cyclesBeforeLongBreak) {
                nextSession = 3;
                setFocusCyclesDone(0);
            } else {
                nextSession = 2;
                setFocusCyclesDone(nextCycle);
            }
        } else {
            nextSession = 1;
        }

        const nextMaximum = createDurations(options.settings())[nextSession];
        setClocks((previous) => ({
            ...previous,
            [current]: {
                timeNow: previous[current].timeMax,
                timeMax: previous[current].timeMax,
            },
            [nextSession]: {
                timeNow: nextMaximum,
                timeMax: nextMaximum,
            },
        }));
        setIsRunning(false);
        startedAt = null;
        stopTicker();
        setSession(nextSession);
        setNow(timestamp);
        options.onSessionFinished(current);
    };

    createEffect(
        on(isRunning, (running) => {
            if (running) {
                if (startedAt === null) startedAt = Date.now();
                stopTicker();
                interval = window.setInterval(() => {
                    const timestamp = Date.now();
                    setNow(timestamp);
                    if (remainingFor(session(), timestamp) <= 0) finish(timestamp);
                }, 200);
                return;
            }

            untrack(() => {
                if (startedAt !== null) {
                    commitElapsed();
                    startedAt = null;
                }
                stopTicker();
            });
        }),
    );

    createEffect(
        on(options.settings, (currentSettings) => {
            untrack(() => {
                const maximums = createDurations(currentSettings);
                const timestamp = Date.now();

                if (startedAt !== null) {
                    commitElapsed(timestamp);
                    startedAt = timestamp;
                }

                setClocks((previous) => ({
                    1: {
                        timeNow: Math.min(previous[1].timeNow, maximums[1]),
                        timeMax: maximums[1],
                    },
                    2: {
                        timeNow: Math.min(previous[2].timeNow, maximums[2]),
                        timeMax: maximums[2],
                    },
                    3: {
                        timeNow: Math.min(previous[3].timeNow, maximums[3]),
                        timeMax: maximums[3],
                    },
                }));
            });
        }),
    );

    onCleanup(stopTicker);

    return {
        session,
        isRunning,
        timeNow: () => remainingFor(session()),
        timeMax: () => clocks()[session()].timeMax,
        toggle: () => setIsRunning((running) => !running),
        reset,
        next,
    };
};
