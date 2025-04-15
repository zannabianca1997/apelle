import type {
	CurrentSongQueryDto,
	QueueDeleteEventDto,
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
	id: Uuid = '00000000-0000-0000-0000-000000000000';
	/** Unique code of the queue */
	code: string = $state('');
	/** The current playing song, if any */
	current?: CurrentSong = $state();
	/** The songs in the queue */
	queue: QueuedSong[] = $state([]);

	public async init(data: QueueQueryDto) {
		const promises = [];

		this.id = data.id;
		this.code = data.code;

		this.current?.destroy();
		if (data.current) {
			this.current = new CurrentSong();
			promises.push(this.current.init(data.current));
		} else {
			this.current = undefined;
		}

		const newQueue = [];
		for (const song of data.queue) {
			const newSong = new QueuedSong();
			promises.push(newSong.init(this.id, song));
			newQueue.push(newSong);
		}
		this.queue = newQueue;

		await Promise.all(promises);
	}

	public async update(event: Exclude<QueueEventDto, QueueDeleteEventDto>) {
		const promises = [];

		switch (event.kind) {
			case 'queue-state': {
				const data = event.queue;

				console.assert(event.queue.id === this.id);

				this.code = data.code;

				this.current?.destroy();
				if (data.current) {
					this.current = new CurrentSong();
					promises.push(this.current.init(data.current));
				} else {
					this.current = undefined;
				}

				const newQueue = [];
				for (const song of data.queue) {
					const loaded = this.queue.find((s) => s.id === song.id);
					if (loaded) {
						promises.push(loaded.update(this.id, song));
						newQueue.push(loaded);
					} else {
						const newSong = new QueuedSong();
						promises.push(newSong.init(this.id, song));
						newQueue.push(newSong);
					}
				}
				this.queue = newQueue;

				break;
			}
		}

		await Promise.all(promises);
	}
}

export class CurrentSong {
	/** Unique id of the song */
	id: Uuid = '00000000-0000-0000-0000-000000000000';
	/** Name of the song */
	name: string = $state('');
	/** Duration of the song */
	duration: durationjs.Duration = $state(dayjs.duration(0));
	/** Source of the song */
	kind: SongKind = $state('Youtube');
	/** Eventual public url of the song */
	url?: string = $state();
	/** Available thumbnails for the song */
	thumbnails?: ThumbnailQueryDto[] = $state();
	/** If the song is currently stopped */
	stopped: boolean = $state(true);
	/** Moment at which the song should have started to reach the current position */
	starts_at: dayjs.Dayjs = $state(dayjs());
	/** Current position in the song */
	position: durationjs.Duration = $state(dayjs.duration(0));

	private animationFrame?: number;

	async init(data: CurrentSongQueryDto) {
		this.id = data.id;
		this.name = data.name;
		this.duration = dayjs.duration(data.duration);
		this.kind = data.kind;
		this.url = data.url;
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

export class QueuedSong {
	/** Unique id of the song */
	id: Uuid = '00000000-0000-0000-0000-000000000000';
	/** Name of the song */
	name: string = $state('');
	/** Duration of the song */
	duration?: durationjs.Duration = $state(dayjs.duration(0));
	/** Source of the song */
	kind?: SongKind = $state('Youtube');
	/** Eventual public url of the song */
	url?: string = $state();
	/** Available thumbnails for the song */
	thumbnails?: ThumbnailQueryDto[] = $state();
	/** The moment this song was added to the queue */
	queued_at: dayjs.Dayjs = $state(dayjs());
	/** The number of likes this song received */
	likes: number = $state(0);
	/** The number of likes this song received by this user */
	user_likes?: number = $state();

	async init(queueId: Uuid, song: QueuedSongShortQueryDto) {
		this.id = song.id;
		this.name = song.name;
		this.duration = undefined;
		this.kind = undefined;
		this.url = undefined;
		this.thumbnails = undefined;
		this.queued_at = dayjs(song.queued_at);
		this.likes = song.likes;
		this.user_likes = undefined;

		await this.hydrate(queueId);
	}

	async update(queueId: Uuid, song: QueuedSongShortQueryDto) {
		console.assert(this.id == song.id);

		this.name = song.name;
		this.likes = song.likes;
		this.queued_at = dayjs(song.queued_at);

		await this.hydrate(queueId);
	}

	async hydrate(queueId: Uuid) {
		const { data } = await getFullSong(queueId, this.id);

		this.name = data.name;
		this.duration = dayjs.duration(data.duration);
		this.kind = data.kind;
		this.url = data.url;
		this.thumbnails = data.thumbnails;
		this.queued_at = dayjs(data.queued_at);
		this.likes = data.likes;
		this.user_likes = data.user_likes;
	}
}
