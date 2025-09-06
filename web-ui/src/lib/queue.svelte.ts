import type { EventContent, Queue } from "./apis/apelle";
import { source, type Source } from 'sveltekit-sse';
import authService from "./auth.svelte";
import { Logger } from "./logger";
import { applyPatch } from "fast-json-patch";

const logger = new Logger('lib.queue');

let state: { source: Source, queue?: Queue } | null = $state(null);
export let queue: Queue | null = $derived.by(() => state?.queue ?? null);

export function connect(queueId: string) {
    state = {
        source: source(`/api/queues/${queueId}/events`, {
            options: {
                method: 'GET',
                headers: authService.headers
            }
        })
    }

    state?.source
        .select('')
        .json<EventContent | null>()
        .subscribe((event) => {
            if (!event || !state) {
                return;
            }

            logger.info(`Received queue event: %o`, event);

            switch (event.kind) {
                case 'Deleted':
                    state.source.close();
                    state = null;
                    return;
                case 'Sync':
                    state.queue = event.value;
                    return;
                case 'Patch':
                    if (!state.queue) {
                        logger.warn(`Got a patch event before a sync event`);
                        return;
                    }
                    applyPatch(state.queue, event.value);
                    return;
                default:
                    const _: never = event;
                    logger.error(`Unknown event kind: ${(event as any).kind}`, event);
                    throw new Error(`Unknown event kind: ${(event as any).kind}`);
            }
        })
}

export function disconnect() {
    state?.source.close();
    state = null;
}