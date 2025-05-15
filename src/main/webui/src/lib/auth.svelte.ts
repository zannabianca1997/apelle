import axios, { AxiosError, type AxiosBasicCredentials } from 'axios';

import { getApiV1UsersMe, postApiV1Users, type UserQueryDto } from '$lib/apis/apelle';
import { browser } from '$app/environment';
import { Logger } from '$lib/logger';
import config from '$lib/config';

const logger = new Logger(config.log.auth);

export type Credentials = AxiosBasicCredentials;
type UserData = Omit<UserQueryDto, 'name'> & Credentials;

export interface AuthService {
	/**
	 * Signin with the given credentials.
	 * @return {Promise<void>} Signin successfully.
	 * @return {Promise<{ error: 'badCredentials' }>} If the credentials are bad.
	 */
	signin(auth: Credentials): Promise<void | { error: 'badCredentials' }>;

	/**
	 * Signup with the given credentials.
	 * @return {Promise<void>} Signup successfully.
	 * @return {Promise<{ error: 'userExists' }>} If the user already exists.
	 */
	signup(auth: Credentials): Promise<void | { error: 'userExists' }>;

	/**
	 * Signout the current user.
	 */
	signout(): Promise<void>;

	/**
	 * Check if the current user is authenticated.
	 */
	authenticated(): boolean;

	get userData(): UserData | null;
}

class AuthServiceBrowser implements AuthService {
	private _userData: UserData | null;

	/**
	 * Constructs a new AuthService instance.
	 *
	 * Tries to read the user data from the local storage. If the data is not
	 * present, the user is considered not authenticated.
	 */
	public constructor() {
		this._userData = JSON.parse(localStorage.getItem('apelleUser') ?? 'null');

		logger.info('Installing auth interceptors');
		axios.interceptors.request.use((config) => {
			// Add authentication if not provided
			if (this.authenticated() && !config.auth) {
				config.auth = {
					username: this.userData!.username,
					password: this.userData!.password
				};
			}

			return config;
		});
	}

	public get userData(): UserData | null {
		return this._userData;
	}

	private set userData(data: UserData | null) {
		this._userData = data;
		if (this.userData) {
			localStorage.setItem('apelleUser', JSON.stringify(this.userData));
		} else {
			localStorage.removeItem('apelleUser');
		}
	}

	/**
	 * Signin with the given credentials.
	 *
	 * Makes a request to the `/me` endpoint with the provided credentials.
	 * If the credentials are valid, the user data is stored in the local
	 * storage and the user is considered authenticated.
	 *
	 * If the credentials are invalid, a `badCredentials` error is returned.
	 * If the `/me` endpoint returns an unexpected response, an error is thrown.
	 *
	 * @param auth the credentials to signin with
	 * @return {Promise<void | { error: 'badCredentials' }>} Signin successfully, or an error if the credentials are bad
	 */
	public async signin(auth: Credentials): Promise<void | { error: 'badCredentials' }> {
		let userQueryDto;
		try {
			userQueryDto = await getApiV1UsersMe({ auth });
		} catch (e) {
			if (e instanceof AxiosError) {
				if (e?.response?.status == 401) {
					return { error: 'badCredentials' };
				}
			}
			throw e;
		}
		if (userQueryDto.status != 200) {
			throw new Error('Unexpected server response from `/me`.');
		}
		logger.debug(`Signing in as ${auth.username}`);
		this.userData = { ...userQueryDto.data, ...auth };
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
	 * @return {Promise<void | { error: 'userExists' }>} Signup successfully, or an error if the user already exists
	 */
	public async signup(auth: Credentials): Promise<void | { error: 'userExists' }> {
		let userQueryDto;
		try {
			userQueryDto = await postApiV1Users({ name: auth.username, password: auth.password });
		} catch (e) {
			if (e instanceof AxiosError) {
				if (e?.response?.status == 409) {
					return { error: 'userExists' };
				}
			}
			throw e;
		}
		if (userQueryDto.status != 201) {
			throw new Error('Unexpected server response from `/me`.');
		}
		logger.debug(`Signing up as ${auth.username}`);
		this.userData = { ...userQueryDto.data, ...auth };
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
		logger.debug(`Signing out from ${this.userData?.username}`);
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

/**
 * Stub class to fake authentication outside of the browser
 */
class AuthServiceStub implements AuthService {
	public async signin(): Promise<void> {
		throw new Error('Auth is not implemented outside the browser');
	}
	public async signup(): Promise<void> {
		throw new Error('Auth is not implemented outside the browser');
	}
	public async signout(): Promise<void> {
		throw new Error('Auth is not implemented outside the browser');
	}
	public authenticated(): boolean {
		throw new Error('Auth is not implemented outside the browser');
	}
	get userData(): UserData | null {
		throw new Error('Auth is not implemented outside the browser');
	}
}

export const authService: AuthService = $state(
	browser ? new AuthServiceBrowser() : new AuthServiceStub()
);

export default authService;
