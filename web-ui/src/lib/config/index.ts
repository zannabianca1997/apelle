import type { LogLevelFilter } from '$lib/logger';
import { merge } from 'ts-deepmerge';

import defaults from './defaults';
import dev from './dev';
import prod from './prod';

/**
 * Configuration for the logging
 */
export type LogConfig = Record<string, LogLevelFilter> & { '': LogLevelFilter };

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
	log: LogConfig;
	player: Player;
}

export const config = (
	import.meta.env.DEV ? merge(defaults, dev) : merge(defaults, prod)
) satisfies Config;
