import { config } from './config';

export type LogLevel = 'debug' | 'info' | 'warn' | 'error';
export type LogLevelFilter = LogLevel | 'all' | 'none';

let logger: Logger | undefined;

function shouldLog(filter: LogLevelFilter, level: LogLevel): boolean {
	switch (filter) {
		case 'none':
			return false;
		case 'error':
			return ['error'].includes(level);
		case 'warn':
			return ['warn', 'error'].includes(level);
		case 'info':
			return ['info', 'warn', 'error'].includes(level);
		case 'debug':
			return ['debug', 'info', 'warn', 'error'].includes(level);
		case 'all':
			return true;
	}
}

const rules = (Object.keys(config.log) as (keyof typeof config.log)[]).sort(
	(a, b) => b.length - a.length
);

export class Logger {
	readonly debug: typeof console.debug;
	readonly info: typeof console.info;
	readonly warn: typeof console.warn;
	readonly error: typeof console.error;

	constructor(name: string) {
		const level = config.log[rules.find((rule) => name.startsWith(rule))!];

		logger?.debug('Creating logger for', name, 'with level', level);

		const wrap =
			(fn: (...args: any[]) => void) =>
			(...args: any[]) => {
				fn(`[${name}]`, ...args);
			};

		this.debug = shouldLog(level, 'debug') ? wrap(console.debug) : () => {};
		this.info = shouldLog(level, 'info') ? wrap(console.info) : () => {};
		this.warn = shouldLog(level, 'warn') ? wrap(console.warn) : () => {};
		this.error = shouldLog(level, 'error') ? wrap(console.error) : () => {};
	}
}

logger = new Logger('lib.logger');
logger.debug('Loaded rules:', rules);
