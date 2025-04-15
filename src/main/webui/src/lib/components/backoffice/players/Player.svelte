<script lang="ts">
	import { _ } from 'svelte-i18n';
	import type { CurrentSong } from '$lib/models/Queue.svelte';

	const { current = $bindable() }: { current: CurrentSong } = $props();
</script>

<div class="iframePlaceholder"></div>
<div class="card">
	<h2>{current.name}</h2>
	<h3>{current.position.format($_('backoffice.song.durationFormat'))}</h3>
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
