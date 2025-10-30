import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params: { queueId } }) => {
    return { queueId };
};
