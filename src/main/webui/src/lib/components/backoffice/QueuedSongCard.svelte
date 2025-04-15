<script lang="ts">
	import type { ThumbnailQueryDto, Uuid } from '$lib/apis/apelle';
	import { postApiV1QueuesIQueueIdQueueSongIdLikes as postLike } from '$lib/apis/apelle';
	import type { QueuedSong } from '$lib/models/Queue.svelte';
	import { _ } from 'svelte-i18n';

	const {
		queue,
		song
	}: {
		queue: Uuid;
		song: QueuedSong;
	} = $props();

	const songId = song.id;

	async function likeSong(song: QueuedSong) {
		await postLike(queue, song.id);
	}

	let thumbHeight: number = $state(176);
	let thumbWidth: number = $state(99);

	let choosedThumb = $derived.by(() => {
		const thumbScore = (thumb: ThumbnailQueryDto) => {
			return -((thumb.height - thumbHeight) ** 2 + (thumb.width - thumbWidth) ** 2);
		};

		return song.thumbnails?.reduce((t1, t2) => (thumbScore(t1) > thumbScore(t2) ? t1 : t2)).url;
	});
</script>

<tr>
	<td class="thumb" bind:offsetHeight={thumbHeight} bind:offsetWidth={thumbWidth}>
		{#if choosedThumb}
			<img alt="" src={choosedThumb} />
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

		td.thumb {
			width: 176px;
			height: 99px;

			background-color: transparent;

			padding: 0;
			img {
				width: 100%;
				height: 100%;

				display: block;
			}
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
