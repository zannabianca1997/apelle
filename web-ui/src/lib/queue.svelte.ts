import type { EventContent, Queue } from './apis/apelle';
import { source as getSource, type Source } from 'sveltekit-sse';
import authService from './auth.svelte';
import { Logger } from './logger';
import { applyPatch } from 'fast-json-patch';

const logger = new Logger('lib.queue');

export default class Connection {
	private readonly source: Source;
	public queue: Queue | null = $state(null);

	public constructor(queueId: string) {
		this.source = getSource(`/api/queues/${queueId}/events`, {
			options: {
				method: 'GET',
				headers: authService.headers
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
							logger.warn(`Got a patch event before a sync event`);
							return;
						}
						applyPatch(this.queue, event.value);
						return;
					default:
						const _: never = event;
						logger.error(`Unknown event kind: ${(event as any).kind}`, event);
						throw new Error(`Unknown event kind: ${(event as any).kind}`);
				}
			});
	}

	public close() {
		this.source.close();
		this.queue = null;
	}
}
