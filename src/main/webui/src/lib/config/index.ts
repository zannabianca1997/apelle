import type { LogLevelFilter } from '$lib/logger';
import { merge } from 'ts-deepmerge';

import defaults from './defaults';
import dev from './dev';
import prod from './prod';
import staging from './staging';

/**
 * Configuration for the logging
 */
export interface Log {
	/**  SSE events */
	sse: LogLevelFilter;
	/** Auth events */
	auth: LogLevelFilter;
}

/**
 * Configuration for the player
 */
export interface Player {
	/** The allowed desync between the player and the backend in seconds */
	allowedDesync: number;
}

/**
 * Profile based configuration
 */
export interface Config {
	log: Log;
	player: Player;
}

export const QUARKUS_PROFILE: 'dev' | 'prod' | 'staging' = import.meta.env.VITE_QUARKUS_PROFILE;

export default merge(
	defaults,
	{
		dev,
		prod,
		staging
	}[QUARKUS_PROFILE]
) satisfies Config;
