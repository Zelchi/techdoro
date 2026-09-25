export type IconName =
    | 'check'
    | 'close'
    | 'grip'
    | 'next'
    | 'pause'
    | 'play'
    | 'plus'
    | 'reset'
    | 'settings'
    | 'thumbsup'
    | 'trash'
    | 'volume-down'
    | 'volume-mute'
    | 'volume-up';

type IconProps = {
    name: IconName;
    size?: number;
};

export default function Icon(props: IconProps) {
    const size = () => props.size ?? 18;

    return (
        <svg
            width={size()}
            height={size()}
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
        >
            {props.name === 'check' && <path d="m5 12 4 4L19 6" />}
            {props.name === 'close' && (
                <>
                    <path d="M6 6 18 18" />
                    <path d="m18 6-12 12" />
                </>
            )}
            {props.name === 'grip' && (
                <>
                    <circle cx="8" cy="7" r="1" />
                    <circle cx="16" cy="7" r="1" />
                    <circle cx="8" cy="12" r="1" />
                    <circle cx="16" cy="12" r="1" />
                    <circle cx="8" cy="17" r="1" />
                    <circle cx="16" cy="17" r="1" />
                </>
            )}
            {props.name === 'next' && (
                <>
                    <path d="m5 4 10 8-10 8V4Z" />
                    <path d="M19 5v14" />
                </>
            )}
            {props.name === 'pause' && (
                <>
                    <rect x="6" y="4" width="4" height="16" rx="1" />
                    <rect x="14" y="4" width="4" height="16" rx="1" />
                </>
            )}
            {props.name === 'play' && <path d="m7 4 13 8-13 8V4Z" />}
            {props.name === 'plus' && (
                <>
                    <path d="M12 5v14" />
                    <path d="M5 12h14" />
                </>
            )}
            {props.name === 'reset' && (
                <>
                    <path d="M3 12a9 9 0 1 0 3-6.7" />
                    <path d="M3 4v6h6" />
                </>
            )}
            {props.name === 'settings' && (
                <>
                    <path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0L6.2 6.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.38a2 2 0 0 0-.73-2.73l-.15-.09a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2Z" />
                    <circle cx="12" cy="12" r="3" />
                </>
            )}
            {props.name === 'thumbsup' && (
                <path d="M7 10v10H4a1 1 0 0 1-1-1v-8a1 1 0 0 1 1-1h3Zm0 10h9.2a2 2 0 0 0 1.9-1.4l2-6A2 2 0 0 0 18.2 10H14l.6-3.2A2.3 2.3 0 0 0 12.3 4L7 10v10Z" />
            )}
            {props.name === 'trash' && (
                <>
                    <path d="M4 7h16" />
                    <path d="M10 11v6" />
                    <path d="M14 11v6" />
                    <path d="m6 7 1 13h10l1-13" />
                    <path d="M9 7V4h6v3" />
                </>
            )}
            {props.name === 'volume-mute' && (
                <>
                    <path d="M11 5 6 9H3v6h3l5 4V5Z" />
                    <path d="m17 9 4 6" />
                    <path d="m21 9-4 6" />
                </>
            )}
            {props.name === 'volume-down' && (
                <>
                    <path d="M11 5 6 9H3v6h3l5 4V5Z" />
                    <path d="M15 10a3 3 0 0 1 0 4" />
                </>
            )}
            {props.name === 'volume-up' && (
                <>
                    <path d="M11 5 6 9H3v6h3l5 4V5Z" />
                    <path d="M15 9a5 5 0 0 1 0 6" />
                    <path d="M18 6a9 9 0 0 1 0 12" />
                </>
            )}
        </svg>
    );
}
