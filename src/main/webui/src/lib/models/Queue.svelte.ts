import type {
	CurrentSongQueryDto,
	QueueDeleteEventDto,
	QueuedSongQueryDto,
	QueuedSongShortQueryDto,
	QueueEventDto,
	QueueQueryDto,
	SongKind,
	ThumbnailQueryDto,
	Uuid
} from '$lib/apis/apelle';
import { dayjs, durationjs } from '$lib/time';

import { getApiV1QueuesIQueueIdQueueSongId as getFullSong } from '$lib/apis/apelle';

export class Queue {
	/** Unique ID of the queue */
	readonly id: Uuid;
	/** Unique code of the queue */
	code: string = $state('');
	/** The current playing song, if any */
	current?: CurrentSong = $state();
	/** The songs in the queue */
	queue: QueuedSong[] = $state([]);
	/** Id of the current state of the player */
	player_state_id: Uuid = $state('000000000-0000-0000-0000-000000000000');

	constructor(id: Uuid) {
		this.id = id;
	}

	public async init(data: QueueQueryDto) {
		console.assert(this.id == data.id);

		const promises = [];

		this.code = data.code;
		this.player_state_id = data.player_state_id;
		promises.push(this.updateCurrent(data.current));
		promises.push(this.updateQueuedSongs(data.queue));

		await Promise.all(promises);
	}

	public async update(event: Exclude<QueueEventDto, QueueDeleteEventDto>) {
		const promises: Promise<void>[] = [];

		switch (event.kind) {
			case 'queue-state': {
				const data = event.queue;

				console.assert(event.queue.id === this.id);

				this.code = data.code;
				this.player_state_id = data.player_state_id;
				promises.push(this.updateCurrent(data.current));
				promises.push(this.updateQueuedSongs(data.queue));

				break;
			}

			case 'current-song-state':
				if (event.player_state_id) {
					this.player_state_id = event.player_state_id;
				}
				promises.push(this.updateCurrent(event.current));
				break;

			case 'queued-songs-state':
				promises.push(this.updateQueuedSongs(event.queue));
				break;

			default: {
				// This stops compilation if new events are added and not handled
				const exhaustiveCheck: never = event;
				throw new Error(`Unknown event kind ${(exhaustiveCheck as { kind: string }).kind} `);
			}
		}

		await Promise.all(promises);
	}

	private async updateQueuedSongs(data: QueuedSongShortQueryDto[]): Promise<void> {
		const promises: Promise<void>[] = [];

		const newQueue: QueuedSong[] = [];
		for (const song of data) {
			const loaded = this.queue.find((s) => s.id === song.id);
			if (loaded) {
				promises.push(loaded.update(this.id, song));
				newQueue.push(loaded);
			} else {
				const newSong = new QueuedSong(song.id);
				promises.push(newSong.init(this.id, song));
				newQueue.push(newSong);
			}
		}
		this.queue = newQueue;

		await Promise.all(promises);
	}

	private async updateCurrent(data: CurrentSongQueryDto | undefined): Promise<void> {
		this.current?.destroy();
		if (data) {
			this.current = new CurrentSong(data.id);
			await this.current.init(data);
		} else {
			this.current = undefined;
		}
	}
}

export class CurrentSong {
	/** Unique id of the song */
	readonly id: Uuid;
	/** Name of the song */
	name: string = $state('');
	/** Duration of the song */
	duration: durationjs.Duration = $state(dayjs.duration(0));
	/** Source of the song */
	kind: SongKind = $state('Youtube');
	/** Eventual public url of the song */
	url?: URL = $state();
	/** Available thumbnails for the song */
	thumbnails?: ThumbnailQueryDto[] = $state();
	/** If the song is currently stopped */
	stopped: boolean = $state(true);
	/** Moment at which the song should have started to reach the current position */
	starts_at: dayjs.Dayjs = $state(dayjs());
	/** Current position in the song */
	position: durationjs.Duration = $state(dayjs.duration(0));

	private animationFrame?: number;

	constructor(id: Uuid) {
		this.id = id;
	}

	async init(data: CurrentSongQueryDto) {
		console.assert(this.id === data.id);

		this.name = data.name;
		this.duration = dayjs.duration(data.duration);
		this.kind = data.kind;
		this.url = data.url ? new URL(data.url) : undefined;
		this.thumbnails = data.thumbnails;
		this.stopped = data.stopped;
		this.position = dayjs.duration(data.position);
		this.starts_at = dayjs(data.starts_at);

		if (!this.stopped) {
			this.setTimeout();
		}
	}

	private setTimeout() {
		const now = dayjs();

		this.position = dayjs.duration(now.diff(this.starts_at));

		if (this.position >= this.duration) {
			this.stopped = true;
		} else {
			this.animationFrame = window.requestAnimationFrame(() => this.setTimeout());
		}
	}

	destroy() {
		if (this.animationFrame) {
			window.cancelAnimationFrame(this.animationFrame);
		}
	}
}

type QueuedSongAdditionalDataDto = Omit<QueuedSongQueryDto, keyof QueuedSongShortQueryDto>;

export class QueuedSong {
	/** Unique id of the song */
	readonly id: Uuid;
	/** Name of the song */
	name: string = $state('');
	/** Duration of the song */
	duration?: durationjs.Duration = $state();
	/** Source of the song */
	kind?: SongKind = $state();
	/** Eventual public url of the song */
	url?: URL = $state();
	/** Available thumbnails for the song */
	thumbnails?: ThumbnailQueryDto[] = $state();
	/** The moment this song was added to the queue */
	queued_at: dayjs.Dayjs = $state(dayjs());
	/** The number of likes this song received */
	likes: number = $state(0);
	/** The number of likes this song received by this user */
	user_likes: number = $state(0);

	constructor(id: Uuid) {
		this.id = id;
	}

	async init(queueId: Uuid, song: QueuedSongShortQueryDto) {
		console.assert(this.id === song.id);

		this.name = song.name;
		this.duration = undefined;
		this.kind = undefined;
		this.url = undefined;
		this.thumbnails = undefined;
		this.queued_at = dayjs(song.queued_at);
		this.likes = song.likes;
		this.user_likes = song.user_likes;

		await this.hydrate(queueId);
	}

	async update(queueId: Uuid, song: QueuedSongShortQueryDto) {
		console.assert(this.id == song.id);

		this.name = song.name;
		this.queued_at = dayjs(song.queued_at);
		this.likes = song.likes;
		this.user_likes = song.user_likes;

		await this.hydrate(queueId);
	}

	/**
	 * Cache for the song data
	 *
	 * This cache store locally the song details and avoid to ask for them again
	 */
	static readonly fullSongCache: Map<string, QueuedSongAdditionalDataDto> = new Map();

	async hydrate(queueId: Uuid) {
		const cacheKey = `${this.id}@${queueId}`;
		let data: QueuedSongAdditionalDataDto | undefined = QueuedSong.fullSongCache.get(cacheKey);

		if (!data) {
			data = (await getFullSong(queueId, this.id)).data;
			QueuedSong.fullSongCache.set(cacheKey, data);
		}

		this.duration = dayjs.duration(data.duration);
		this.kind = data.kind;
		this.url = data.url ? new URL(data.url) : undefined;
		this.thumbnails = data.thumbnails;
	}
}
