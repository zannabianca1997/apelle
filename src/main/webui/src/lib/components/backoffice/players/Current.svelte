<script lang="ts">
	import type { Uuid } from '$lib/apis/apelle.ts';
	import {
		postApiV1QueuesIQueueIdStart as postStart,
		postApiV1QueuesIQueueIdStop as postStop,
		postApiV1QueuesIQueueIdNext as postNext
	} from '$lib/apis/apelle';
	import { _ } from 'svelte-i18n';
	import Player from './Player.svelte';
	import type { QueueUserQueryWithRoleDto } from '$lib/models/QueueUserQueryWithRoleDto';
	import IconPlay from '~icons/mdi/play';
	import IconPause from '~icons/mdi/pause';
	import IconNext from '~icons/mdi/skip-next';
	import type { CurrentSong } from '$lib/models/Queue.svelte';

	let {
		queueId,
		playerStateId = $bindable(),
		current = $bindable(),
		user,
		isPlayer = $bindable(false)
	}: {
		queueId: Uuid;
		playerStateId: Uuid;
		current?: CurrentSong;
		user: QueueUserQueryWithRoleDto;
		isPlayer: boolean;
	} = $props();

	async function start() {
		await postStart(queueId, {
			headers: {
				'If-Match': `W/"${playerStateId}"`
			}
		});
	}
	async function stop() {
		await postStop(queueId, {
			headers: {
				'If-Match': `W/"${playerStateId}"`
			}
		});
	}
	async function next() {
		await postNext(queueId, {
			headers: {
				'If-Match': `W/"${playerStateId}"`
			}
		});
	}
</script>

<section>
	{#if current}
		<Player bind:current {isPlayer} />
	{:else}
		<h1>{$_('backoffice.currentSong.nothingPlaying')}</h1>
	{/if}
	<div class="playControls">
		{#if !current || current.stopped}
			{#if user.queue_role.permissions.queue.start}
				<button onclick={start}><IconPlay height={75} width={75} /></button>
			{/if}
		{:else if user.queue_role.permissions.queue.stop}
			<button onclick={stop}><IconPause height={75} width={75} /></button>
		{/if}
		{#if user.queue_role.permissions.queue.next}
			<button onclick={next}><IconNext height={75} width={75} /></button>
		{/if}
	</div>
</section>

<style lang="scss">
	section {
		display: flex;
		flex-direction: row;
		justify-content: space-evenly;
		align-items: center;
		gap: 24px;

		width: 100%;
		height: 244px;
		border-radius: 6px;
		padding: 12px;

		background: linear-gradient(90deg, rgba(55, 155, 70, 0.75) 0%, rgba(36, 101, 46, 0.75) 81.67%);

		h1 {
			margin: 0;
			flex-grow: 1;

			text-align: center;
		}

		.playControls {
			display: flex;
			gap: 55px;
			flex-grow: 0;

			button {
				background: transparent;
				border: 0;
				color: white;

				&:hover {
					background: radial-gradient(closest-side, #00000088, #00000000);
				}
			}
		}
	}
</style>
