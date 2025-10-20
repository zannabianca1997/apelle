import dayjs from 'dayjs';
import durationjs from 'dayjs/plugin/duration';
import { readable } from 'svelte/store';
dayjs.extend(durationjs);

/**
 * Current time, with a second precision
 */
const time = readable(dayjs(), (set) => {
    set(dayjs());

    const interval = setInterval(() => {
        set(dayjs());
    }, 1000);

    return () => clearInterval(interval);
});

export { dayjs, durationjs, time };
