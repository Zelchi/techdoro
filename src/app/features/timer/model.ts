import type { Settings } from '../settings/model';

export type Session = 1 | 2 | 3;

export type ClockState = {
    timeNow: number;
    timeMax: number;
};

export type ClockMap = Record<Session, ClockState>;
export type DurationMap = Record<Session, number>;

export const createDurations = (settings: Settings): DurationMap => ({
    1: settings.focusMinutes * 60,
    2: settings.shortBreakMinutes * 60,
    3: settings.longBreakMinutes * 60,
});

export const createClockMap = (durations: DurationMap): ClockMap => ({
    1: { timeNow: durations[1], timeMax: durations[1] },
    2: { timeNow: durations[2], timeMax: durations[2] },
    3: { timeNow: durations[3], timeMax: durations[3] },
});

export const getNextSession = (session: Session): Session => (session >= 3 ? 1 : ((session + 1) as Session));
