import { type Accessor, createEffect, onCleanup, onMount } from 'solid-js';
import alarmSource from '../../assets/alarme.mp3';
import clickSource from '../../assets/click.mp3';

const sources = {
    click: clickSource,
    alarm: alarmSource,
} as const;

export type SoundName = keyof typeof sources;

export const useSound = (name: SoundName, volume: Accessor<number>): (() => void) => {
    let audio: HTMLAudioElement | undefined;

    onMount(() => {
        audio = new Audio(sources[name]);
        audio.volume = Math.max(0, Math.min(100, volume())) / 100;
    });

    createEffect(() => {
        const currentVolume = volume();
        if (audio) audio.volume = Math.max(0, Math.min(100, currentVolume)) / 100;
    });

    onCleanup(() => {
        audio?.pause();
        audio?.removeAttribute('src');
        audio?.load();
        audio = undefined;
    });

    return () => {
        if (!audio) return;
        audio.currentTime = 0;
        void audio.play().catch(() => undefined);
    };
};
