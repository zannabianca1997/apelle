import dev from './dev';
import prod from './prod';
import staging from './staging';

/**
 * Configuration for the logging
 */
export interface Log {
	/**
	 * Log all sse events
	 */
	sse: boolean;
}

/**
 * Profile based configuration
 */
export interface Config {
	log: Log;
}

export const QUARKUS_PROFILE: 'dev' | 'prod' | 'staging' = import.meta.env.VITE_QUARKUS_PROFILE;

export const config: Config = {
	dev,
	prod,
	staging
}[QUARKUS_PROFILE];
