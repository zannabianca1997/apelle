<script lang="ts">
	import type { CurrentSongQueryDto } from '$lib/apis/apelle';
	import { onMount } from 'svelte';

	const { current }: { current: CurrentSongQueryDto } = $props();
	let position = $state(current?.position);

	onMount(() => {
		let interval = setInterval(() => {
			//TODO
		}, 1000);
		return () => clearInterval(interval);
	});
</script>

<div class="iframePlaceholder"></div>
<div class="card">
	<h2>{current.name}</h2>
	<h3>{position}</h3>
</div>

<style lang="scss">
	.iframePlaceholder {
		width: 244px;
		height: 137px;

		background-color: #d6d6d6;
	}
</style>
