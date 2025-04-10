<script lang="ts">
	import {
		postApiV1QueuesIQueueIdQueue as enqueueSong,
		type QueueEventDto,
		type QueueQueryDto
	} from '$lib/apis/apelle';
	import TextInput from '$lib/components/forms/TextInput.svelte';
	import type { PageProps } from './$types';
	import type { QueueUserQueryWithRoleDto } from './+page';
	import { _ } from 'svelte-i18n';
	import Player from '$lib/components/backoffice/players/Container.svelte';
	import QueuedSongCard from '$lib/components/backoffice/QueuedSongCard.svelte';
	import { source } from 'sveltekit-sse';
	import authService from '$lib/auth.svelte';
	import { goto } from '$app/navigation';

	const { data }: PageProps = $props();

	const queueId = data.queue.id;
	let queue: QueueQueryDto = $state(data.queue);
	let isPlayer: boolean = $state(data.isPlayer);
	const user: QueueUserQueryWithRoleDto = $state(data.user);

	let songQuery: string | null = $state(null);

	source(`/api/v1/queues/i/${queueId}/events`, {
		options: {
			method: 'GET',
			headers: {
				Authorization:
					'Basic ' + btoa(`${authService.userData?.username}:${authService.userData?.password}`)
			}
		}
	})
		.select('')
		.json<QueueEventDto | null>()
		.subscribe((event) => {
			if (!event) {
				return;
			}

			switch (event.kind) {
				case 'queue-state':
					queue = event.queue;
					break;
				case 'queue-delete':
					goto('/');
					break;
			}
		});

	async function addToQueue(e: SubmitEvent) {
		e.preventDefault();
		const videoId = songQuery?.trim();
		if (!videoId) {
			return;
		}
		enqueueSong(queueId, { kind: 'Youtube', video_id: videoId });
	}
</script>

<main>
	{#if isPlayer}
		<Player current={queue.current} />
	{/if}
	<section>
		<h1>{$_('backoffice.partyName')}<code>{queue.code}</code></h1>
		<form onsubmit={addToQueue}>
			<TextInput
				bind:value={songQuery}
				label={$_('backoffice.addSong.label')}
				placeholder={$_('backoffice.addSong.placeholder')}
			/>
			<button>{$_('backoffice.addSong.submit')}</button>
		</form>
	</section>
	<section>
		<h1>Queue</h1>
		<ol class="queue">
			{#each queue.queue as song (song.id)}
				<QueuedSongCard {song} queue={queue.id} />
			{/each}
		</ol>
	</section>
</main>

<style lang="scss">
	main {
		width: 80%;

		display: flex;
		flex-direction: column;
		gap: 57px;

		h1 {
			font-weight: 900;
			font-size: 32px;
			line-height: 150%;
			letter-spacing: 1%;

			color: #379b46;

			code {
				color: white;
			}
		}

		form {
			width: 100%;

			display: flex;
			gap: 12px;
			align-items: last baseline;

			--input-flex-grow: 1;

			button {
				width: 100px;
				border: 1px solid white;
				height: 48px;
				border-radius: 4px;
				padding-top: 6px;
				padding-right: 12px;
				padding-bottom: 6px;
				padding-left: 12px;

				font-weight: 900;
				font-size: 16px;
				line-height: 100%;
				letter-spacing: 0%;

				color: white;
				background-color: transparent;

				cursor: pointer;
			}
		}

		ol.queue {
			display: flex;
			flex-direction: column;
			gap: 5px;

			list-style-type: none;
			margin: 0;
			padding: 0;
		}
	}
</style>
