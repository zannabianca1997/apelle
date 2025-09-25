<script lang="ts">
	import type { Queue } from '$lib/apis/apelle';
	import Search from './Search.svelte';
	import type { PageProps } from './$types';
	import Player from './player/Player.svelte';
	import QueueView from './queue/QueueView.svelte';
	import isString from '$lib/utils/isString';
	import { _ } from 'svelte-i18n'

	const { data }: PageProps = $props();

	const queueId = $derived(data.connection.queue?.id);
	let queue: Queue | null = $derived(data.connection.queue);

</script>

<svelte:head>
	{#if queue}
		{#if queue.current && !isString(queue.current.song)}
			<title>Apelle - {queue.current.song.title}</title>
		{:else}
			<title>Apelle - {queue.code}</title>
		{/if}
	{/if}
</svelte:head>

{#if queue}
	<aside>
		{JSON.stringify(queue, undefined, 2)}
	</aside>

	<main class="w-4/5 flex flex-col gap-[57px]">
		<section class="flex flex-row justify-evenly items-center gap-6 w-full h-[244px] rounded-md p-3 bg-gradient-to-r from-[rgba(55,155,70,0.75)] to-[rgba(36,101,46,0.75)]">
			<Player song={queue.current} />
		</section>
		<section>
			<h1 class="font-black text-[32px] leading-[1.5] tracking-[.01em] text-[#379b46]">
				{$_('backoffice.partyName')}<code  class="text-white">{queue.code}</code>
			</h1>
			<Search />
		</section>
		<section>
			<h1 class="font-black text-[32px] leading-[1.5] tracking-[.01em] text-[#379b46]">
				{$_('backoffice.queue.title')}
			</h1>
			<QueueView songs={queue.queue} />
		</section>
	</main>
{:else}
	<h1>Loading...</h1>
{/if}


