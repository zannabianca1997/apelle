import { getApiV1QueuesIQueueId as getQueueById, type QueueQueryDto } from '$lib/apis/apelle';
import { AxiosError } from 'axios';
import type { PageLoad } from './$types';
import { error } from '@sveltejs/kit';

export const load: PageLoad = async ({ params: { queueId } }) => {
	let queue: QueueQueryDto;
	try {
		const { data } = await getQueueById(queueId);
		queue = data;
	} catch (e) {
		if (e instanceof AxiosError) {
			if (e.status == 404) {
				error(404, {
					message: `Queue ${queueId} not found.`
				});
			}
		}
		throw e;
	}
	return { queue } satisfies { queue: QueueQueryDto };
};
