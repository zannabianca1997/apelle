<script lang="ts">
	import type { QueuedSongQueryDto, QueuedSongShortQueryDto, Uuid } from '$lib/apis/apelle';
	import {
		getApiV1QueuesIQueueIdQueueSongId as getFullSong,
		postApiV1QueuesIQueueIdQueueSongIdLikes as postLike
	} from '$lib/apis/apelle';
	import { dayjs } from '$lib/time';
	import { _ } from 'svelte-i18n';

	const {
		queue,
		song: songShort
	}: {
		queue: Uuid;
		song: QueuedSongShortQueryDto;
	} = $props();

	const songId = songShort.id;
	let song: QueuedSongShortQueryDto | QueuedSongQueryDto = $state(songShort);

	function isFullLoaded(
		song: QueuedSongShortQueryDto | QueuedSongQueryDto
	): song is QueuedSongQueryDto {
		return 'user_likes' in song;
	}

	$effect(() => {
		getFullSong(queue, songId).then(({ data }) => (song = data));
	});

	async function likeSong(song: QueuedSongShortQueryDto | QueuedSongQueryDto) {
		await postLike(queue, song.id);
	}
</script>

<tr>
	<td class="iframe"></td>
	<td class="card">
		<h2>{song.name}</h2>
		<span>
			Duration: {dayjs.duration(song.duration).format($_('backoffice.song.durationFormat'))}
			Likes: {song.likes}
		</span>
	</td>
	<td class="likeButton">
		<button onclick={() => likeSong(song)}>{$_('backoffice.queue.like')}</button>
	</td>
	<td class="likesCount">
		{#if isFullLoaded(song) && song.user_likes > 0}
			{$_('backoffice.queue.liked.pre', { default: '' })}
			<em>{song.user_likes} {$_('backoffice.queue.liked.unit')}</em>
			{$_('backoffice.queue.liked.post', { default: '' })}
		{/if}
	</td>
</tr>

<style lang="scss">
	tr {
		height: 99px;

		td.iframe {
			width: 99px;
			height: 99px;

			background-color: #d6d6d6;
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
