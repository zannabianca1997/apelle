<script lang="ts">
	import config from '$lib/config';
	import IconRemove from '~icons/mdi/close';

	interface Notify {
		id: number;
		kind: 'error';
		msg: string;
	}

	let nextId = $state(0);

	let notifications: Notify[] = $state([]);
	let current: Notify | null = $derived(notifications.at(0) ?? null);

	export function error(msg: string) {
		notifications.push({ kind: 'error', msg, id: nextId++ });
	}

	function close() {
		notifications.shift();
	}

	$effect(() => {
		if (current) {
			const interval = setInterval(close, config.notifications.timeout);
			return () => clearInterval(interval);
		}
	});
</script>

<aside class="center fixed bottom-4 flex flex-col-reverse gap-[5px]">
	{#each notifications as notification, _ (notification.id)}
		<div
			id="notification-{notification.id}"
			class="flex items-center justify-between rounded-md bg-red-500 p-4 text-white"
		>
			{notification.msg}
			<button class="text-white" onclick={close} aria-label="close">
				<IconRemove />
			</button>
		</div>
	{/each}
</aside>
