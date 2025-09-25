<script lang="ts">
	import type { Queue } from '$lib/apis/apelle';
	import type { PageProps } from './$types';
	import Player from './player/Player.svelte';
	import QueueView from './queue/QueueView.svelte';

	const { data }: PageProps = $props();

	const queueId = $derived(data.connection.queue?.id);
	let queue: Queue | null = $derived(data.connection.queue);
</script>

<svelte:head>
	{#if queue}
		{#if queue.current && typeof queue.current.song != 'string'}
			<title>Apelle - {queue.current.song.title}</title>
		{:else}
			<title>Apelle - {queue.code}</title>
		{/if}
	{/if}
</svelte:head>

{#if queue}
	<div>
		{JSON.stringify(queue, undefined, 2)}
	</div>

	<main>
		<section>
			<Player song={queue.current} />
		</section>
		<section>
			<!-- Search bar -->
		</section>
		<section>
			<QueueView songs={queue.queue} />
		</section>
	</main>
{:else}
	<h1>Loading...</h1>
{/if}
