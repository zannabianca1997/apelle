export type LogLevel = 'debug' | 'info' | 'warn' | 'error';
export type LogLevelFilter = LogLevel | 'all' | 'none';

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

export class Logger {
	readonly debug: typeof console.debug;
	readonly info: typeof console.info;
	readonly warn: typeof console.warn;
	readonly error: typeof console.error;

	constructor(level: LogLevelFilter) {
		this.debug = shouldLog(level, 'debug') ? console.debug : () => {};
		this.info = shouldLog(level, 'info') ? console.info : () => {};
		this.warn = shouldLog(level, 'warn') ? console.warn : () => {};
		this.error = shouldLog(level, 'error') ? console.error : () => {};
	}
}
