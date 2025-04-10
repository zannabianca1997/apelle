import type { QueueUserQueryDto, QueueUserRole } from '$lib/apis/apelle';

export type QueueUserQueryWithRoleDto = Omit<QueueUserQueryDto, 'queue_role'> & {
	queue_role: QueueUserRole;
};
