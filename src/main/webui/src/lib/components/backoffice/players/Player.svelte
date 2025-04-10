<script lang="ts">
	import type { CurrentSongQueryDto } from '$lib/apis/apelle';
	import { onMount } from 'svelte';
	import { dayjs } from '$lib/time';
	import { _ } from 'svelte-i18n';

	const { current = $bindable() }: { current: CurrentSongQueryDto } = $props();
	let position = $state(dayjs.duration(current.position));

	onMount(() => {
		let interval = setInterval(() => {
			if (!current.stopped) {
				position = dayjs.duration(dayjs().diff(current.starts_at));
			}
		}, 1000);
		return () => clearInterval(interval);
	});
</script>

<div class="iframePlaceholder"></div>
<div class="card">
	<h2>{current.name}</h2>
	<h3>{position.format($_('backoffice.song.durationFormat'))}</h3>
</div>

<style lang="scss">
	.iframePlaceholder {
		flex-shrink: 0;
		width: 244px;
		height: 137px;

		background-color: #d6d6d6;
	}

	.card {
		flex-grow: 1;
		min-width: 0;

		h2 {
			overflow: hidden;
			white-space: nowrap;
			text-overflow: ellipsis;
		}
	}
</style>
