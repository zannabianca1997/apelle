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
                this.current = data.current ? new CurrentSong(data.current) : undefined;
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
    /** Moment at which the song should have started to reach the current position */
    private readonly _starts_at?: dayjs.Dayjs;
    /** Current position in the song */
    private readonly _position?: durationjs.Duration;

    constructor(data: CurrentSongQueryDto) {
        this.id = data.id;
        this.name = data.name;
        this.duration = dayjs.duration(data.duration);
        this.kind = data.kind;
        this.url = data.url;
        if (data.stopped) {
            this._starts_at = undefined;
            this._position = dayjs.duration(data.position);
        } else {
            this._starts_at = dayjs(data.starts_at);
            this._position = undefined;
        }
    }

    get stopped(): boolean {
        return !this._starts_at?.add(this.duration).isBefore(dayjs());
    }

    get starts_at(): dayjs.Dayjs {
        if (this._starts_at) {
            if (this.stopped) {
                return dayjs().subtract(this.duration);
            }
            return this._starts_at;
        }
        return dayjs().subtract(this._position!);
    }

    get position(): durationjs.Duration {
        if (this._position) {
            return this._position;
        }
        if (this.stopped) {
            return this.duration;
        }
        return dayjs.duration(dayjs().diff(this.starts_at));
    }
}
