<script lang="ts">
	import type { CurrentSongQueryDto } from '$lib/apis/apelle';
	import { onMount } from 'svelte';
	import { _ } from 'svelte-i18n';
	import YoutubePlayer from './players/YoutubePlayer.svelte';

	const { current }: { current?: CurrentSongQueryDto } = $props();

	onMount(() => {
		let interval = setInterval(() => {}, 1000);
		return () => clearInterval(interval);
	});
</script>

<section>
	{#if current}
		{#if current.kind == 'Youtube'}
			<YoutubePlayer {current} />
		{:else}
			<!-- No other song kind -->
		{/if}
		<div class="card">
			<h2>{current.name}</h2>
			<h3></h3>
		</div>
	{:else}
		<h1>{$_('backoffice.currentSong.nothingPlaying')}</h1>
	{/if}
</section>

<style lang="scss">
	section {
		display: flex;
		flex-direction: row;
		justify-content: center;
		align-items: center;
		gap: 24px;

		width: 100%;
		height: 161px;
		border-radius: 6px;
		padding: 12px;

		background: linear-gradient(90deg, rgba(55, 155, 70, 0.75) 0%, rgba(36, 101, 46, 0.75) 81.67%);

		h1 {
			margin: 0;
		}
	}
</style>
