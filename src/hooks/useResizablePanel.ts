import {useCallback, useEffect, useState} from "react";
import type {PointerEvent as ReactPointerEvent} from "react";

interface UseResizablePanelOptions {
    initialWidth: number;
    minWidth: number;
    maxWidth: number;
    offsetLeft?: number;
}

export function useResizablePanel(
    {
        initialWidth,
        minWidth,
        maxWidth,
        offsetLeft = 0,
    }: UseResizablePanelOptions) {
    const [width, setWidth] = useState(initialWidth);
    const [dragging, setDragging] = useState(false);

    useEffect(() => {
        if (!dragging) return;

        const onPointerMove = (event: PointerEvent) => {
            // Clamp the panel so drag gestures cannot steal the whole workspace.
            const newWidth = Math.min(maxWidth, Math.max(minWidth, event.clientX - offsetLeft));
            setWidth(newWidth);
        };

        const onPointerUp = () => setDragging(false);

        window.addEventListener("pointermove", onPointerMove);
        window.addEventListener("pointerup", onPointerUp);

        return () => {
            window.removeEventListener("pointermove", onPointerMove);
            window.removeEventListener("pointerup", onPointerUp);
        };
    }, [dragging, maxWidth, minWidth, offsetLeft]);

    const startResize = useCallback((event: ReactPointerEvent) => {
        event.preventDefault();
        setDragging(true);
    }, []);

    return {
        width,
        dragging,
        resizeHandleProps: {
            onPointerDown: startResize,
        },
    };
}