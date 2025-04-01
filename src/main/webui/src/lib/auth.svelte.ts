import type { AxiosBasicCredentials } from "axios";

import { getUsersMe, postUsers, type UserQueryDto } from "$lib/apis/apelle"
import { browser } from '$app/environment';


type Credentials = AxiosBasicCredentials;

type UserData = Omit<UserQueryDto, 'name'> & Credentials;

const userData: {
    data: UserData | null
} = $state({
    data: JSON.parse((browser && localStorage.getItem('apelleUser')) || 'null') as UserData
});

function setUserData(data: UserData | null) {
    userData.data = data;
    if (userData.data) { localStorage.setItem('apelleUser', JSON.stringify(userData.data)) }
    else { localStorage.removeItem('apelleUser') }
}


export async function signin(auth: Credentials): Promise<void | { error: 'badCredentials' }> {
    let userQueryDto;
    try {
        userQueryDto = await getUsersMe({ auth });
    } catch (e: any) {
        if (e?.response?.status == 401) { return { error: 'badCredentials' } }
        throw e;
    }
    if (userQueryDto.status != 200) {
        throw { msg: "Unexpected server response from `/me`.", userQueryDto };
    }
    console.debug(`Signing in as ${auth.username}`)
    setUserData({ ...userQueryDto.data, ...auth })
}

export async function signup(auth: Credentials): Promise<void | { error: 'userExists' }> {
    let userQueryDto;
    try {
        userQueryDto = await postUsers({ name: auth.username, password: auth.password });
    } catch (e: any) {
        if (e?.response?.status == 409) { return { error: 'userExists' } }
        throw e;
    }
    if (userQueryDto.status != 201) {
        throw { msg: "Unexpected server response from `/me`.", userQueryDto };
    }
    console.debug(`Signing up as ${auth.username}`)
    setUserData({ ...userQueryDto.data, ...auth })
}

export async function signout(): Promise<void> {
    console.debug(`Signing out from ${userData.data?.username}`)
    setUserData(null)
}

export function authenticated(): boolean {
    return userData.data != null;
}