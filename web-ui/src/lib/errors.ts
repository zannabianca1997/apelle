export type Success<T> = Result<T, never>;
export type Failure<E extends ErrorType> = Result<never, E>;

export type ErrorType = {
	_tag: string;
	[key: string]: any;
};

type MatchCases<T, E extends ErrorType, U> = {
	Success: (data: T) => U;
} & {
	[K in E['_tag']]: (error: Extract<E, { _tag: K }>) => U;
};

export class Result<T, E extends ErrorType> {
	protected constructor(
		readonly _tag: 'Success' | 'Failure',
		protected readonly value: T | E
	) {}

	static succeed<T>(data: T): Success<T> {
		return new Result('Success', data) as Success<T>;
	}

	static fail<E extends ErrorType>(tag: E['_tag'], error: Omit<E, '_tag'>): Failure<E> {
		return new Result('Failure', { _tag: tag, ...error }) as Failure<E>;
	}

	isSuccess(): this is Success<T> {
		return this._tag === 'Success';
	}

	isFailure(): this is Failure<E> {
		return this._tag === 'Failure';
	}

	get data(): T {
		if (this.isSuccess()) return this.value as T;
		throw new Error('Cannot get data from a Failure');
	}

	get error(): E {
		if (this.isFailure()) return this.value as E;
		throw new Error('Cannot get error from a Success');
	}

	map<U>(f: (value: T) => U): Result<U, E> {
		return this.isSuccess() ? Result.succeed(f(this.data)) : (this as unknown as Result<U, E>);
	}

	flatMap<U>(f: (value: T) => Result<U, E>): Result<U, E> {
		return this.isSuccess() ? f(this.data) : (this as unknown as Result<U, E>);
	}

	equals(that: unknown): boolean {
		return that instanceof Result && this._tag === that._tag && this.value === that.value;
	}

	toJSON() {
		return {
			_tag: this._tag,
			[this._tag === 'Success' ? 'data' : 'error']: this.value
		};
	}

	toString(): string {
		return JSON.stringify(this.toJSON());
	}

	static matchTag<T, E extends ErrorType, U>(result: Result<T, E>, cases: MatchCases<T, E, U>): U {
		if (result.isSuccess()) {
			return cases.Success(result.data);
		} else {
			const errorHandler = cases[result.error._tag as keyof typeof cases];
			if (errorHandler) {
				return errorHandler(result.error as any);
			}
			throw new Error(`Unhandled error type: ${result.error._tag}`);
		}
	}
}
