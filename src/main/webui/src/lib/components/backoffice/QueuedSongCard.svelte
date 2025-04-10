<script lang="ts">
	import type { QueuedSongQueryDto, QueuedSongShortQueryDto, Uuid } from '$lib/apis/apelle';
	import { getApiV1QueuesIQueueIdQueueSongId as getFullSong } from '$lib/apis/apelle';
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

	$effect(() => {
		getFullSong(queue, songId).then(({ data }) => (song = data));
	});
</script>

<li>
	<div class="iframePlaceholder"></div>
	<div class="card">
		<h2>{song.name}</h2>
		<h3>{dayjs.duration(song.duration).format($_('backoffice.song.durationFormat'))}</h3>
	</div>
</li>

<style lang="scss">
	li {
		display: flex;
		align-items: center;
		height: 99px;
		gap: 12px;

		.iframePlaceholder {
			width: 99px;
			height: 99px;

			flex-shrink: 0;
			flex-grow: 0;

			background-color: #d6d6d6;
		}

		.card {
			flex-grow: 1;
			min-width: 0;

			h2 {
				font-weight: 900;
				font-size: 16px;
				line-height: 150%;
				letter-spacing: 1%;

				overflow: hidden;
				white-space: nowrap;
				text-overflow: ellipsis;
			}
			h3 {
				font-weight: 300;
				font-size: 16px;
				line-height: 150%;
				letter-spacing: 1%;
			}
		}
	}
</style>
