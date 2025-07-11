<script lang="ts">
	import {
		postApiV1QueuesIQueueIdQueue as enqueueSong,
		type QueueEventDto,
		type SongAddDto
	} from '$lib/apis/apelle';
	import type { PageProps, Snapshot } from './$types';
	import type { QueueUserQueryWithRoleDto } from '$lib/models/QueueUserQueryWithRoleDto';
	import { _ } from 'svelte-i18n';
	import Current from '$lib/components/backoffice/players/Current.svelte';
	import QueuedSongCard from '$lib/components/backoffice/QueuedSongCard.svelte';
	import { source } from 'sveltekit-sse';
	import authService from '$lib/auth.svelte';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { Queue } from '$lib/models/Queue.svelte';
	import { Logger } from '$lib/logger';
	import config from '$lib/config';
	import SearchBar from '$lib/components/backoffice/search/SearchBar.svelte';
	import SearchDialog from '$lib/components/backoffice/search/SearchDialog.svelte';
	import NavBarToggle from '$lib/components/navbar/elements/NavBarToggle.svelte';
	import { page } from '$app/state';
	import NavBarSection from '$lib/components/navbar/NavBarSection.svelte';

	const { data }: PageProps = $props();

	const queueId = data.queue.id;
	let queue: Queue = $state(data.queue);

	let isPlayer: boolean = $state(data.isPlayer);
	$effect(() => {
		const url = page.url;

		if (isPlayer === (url.searchParams.get('player') == 'true')) {
			return;
		}
		url.searchParams.set('player', isPlayer ? 'true' : 'false');
		goto(url);
	});

	const user: QueueUserQueryWithRoleDto = $state(data.user);

	const SSELogger = new Logger(config.log.sse);

	onMount(() =>
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

				SSELogger.info(`Received queue event: %o`, event);

				if (event.kind === 'queue-delete') {
					goto('/');
				} else {
					queue.update(event);
				}
			})
	);

	async function openSearch(query: string): Promise<boolean> {
		await searchDialog.open(query);
		return true;
	}

	async function onSongChosen(song: SongAddDto) {
		await enqueueSong(queueId, song);
	}

	let searchDialog: SearchDialog;
	let current: Current | undefined = $state();

	export const snapshot: Snapshot<{
		dialog: typeof searchDialog.snapshot extends Snapshot<infer T> ? T : never;
		current:
			| (NonNullable<typeof current> extends { snapshot: Snapshot<infer T> } ? T : never)
			| undefined;
		autoplay: boolean;
	}> = {
		capture: () => ({
			dialog: searchDialog.snapshot.capture(),
			current: current?.snapshot.capture(),
			autoplay: queue.autoplay
		}),
		restore: (value) => {
			searchDialog.snapshot.restore(value.dialog);
			current?.snapshot.restore(value.current);
			queue.autoplay = value.autoplay;
		}
	};
</script>

<svelte:head>
	<title>Apelle - {queue.code}</title>
</svelte:head>

{#if user.queue_role.permissions.queue.next}
	<NavBarSection menu order={1}>
		<NavBarToggle icons bind:value={queue.autoplay}>
			{$_('navbar.autoplay')}
		</NavBarToggle>
	</NavBarSection>
{/if}

<main>
	<Current
		bind:this={current}
		{queueId}
		bind:playerStateId={queue.player_state_id}
		bind:current={queue.current}
		{user}
		bind:isPlayer
	/>
	<section>
		<h1>{$_('backoffice.partyName')}<code>{queue.code}</code></h1>
		{#if user.queue_role.permissions.queue.enqueue}
			<SearchBar onsubmit={openSearch} placeholder="" />
		{/if}
	</section>
	<section>
		<h1>{$_('backoffice.queue.title')}</h1>
		<table class="queue">
			<tbody>
				{#each queue.queue as song (song.id)}
					<QueuedSongCard {song} queue={queue.id} permissions={user.queue_role.permissions.queue} />
				{/each}
			</tbody>
		</table>
	</section>

	<SearchDialog bind:this={searchDialog} {onSongChosen} />
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

		table.queue {
			width: 100%;
			table-layout: fixed;
			border-spacing: 12px;
		}
	}
</style>
