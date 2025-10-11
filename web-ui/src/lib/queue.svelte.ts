import type { EventContent, Queue } from './apis/apelle';
import { source as getSource, type Source, type Event } from 'sveltekit-sse';
import authService from './auth.svelte';
import { Logger } from './logger';
import { applyPatch } from 'fast-json-patch';

const logger = new Logger('lib.queue');


export default class Connection {
    private readonly source: Source;
    public queue: Queue | null = $state(null);

    public constructor(queueId: string, notFound?: () => void) {
        this.source = getSource(`/api/queues/${queueId}/events`, {
            options: {
                method: 'GET',
                headers: authService.headers,
            },
            open() {
                logger.info(`Successfully connected to queue ${queueId}`)
            },
            close(event: Event) {
                if (event.isLocal || (event.status >= 200 && event.status < 300)) {
                    // Normal shutdown
                    logger.info(`Disconnetting from queue ${queueId}`, event)
                }
                logger.warn(`Unexpected closure from queue ${queueId}`, event)
                switch (event.status) {
                    case 404: notFound?.()
                }
            },
            error(event: Event) {
                logger.error(`Error from queue ${queueId}`, event)
            }
        });

        this.source
            .select('')
            .json<EventContent | null>()
            .subscribe((event) => {
                if (!event || !this.source) {
                    return;
                }

                logger.info(`Received queue event: %o`, event);

                switch (event.kind) {
                    case 'Deleted':
                        this.close();
                        return;
                    case 'Sync':
                        this.queue = event.value;
                        return;
                    case 'Patch':
                        if (!this.queue) {
                            logger.warn(
                                `Got a patch event before a sync event`
                            );
                            return;
                        }
                        applyPatch(this.queue, event.value);
                        return;
                    default:
                        const _: never = event;
                        logger.error(
                            `Unknown event kind: ${(event as any).kind}`,
                            event
                        );
                        throw new Error(
                            `Unknown event kind: ${(event as any).kind}`
                        );
                }
            });
    }

    public close() {
        this.source.close();
        this.queue = null;
    }
}
