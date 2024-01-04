import { useEffect } from "react";

export const AutoFullscreen = (): null => {
    useEffect(() => {
        /** Check if the document is in fullscreen mode. */
        const isFullscreen = () => document.fullscreenElement !== null;

        /** Timer to exit fullscreen mode after a period of inactivity. */
        let inactivityTimer: ReturnType<typeof setTimeout> | null = null;
        const resetTimer = () => {
            if (inactivityTimer !== null) {
                clearTimeout(inactivityTimer);
            }

            inactivityTimer = setTimeout(() => {
                if (isFullscreen()) {
                    document.exitFullscreen().catch(console.error);
                }
            }, 3 * 1000);
        };

        /** Listen for input events to reset the timer. */
        const inputListener = () => {
            resetTimer();

            if (isFullscreen()) {
                return;
            }

            document.documentElement.requestFullscreen().catch(console.error);
        };

        // The events to listen for on the document
        const events = ["click", "keydown"];

        // Add the event listeners
        for (const event of events) {
            document.addEventListener(event, inputListener);
        }

        // Remove them when the component unmounts
        return () => {
            for (const event of events) {
                document.removeEventListener(event, inputListener);
            }
        };
    }, []);

    return null;
};
