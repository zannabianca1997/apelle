import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params: { queueId }, url }) => {
    return { queueId };
};
