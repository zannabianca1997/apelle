import {
	getApiV1QueuesIQueueId as getQueueById,
	getApiV1QueuesIQueueIdUsersMe as getQueueUser,
	getApiV1ConfigsQueueUserRolesRoleName as getRoleConfig,
	type QueueQueryDto,
	type QueueUserQueryDto
} from '$lib/apis/apelle';
import { AxiosError } from 'axios';
import type { PageLoad } from './$types';
import { error } from '@sveltejs/kit';
import type { QueueUserQueryWithRoleDto } from '$lib/models/QueueUserQueryWithRoleDto';

async function fillUserRole(user: QueueUserQueryDto): Promise<QueueUserQueryWithRoleDto> {
	const { data: queue_role } = await getRoleConfig(user.queue_role);
	return {
		...user,
		queue_role
	};
}

export const load: PageLoad = async ({ params: { queueId }, url }) => {
	const queue: Promise<QueueQueryDto> = getQueueById(queueId)
		.then(({ data }) => data)
		.catch((e: unknown) => {
			if (e instanceof AxiosError) {
				if (e.status == 404) {
					error(404, {
						message: `Queue ${queueId} not found.`
					});
				}
			}
			throw e;
		});
	const user: Promise<QueueUserQueryWithRoleDto | null> = getQueueUser(queueId)
		.then(({ data }) => fillUserRole(data))
		.catch((e: unknown) => {
			if (e instanceof AxiosError) {
				if (e.status == 404) {
					return null;
				}
			}
			throw e;
		});
	const isPlayer: boolean = url.searchParams.get('player') == 'true';
	return (await Promise.all([queue, user]).then(([queue, user]) => {
		if (user == null) {
			throw new Error(
				'This should not be possible as the only case in which 404 is returned for users/me is if the queue does not exist.'
			);
		}
		return {
			isPlayer,
			queue,
			user
		};
	})) satisfies { queue: QueueQueryDto; user: QueueUserQueryWithRoleDto; isPlayer: boolean };
};
