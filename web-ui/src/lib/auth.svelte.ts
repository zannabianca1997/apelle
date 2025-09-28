import axios, { AxiosError, type AxiosBasicCredentials } from 'axios';
import {
    usersGet,
    usersCreate,
    type UserDto,
    type UserCreateDto
} from '$lib/apis/apelle';
import { Logger } from '$lib/logger';
import config from '$lib/config';
import { Result } from '$lib/errors.svelte';
import { goto } from '$app/navigation';

const logger = new Logger('lib.auth');

const localStorageKey = config.auth.localStorageKey;

class BadCredentials {
    _tag: 'badCredentials' = 'badCredentials';
    username: string;

    constructor(username: string) {
        this.username = username;
    }
}

class UserExists {
    _tag: 'userExists' = 'userExists';
    username: string;

    constructor(username: string) {
        this.username = username;
    }
}

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
        this._userData = (
            stored ? JSON.parse(stored) : null
        ) satisfies UserData | null;

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

    public get headers(): Record<string, string> {
        return this.auth
            ? {
                  Authorization: `Basic ${btoa(`${this.auth.username}:${this.auth.password}`)}`
              }
            : {};
    }

    private set userData(data: UserData | null) {
        this._userData = data;
        if (this._userData) {
            localStorage.setItem(
                localStorageKey,
                JSON.stringify(this._userData)
            );
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
    public async signin(
        auth: AxiosBasicCredentials
    ): Promise<Result<void, BadCredentials>> {
        let userQueryDto;
        try {
            userQueryDto = await usersGet({ auth });
        } catch (e) {
            if (e instanceof AxiosError) {
                if (e?.response?.status == 401) {
                    return Result.fail(new BadCredentials(auth.username));
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
    public async signup(
        auth: UserCreateDto
    ): Promise<Result<void, UserExists>> {
        let userQueryDto;
        try {
            userQueryDto = await usersCreate(auth);
        } catch (e) {
            if (e instanceof AxiosError) {
                if (e?.response?.status == 409) {
                    return Result.fail(new UserExists(auth.name));
                }
            }
            throw e;
        }
        if (userQueryDto.status != 201) {
            throw new Error('Unexpected server response from `/me`.');
        }
        logger.debug(`Signing up as ${auth.name}`);
        this.userData = {
            data: userQueryDto.data,
            auth: { username: auth.name, password: auth.password }
        };
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
        logger.debug(`Signing out from ${this.user?.name}`);
        this.userData = null;
    }

    /**
     * Checks if the current user is authenticated.
     *
     * @return {boolean} true if the user is authenticated, false otherwise
     */
    public authenticated(): boolean {
        return this.auth != null;
    }
}

const authService = $state(new AuthService());
export default authService;

export async function routeToAuth(url: URL) {
    logger.debug('User is not authenticated, rerouting to auth endpoint');

    const authUrl = new URL('/auth', url);
    authUrl.searchParams.set('original', url.toString());
    await goto(authUrl);
}
