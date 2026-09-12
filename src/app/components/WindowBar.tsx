import icon from '../../assets/icon.png';
import { minimizeWindow, startWindowDrag } from '../services/window';
import Icon from './Icon';

const isInteractiveTarget = (target: EventTarget | null): boolean =>
    target instanceof Element && Boolean(target.closest('button, a, input, select, textarea'));

export default function WindowBar() {
    const handleMouseDown = (event: MouseEvent): void => {
        if (event.button !== 0 || isInteractiveTarget(event.target)) return;

        event.preventDefault();
        startWindowDrag();
    };

    const preventDoubleClick = (event: MouseEvent): void => {
        event.preventDefault();
        event.stopPropagation();
    };

    return (
        <header
            class="window-bar"
            role="toolbar"
            data-tauri-drag-region
            onMouseDown={handleMouseDown}
            onDblClick={preventDoubleClick}
        >
            <div class="window-bar__side window-bar__side--left">
                <span class="window-icon" aria-hidden="true">
                    <img src={icon} alt="" draggable="false" />
                </span>
            </div>
            <div class="window-title">Techdoro</div>
            <div class="window-bar__side window-bar__side--right">
                <button class="window-button" type="button" onClick={minimizeWindow}>
                    <Icon name="close" size={16} />
                </button>
            </div>
        </header>
    );
}
