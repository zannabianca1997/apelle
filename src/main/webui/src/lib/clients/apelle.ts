/**
 * Generated by orval v7.7.0 🍺
 * Do not edit manually.
 * Apelle API (development)
 * > A communist music queue

`apelle` is a backend for handling a shared music queue. 
Users can insert songs in the queues, and upvote them to push them upward. `apelle`
will track the position of each song in the queue, and the position of the currently
playing song.

It also fetch the song data from the sources (for now, only Youtube is supported).
Users provides only the minimal necessary to identify the song (e.g. the youtube video ID).

### Websockets
To avoid polling the REST API, `apelle` provide a websocket interface to each queue.
The relative URL is `/queues/{queueId}/ws`. It needs basic auth to connect.

The websocket does not listen for now to any message, but sends JSON messages at each queue change.
See the `ServerMessage` schema for the schema.
 * OpenAPI spec version: 0.0.1
 */
import axios from 'axios';
import type { AxiosRequestConfig, AxiosResponse } from 'axios';

export type ApelleUserRole = (typeof ApelleUserRole)[keyof typeof ApelleUserRole];

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const ApelleUserRole = {
	ADMIN: 'ADMIN',
	USER: 'USER'
} as const;

/**
 * The song currently being played
 */
export interface CurrentSongQueryDto {
	/** Unique id of the song */
	id: Uuid;
	/** Name of the song */
	name: string;
	/** Duration of the song */
	duration: Duration;
	/** Source of the song */
	kind: SongKind;
	/** Eventual public url of the song */
	url?: string;
	/** If the song is currently stopped */
	stopped: boolean;
	/** Moment at which the song should have started to reach the current position */
	starts_at: Instant;
	/** Current position in the song */
	position: Duration;
}

export type Duration = string;

export type Instant = string;

export interface Permissions {
	[key: string]: unknown;
}

/**
 * A queue of songs
 */
export interface QueueQueryDto {
	/** Unique ID of the queue */
	id: Uuid;
	/** The current playing song, if any */
	current?: CurrentSongQueryDto;
	/** The songs in the queue */
	queue: QueuedSongShortQueryDto[];
}

export type QueueStateMessageKind =
	(typeof QueueStateMessageKind)[keyof typeof QueueStateMessageKind];

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const QueueStateMessageKind = {
	'queue-state': 'queue-state'
} as const;

/**
 * An authoritative broadcast of the queue state.

After receiving this message a client must assume the queue is in the provided state.
 */
export interface QueueStateMessage {
	kind?: QueueStateMessageKind;
	queue?: QueueQueryDto;
}

/**
 * Data about a user of a queue
 */
export interface QueueUserQueryDto {
	/** Unique ID of the user */
	id: Uuid;
	/** Unique username of the user */
	name: string;
	/** Comma separated list of roles the user has */
	roles: ApelleUserRole[];
	/** Role of the user in the queue */
	queue_role: QueueUserRole;
	/** Number of likes given in the queue */
	likes: number;
	/** Maximum number of likes that can be given */
	max_likes: number;
}

export interface QueueUserRole {
	name?: string;
	config?: QueueUserRoleConfig;
	maxLikes?: number;
	permissions?: Permissions;
}

export interface QueueUserRoleConfig {
	[key: string]: unknown;
}

/**
 * Full description of a song inside a queue
 */
export interface QueuedSongQueryDto {
	/** Unique id of the song */
	id: Uuid;
	/** Name of the song */
	name: string;
	/** Duration of the song */
	duration: Duration;
	/** Source of the song */
	kind: SongKind;
	/** Eventual public url of the song */
	url?: string;
	/** The number of likes this song received */
	likes: number;
}

/**
 * A song inside a queue
 */
export interface QueuedSongShortQueryDto {
	/** Unique id of the song */
	id: Uuid;
	/** Name of the song */
	name: string;
	/** Duration of the song */
	duration: Duration;
	/** Source of the song */
	kind: SongKind;
	/** Eventual public url of the song */
	url?: string;
	/** The number of likes this song received */
	likes: number;
}

/**
 * A message from the server.

The `kind` property discriminates between the different messages.
 */
export type ServerMessage = QueueStateMessage | UnknowQueueMessage;

/**
 * Data defining a song to add
 */
export type SongAddDto = YoutubeSongAddDto;

export type SongKind = (typeof SongKind)[keyof typeof SongKind];

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const SongKind = {
	Youtube: 'Youtube'
} as const;

/**
 * A song
 */
export interface SongQueryDto {
	/** Unique id of the song */
	id: Uuid;
	/** Name of the song */
	name: string;
	/** Duration of the song */
	duration: Duration;
	/** Source of the song */
	kind: SongKind;
	/** Eventual public url of the song */
	url?: string;
}

/**
 * @pattern [a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}
 */
export type Uuid = string;

