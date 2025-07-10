import axios, { AxiosError, type AxiosBasicCredentials } from 'axios';

import { usersGet, usersCreate, type UserDto } from '$lib/apis/apelle';
import { Logger } from '$lib/logger';

import config from '$lib/config';
import { Result } from '$lib/errors.svelte';

const logger = new Logger('lib.auth');

const localStorageKey = config.auth.localStorageKey;

type BadCredentials = {
	_tag: 'badCredentials';
	username: string;
};

type UserExists = {
	_tag: 'userExists';
	username: string;
};

type UserData = {
	data: UserDto;
	auth: AxiosBasicCredentials;
};

class AuthService {
	private _userData: UserData | null = $state(null);

	/**
	 * Constructs a new AuthService instance.
	 *
	 * Tries to read the user data from the local storage. If the data is not
	 * present, the user is considered not authenticated.
	 */
	public constructor() {
		let stored = localStorage.getItem(localStorageKey);
		this._userData = (stored ? JSON.parse(stored) : null) satisfies UserData | null;

		logger.info('Installing auth interceptors');
		axios.interceptors.request.use((config) => {
			// Add authentication if not provided
			if (this.authenticated() && !config.auth) {
				config.auth = this.auth!;
			}

			return config;
		});
	}

	public get user(): UserDto | null {
		return this._userData?.data ?? null;
	}

	private get auth(): AxiosBasicCredentials | null {
		return this._userData?.auth ?? null;
	}

	private set userData(data: UserData | null) {
		this._userData = data;
		if (this.userData) {
			localStorage.setItem(localStorageKey, JSON.stringify(this.userData));
		} else {
			localStorage.removeItem(localStorageKey);
		}
	}

	/**
	 * Signin with the given credentials.
	 *
	 * Makes a request to the `/me` endpoint with the provided credentials.
	 * If the credentials are valid, the user data is stored in the local
	 * storage and the user is considered authenticated.
	 *
	 * If the credentials are invalid, a `BadCredentials` error is returned.
	 * If the `/me` endpoint returns an unexpected response, an error is thrown.
	 *
	 * @param auth the credentials to signin with
	 * @return {Promise<Result<void, BadCredentials>>} Signin successfully, or an error if the credentials are bad
	 */
	public async signin(auth: AxiosBasicCredentials): Promise<Result<void, BadCredentials>> {
		let userQueryDto;
		try {
			userQueryDto = await usersGet({ auth });
		} catch (e) {
			if (e instanceof AxiosError) {
				if (e?.response?.status == 401) {
					return Result.fail('badCredentials', { username: auth.username });
				}
			}
			throw e;
		}
		if (userQueryDto.status != 200) {
			throw new Error('Unexpected server response from `/me`.');
		}
		logger.debug(`Signing in as ${auth.username}`);
		this.userData = { data: userQueryDto.data, auth };
		return Result.succeed(undefined);
	}

	/**
	 * Signup with the given credentials.
	 *
	 * Makes a request to the `/users` endpoint with the provided credentials.
	 * If the credentials are valid, the user data is stored in the local
	 * storage and the user is considered authenticated.
	 *
	 * If the user already exist, a `userExists` error is returned.
	 * If the `/me` endpoint returns an unexpected response, an error is thrown.
	 *
	 * @param auth the credentials to signup with
	 * @return {Promise<Result<void, UserExists>>} Signup successfully, or an error if the user already exists
	 */
	public async signup(auth: AxiosBasicCredentials): Promise<Result<void, UserExists>> {
		let userQueryDto;
		try {
			userQueryDto = await usersCreate({ name: auth.username, password: auth.password });
		} catch (e) {
			if (e instanceof AxiosError) {
				if (e?.response?.status == 409) {
					return Result.fail('userExists', { username: auth.username });
				}
			}
			throw e;
		}
		if (userQueryDto.status != 201) {
			throw new Error('Unexpected server response from `/me`.');
		}
		logger.debug(`Signing up as ${auth.username}`);
		this.userData = { data: userQueryDto.data, auth };
		return Result.succeed(undefined);
	}

	/**
	 * Signout the current user.
	 *
	 * Removes the user data from the local storage.
	 * The user is no longer considered authenticated.
	 *
	 * @return {Promise<void>} Signout successfully
	 */
	public async signout(): Promise<void> {
		logger.debug(`Signing out from ${this.userData?.data.name}`);
		this.userData = null;
	}

	/**
	 * Checks if the current user is authenticated.
	 *
	 * @return {boolean} true if the user is authenticated, false otherwise
	 */
	public authenticated(): boolean {
		return this.userData != null;
	}
}

const authService = $state(new AuthService());
export default authService;
