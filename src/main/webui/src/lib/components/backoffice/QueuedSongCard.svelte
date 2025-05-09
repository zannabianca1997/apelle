<script lang="ts">
	import type { QueueUserRoleQueuePermissionsQueryDto } from '$lib/apis/apelle.ts';
	import type { Uuid } from '$lib/apis/apelle';
	import {
		postApiV1QueuesIQueueIdQueueSongIdLikes as postLike,
		deleteApiV1QueuesIQueueIdQueueSongId as deleteQueuedSong,
		postApiV1QueuesIQueueIdQueueSongIdPlay as playQueuedSong
	} from '$lib/apis/apelle';
	import type { QueuedSong } from '$lib/models/Queue.svelte';
	import { _, t } from 'svelte-i18n';
	import IconRemove from '~icons/mdi/delete-empty-outline';
	import IconBan from '~icons/mdi/cancel';
	import IconPlay from '~icons/mdi/play';
	import IconVotedOnce from '~icons/mdi/chevron-up';
	import IconVotedTwice from '~icons/mdi/chevron-double-up';
	import IconVotedMany from '~icons/mdi/chevron-triple-up';
	import IconMoveUp from '~icons/mdi/arrow-up';
	import Thumbnail from './Thumbnail.svelte';

	const {
		queue,
		song,
		permissions
	}: {
		queue: Uuid;
		song: QueuedSong;
		permissions: QueueUserRoleQueuePermissionsQueryDto;
	} = $props();

	async function likeSong(song: QueuedSong) {
		await postLike(queue, song.id);
	}

	async function remove() {
		await deleteQueuedSong(queue, song.id);
	}

	async function ban() {}

	async function play() {
		await playQueuedSong(queue, song.id);
	}
</script>

<tr>
	{#if permissions.remove || permissions.ban || permissions.next}
		<td class="buttons"
			><div>
				{#if permissions.remove}
					<button aria-label="remove" onclick={remove}><IconRemove height={24} width={24} /></button
					>
				{/if}
				{#if false && permissions.ban}
					<button aria-label="ban" onclick={ban}><IconBan height={24} width={24} /></button>
				{/if}
				{#if permissions.next}
					<button aria-label="play" onclick={play}><IconPlay height={24} width={24} /></button>
				{/if}
			</div></td
		>
	{/if}
	<td class="thumb">
		{#if song.thumbnails}
			<Thumbnail thumbnails={song.thumbnails} />
		{/if}
	</td>
	<td class="card">
		<h2>{song.name}</h2>
		<ul>
			{#if song.duration}
				<li>
					<span class="label">{$_('backoffice.song.duration')}</span>
					<span class="value">{song.duration.format($_('backoffice.song.durationFormat'))}</span>
				</li>
			{/if}
			<li>
				<span class="label">{$_('backoffice.song.likes')}</span>
				<span class="value">{song.likes}</span>
			</li>
		</ul>
	</td>
	<td class="likes">
		<button onclick={() => likeSong(song)}>
			{$_('backoffice.queue.like')}
			<IconMoveUp height={24} width={24} />
		</button>
		<div class="byUser">
			{#if song.user_likes && song.user_likes > 0}
				<span>
					{$_('backoffice.queue.liked.pre', { default: '' })}
					<em>{song.user_likes} {$_('backoffice.queue.liked.unit')}</em>
					{$_('backoffice.queue.liked.post', { default: '' })}
				</span>
				{#if song.user_likes === 1}
					<IconVotedOnce height={24} width={24} color="#379b46" />
				{:else if song.user_likes === 2}
					<IconVotedTwice height={24} width={24} color="#379b46" />
				{:else}
					<IconVotedMany height={24} width={24} color="#379b46" />
				{/if}
			{/if}
		</div>
	</td>
</tr>

<style lang="scss">
	tr {
		height: 99px;

		td.buttons {
			width: 24px;

			div {
				height: 99px;

				display: flex;
				flex-direction: column;
				justify-content: space-evenly;

				button {
					background: transparent;
					border: 0;
					color: white;

					padding: 0;

					&:hover {
						background: radial-gradient(closest-side, #ffffff88, #00000000);
					}
				}
			}
		}

		td.thumb {
			width: 176px;
			height: 99px;

			background-color: transparent;

			padding: 0;
		}

		td.card {
			h2 {
				font-weight: 900;
				font-size: 16px;
				line-height: 150%;
				letter-spacing: 1%;

				overflow: hidden;
				white-space: nowrap;
				text-overflow: ellipsis;
			}
			ul {
				margin: 0;
				padding: 0;

				list-style: none;

				font-weight: 300;
				font-size: 16px;
				line-height: 150%;
				letter-spacing: 1%;

				display: flex;
				flex-direction: row;
				gap: 10px;

				li {
					.label {
						color: #ffffff88;

						&::after {
							content: ':';
						}
					}
				}
			}
		}

		td.likes {
			width: 175px;

			button {
				font-weight: 900;
				font-size: 16px;
				line-height: 100%;
				letter-spacing: 0%;

				width: 100%;
				border: 0;
				height: 48px;
				border-radius: 4px;
				padding-top: 6px;
				padding-right: 12px;
				padding-bottom: 6px;
				padding-left: 12px;

				color: white;
				background: #379b46;

				cursor: pointer;

				display: flex;
				justify-content: center;
				align-items: center;
				gap: 10px;
			}

			.byUser {
				height: 24px;

				padding-top: 12px;

				display: flex;
				align-items: center;
				justify-content: end;

				font-weight: 300;
				font-size: 16px;
				line-height: 150%;
				letter-spacing: 1%;

				em {
					color: #379b46;
					font-style: normal;
				}
			}
		}
	}
</style>
