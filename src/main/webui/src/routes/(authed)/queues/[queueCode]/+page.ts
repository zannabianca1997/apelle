import { getApiV1QueuesCQueueCode as getQueueByCode, type Uuid } from '$lib/apis/apelle';
import { AxiosError } from 'axios';
import type { PageLoad } from './$types';
import { error } from '@sveltejs/kit';
import { goto } from '$app/navigation';

export const load: PageLoad = async ({ params: { queueCode } }) => {
    let id: Uuid;
    try {
        const {
            data: { id: returnedId }
        } = await getQueueByCode(queueCode);
        id = returnedId;
    } catch (e: unknown) {
        if (e instanceof AxiosError) {
            if (e.status == 404) {
                error(404, {
                    message: `Queue ${queueCode} not found.`
                });
            }
        }
        throw e;
    }

    goto(`/queues/${id}`)
};