export type UnknowQueueMessageKind =
	(typeof UnknowQueueMessageKind)[keyof typeof UnknowQueueMessageKind];

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const UnknowQueueMessageKind = {
	'unknow-queue': 'unknow-queue'
} as const;

/**
 * The queue id is invalid.

Either the queue does not exist, it was cancelled.

After this message the socket will be closed.

 */
export interface UnknowQueueMessage {
	kind?: UnknowQueueMessageKind;
	queueId?: string;
}

/**
 * User creation data
 */
export interface UserCreateDto {
	/** Unique username for the user */
	name: string;
	/** Password for the user */
	password: string;
}

/**
 * Data about a single user
 */
export interface UserQueryDto {
	/** Unique ID of the user */
	id: Uuid;
	/** Unique username of the user */
	name: string;
	/** Comma separated list of roles the user has */
	roles: ApelleUserRole[];
}

export type YoutubeSongAddDtoKind =
	(typeof YoutubeSongAddDtoKind)[keyof typeof YoutubeSongAddDtoKind];

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const YoutubeSongAddDtoKind = {
	Youtube: 'Youtube'
} as const;

/**
 * A song that comes from youtube
 */
export interface YoutubeSongAddDto {
	kind: YoutubeSongAddDtoKind;
	/** The video ID */
	video_id: string;
}

export type PostQueuesQueueIdQueueSongIdLikesParams = {
	/**
	 * How many time to like the song. If negative, nothing will happen.
	 */
	count?: number;
};

/**
 * Create a new queue without any song inside it
 * @summary Create a new queue
 */
export const postQueues = <TData = AxiosResponse<QueueQueryDto>>(
	options?: AxiosRequestConfig
): Promise<TData> => {
	return axios.post(`/queues`, undefined, options);
};

/**
 * Get the queue state, with both the currently playing song and the list of songs to play next
 * @summary Get the queue state
 */
export const getQueuesQueueId = <TData = AxiosResponse<QueueQueryDto>>(
	queueId: Uuid,
	options?: AxiosRequestConfig
): Promise<TData> => {
	return axios.get(`/queues/${queueId}`, options);
};

/**
 * Start the next song in the queue.
The current one will be requeued as the last one, with no likes.
 * @summary Start playing the next song
 */
export const postQueuesQueueIdNext = <TData = AxiosResponse<void>>(
	queueId: Uuid,
	options?: AxiosRequestConfig
): Promise<TData> => {
	return axios.post(`/queues/${queueId}/next`, undefined, options);
};

/**
 * Add a song to the queue, with no likes.
 * @summary Add a song to the queue
 */
export const postQueuesQueueIdQueue = <TData = AxiosResponse<QueuedSongShortQueryDto>>(
	queueId: Uuid,
	songAddDto: SongAddDto,
	options?: AxiosRequestConfig
): Promise<TData> => {
	return axios.post(`/queues/${queueId}/queue`, songAddDto, options);
};

/**
 * Get the full state of the queued song, with all data.

TODO: Add query parameters to ask for thumbnails.
 * @summary Get the queued song
 */
export const getQueuesQueueIdQueueSongId = <TData = AxiosResponse<QueuedSongQueryDto>>(
	queueId: Uuid,
	songId: Uuid,
	options?: AxiosRequestConfig
): Promise<TData> => {
	return axios.get(`/queues/${queueId}/queue/${songId}`, options);
};

/**
 * Add a like to the song, pushing it upwards in the queue.

If the maximum number of likes was already reached, the oldest like will be removed.
This will happen trasparently even if a number of likes larger than available is specified,
effectively removing all likes and moving them to the song.
 * @summary Add a like to the song
 */
export const postQueuesQueueIdQueueSongIdLikes = <TData = AxiosResponse<void>>(
	queueId: Uuid,
	songId: Uuid,
	params?: PostQueuesQueueIdQueueSongIdLikesParams,
	options?: AxiosRequestConfig
): Promise<TData> => {
	return axios.post(`/queues/${queueId}/queue/${songId}/likes`, undefined, {
		...options,
		params: { ...params, ...options?.params }
	});
};

/**
 * Start playing music from the queue.
 * @summary Start playing
 */
export const postQueuesQueueIdStart = <TData = AxiosResponse<void>>(
	queueId: Uuid,
	options?: AxiosRequestConfig
): Promise<TData> => {
	return axios.post(`/queues/${queueId}/start`, undefined, options);
};

/**
 * Stop playing music from the queue.
 * @summary Stop playing
 */
export const postQueuesQueueIdStop = <TData = AxiosResponse<void>>(
	queueId: Uuid,
	options?: AxiosRequestConfig
): Promise<TData> => {
	return axios.post(`/queues/${queueId}/stop`, undefined, options);
};

/**
 * Get the state of a queue user by id, with role and likes data.
 * @summary Get a queue user by id
 */
export const getQueuesQueueIdUsersIUserId = <TData = AxiosResponse<QueueUserQueryDto>>(
	queueId: Uuid,
	userId: Uuid,
	options?: AxiosRequestConfig
): Promise<TData> => {
	return axios.get(`/queues/${queueId}/users/i/${userId}`, options);
};

