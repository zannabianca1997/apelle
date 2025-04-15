import type { CurrentSongQueryDto, QueueDeleteEventDto, QueuedSongShortQueryDto, QueueEventDto, QueueQueryDto, SongKind, Uuid } from "$lib/apis/apelle";
import { dayjs, durationjs } from "$lib/time";

export class Queue {
    /** Unique ID of the queue */
    id: Uuid;
    /** Unique code of the queue */
    code: string;
    /** The current playing song, if any */
    current?: CurrentSong;
    /** The songs in the queue */
    queue: QueuedSongShortQueryDto[];

    constructor(data: QueueQueryDto) {
        this.id = data.id;
        this.code = data.code;
        this.current = data.current ? new CurrentSong(data.current) : undefined;
        this.queue = data.queue;
    }

    public update(event: Exclude<QueueEventDto, QueueDeleteEventDto>) {
        switch (event.kind) {
            case "queue-state": {
                const data = event.queue;

                this.id = data.id;
                this.code = data.code;
                if (this.current) {
                    if (data.current) {
                        this.current.update(data.current)
                    } else {
                        this.current = undefined;
                    }
                } else {
                    this.current = data.current ? new CurrentSong(data.current) : undefined;
                }
                this.queue = data.queue;

                break;
            }
        }
    }
}

export class CurrentSong {
    /** Unique id of the song */
    id: Uuid;
    /** Name of the song */
    name: string;
    /** Duration of the song */
    duration: durationjs.Duration;
    /** Source of the song */
    kind: SongKind;
    /** Eventual public url of the song */
    url?: string;
    /** If the song is currently stopped */
    stopped: boolean;
    /** Moment at which the song should have started to reach the current position */
    starts_at: dayjs.Dayjs;
    /** Current position in the song */
    position: durationjs.Duration;

    constructor(data: CurrentSongQueryDto) {
        this.id = data.id;
        this.name = data.name;
        this.duration = dayjs.duration(data.duration);
        this.kind = data.kind;
        this.url = data.url;
        this.stopped = data.stopped;
        this.starts_at = dayjs(data.starts_at);
        this.position = dayjs.duration(data.position);
    }

    update(data: CurrentSongQueryDto) {
        this.id = data.id;
        this.name = data.name;
        this.duration = dayjs.duration(data.duration);
        this.kind = data.kind;
        this.url = data.url;
        this.stopped = data.stopped;
        this.starts_at = dayjs(data.starts_at);
        this.position = dayjs.duration(data.position);
    }
}
