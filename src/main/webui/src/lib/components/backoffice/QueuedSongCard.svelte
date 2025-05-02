<script lang="ts">
	import type { QueuePermissions } from '$lib/apis/apelle.ts';
	import type { ThumbnailQueryDto, Uuid } from '$lib/apis/apelle';
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
	import Thumbnail from './Thumbnail.svelte';

	const {
		queue,
		song,
		permissions
	}: {
		queue: Uuid;
		song: QueuedSong;
		permissions: QueuePermissions;
	} = $props();

	const songId = song.id;

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
		<span>
			{#if song.duration}
				Duration: {song.duration.format($_('backoffice.song.durationFormat'))}
			{/if}
			Likes: {song.likes}
		</span>
	</td>
	<td class="likeButton">
		<button onclick={() => likeSong(song)}>{$_('backoffice.queue.like')}</button>
	</td>
	<td class="likesCount">
		{#if song.user_likes && song.user_likes > 0}
			{$_('backoffice.queue.liked.pre', { default: '' })}
			<em>{song.user_likes} {$_('backoffice.queue.liked.unit')}</em>
			{$_('backoffice.queue.liked.post', { default: '' })}
		{/if}
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
			span {
				font-weight: 300;
				font-size: 16px;
				line-height: 150%;
				letter-spacing: 1%;
			}
		}

		td.likeButton {
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
			}
		}

		td.likesCount {
			text-align: center;
			width: 145px;

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
</style>
