<script lang="ts">
	import {
		postApiV1QueuesIQueueIdQueue as enqueueSong,
		type QueueEventDto
	} from '$lib/apis/apelle';
	import TextInput from '$lib/components/forms/TextInput.svelte';
	import type { PageProps } from './$types';
	import type { QueueUserQueryWithRoleDto } from '$lib/models/QueueUserQueryWithRoleDto';
	import { _ } from 'svelte-i18n';
	import Player from '$lib/components/backoffice/players/Container.svelte';
	import QueuedSongCard from '$lib/components/backoffice/QueuedSongCard.svelte';
	import { source } from 'sveltekit-sse';
	import authService from '$lib/auth.svelte';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { Queue } from '$lib/models/Queue.svelte';

	const { data }: PageProps = $props();

	const queueId = data.queue.id;
	let queue: Queue = $state(data.queue);

	let isPlayer: boolean = $state(data.isPlayer);
	const user: QueueUserQueryWithRoleDto = $state(data.user);

	let songQuery: string | null = $state(null);

	onMount(() => {
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
					case 'queue-delete':
						goto('/');
						break;
					default:
						queue.update(event);
						break;
				}
			});
	});

	async function addToQueue(e: SubmitEvent) {
		e.preventDefault();
		const videoId = songQuery?.trim();
		if (!videoId) {
			return;
		}
		await enqueueSong(queueId, { kind: 'Youtube', video_id: videoId });
		songQuery = null;
	}
</script>

<main>
	{#if isPlayer}
		<Player {queueId} bind:current={queue.current} {user} />
	{/if}
	<section>
		<h1>{$_('backoffice.partyName')}<code>{queue.code}</code></h1>
		{#if user.queue_role.permissions.queue.enqueue}
			<form onsubmit={addToQueue}>
				<TextInput
					bind:value={songQuery}
					label={$_('backoffice.addSong.label')}
					placeholder={$_('backoffice.addSong.placeholder')}
				/>
				<button>{$_('backoffice.addSong.submit')}</button>
			</form>
		{/if}
	</section>
	<section>
		<h1>{$_('backoffice.queue.title')}</h1>
		<table class="queue">
			{#each queue.queue as song (song.id)}
				<QueuedSongCard {song} queue={queue.id} />
			{/each}
		</table>
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
				width: 175px;
				height: 48px;
				top: 26px;
				left: 758px;
				border-radius: 4px;
				padding-top: 6px;
				padding-right: 12px;
				padding-bottom: 6px;
				padding-left: 12px;

				font-weight: 900;
				font-size: 16px;
				line-height: 100%;
				letter-spacing: 0%;

				text-transform: uppercase;
				border: 0;

				color: white;
				background: #911616;

				cursor: pointer;
			}
		}

		table.queue {
			width: 100%;
			table-layout: fixed;
			border-spacing: 12px;
		}
	}
</style>