/**
 * Get the state of the current queue user, with role and likes data.
 * @summary Get the current queue user
 */
export const getQueuesQueueIdUsersMe = <TData = AxiosResponse<QueueUserQueryDto>>(
	queueId: Uuid,
	options?: AxiosRequestConfig
): Promise<TData> => {
	return axios.get(`/queues/${queueId}/users/me`, options);
};

/**
 * Get the state of a queue user by name, with role and likes data.
 * @summary Get a queue user by name
 */
export const getQueuesQueueIdUsersNUserName = <TData = AxiosResponse<QueueUserQueryDto>>(
	queueId: Uuid,
	userName: string,
	options?: AxiosRequestConfig
): Promise<TData> => {
	return axios.get(`/queues/${queueId}/users/n/${userName}`, options);
};

/**
 * Create an user that can access queues and vote on them
 * @summary Create a user
 */
export const postUsers = <TData = AxiosResponse<UserQueryDto>>(
	userCreateDto: UserCreateDto,
	options?: AxiosRequestConfig
): Promise<TData> => {
	return axios.post(`/users`, userCreateDto, options);
};

/**
 * Find a user by their universal id
 * @summary Find user by id
 */
export const getUsersIUserId = <TData = AxiosResponse<UserQueryDto>>(
	userId: Uuid,
	options?: AxiosRequestConfig
): Promise<TData> => {
	return axios.get(`/users/i/${userId}`, options);
};

/**
 * Delete a user by id. Need to have the role `admin`
 * @summary Delete a user by id
 */
export const deleteUsersIUserId = <TData = AxiosResponse<void>>(
	userId: Uuid,
	options?: AxiosRequestConfig
): Promise<TData> => {
	return axios.delete(`/users/i/${userId}`, options);
};

/**
 * Returns the data of the user that is currently logged in
 * @summary Current user data
 */
export const getUsersMe = <TData = AxiosResponse<UserQueryDto>>(
	options?: AxiosRequestConfig
): Promise<TData> => {
	return axios.get(`/users/me`, options);
};

/**
 * Delete the current user
 * @summary Delete current user
 */
export const deleteUsersMe = <TData = AxiosResponse<void>>(
	options?: AxiosRequestConfig
): Promise<TData> => {
	return axios.delete(`/users/me`, options);
};

/**
 * Find a user by their username
 * @summary Find user by name
 */
export const getUsersNUserName = <TData = AxiosResponse<UserQueryDto>>(
	userName: string,
	options?: AxiosRequestConfig
): Promise<TData> => {
	return axios.get(`/users/n/${userName}`, options);
};

/**
 * Delete a user by name. Need to have the role `admin`
 * @summary Delete a user by name
 */
export const deleteUsersNUserName = <TData = AxiosResponse<void>>(
	userName: string,
	options?: AxiosRequestConfig
): Promise<TData> => {
	return axios.delete(`/users/n/${userName}`, options);
};

/**
 * Return the version of the server
 * @summary Version of the server
 */
export const getVersion = <TData = AxiosResponse<unknown>>(
	options?: AxiosRequestConfig
): Promise<TData> => {
	return axios.get(`/version`, options);
};

export type PostQueuesResult = AxiosResponse<QueueQueryDto>;
export type GetQueuesQueueIdResult = AxiosResponse<QueueQueryDto>;
export type PostQueuesQueueIdNextResult = AxiosResponse<void>;
export type PostQueuesQueueIdQueueResult = AxiosResponse<QueuedSongShortQueryDto>;
export type GetQueuesQueueIdQueueSongIdResult = AxiosResponse<QueuedSongQueryDto>;
export type PostQueuesQueueIdQueueSongIdLikesResult = AxiosResponse<void>;
export type PostQueuesQueueIdStartResult = AxiosResponse<void>;
export type PostQueuesQueueIdStopResult = AxiosResponse<void>;
export type GetQueuesQueueIdUsersIUserIdResult = AxiosResponse<QueueUserQueryDto>;
export type GetQueuesQueueIdUsersMeResult = AxiosResponse<QueueUserQueryDto>;
export type GetQueuesQueueIdUsersNUserNameResult = AxiosResponse<QueueUserQueryDto>;
export type PostUsersResult = AxiosResponse<UserQueryDto>;
export type GetUsersIUserIdResult = AxiosResponse<UserQueryDto>;
export type DeleteUsersIUserIdResult = AxiosResponse<void>;
export type GetUsersMeResult = AxiosResponse<UserQueryDto>;
export type DeleteUsersMeResult = AxiosResponse<void>;
export type GetUsersNUserNameResult = AxiosResponse<UserQueryDto>;
export type DeleteUsersNUserNameResult = AxiosResponse<void>;
export type GetVersionResult = AxiosResponse<unknown>;
