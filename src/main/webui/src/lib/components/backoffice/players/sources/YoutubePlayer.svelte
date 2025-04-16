<script lang="ts">
	import { _ } from 'svelte-i18n';
	import type { CurrentSong } from '$lib/models/Queue.svelte';
	import { onMount, untrack } from 'svelte';

	const id = $props.id();
	const { current = $bindable() }: { current: CurrentSong } = $props();

	let videoId = $derived(current.url!.searchParams.get('v')!);

	let player: YT.Player | undefined = $state();

	let loaded: boolean = $state(false);

	$effect(() => {
		if (!loaded) return;

		const newId = videoId;
		// do not track the position, or the player would be reset every frame
		const position = untrack(() => current.position.asSeconds());

		player?.loadVideoById?.(newId, position);
	});

	$effect(() => {
		if (!loaded) return;

		const stopped = current.stopped;
		// do not track the position, or the player would be reset every frame
		const position = untrack(() => current.position.asSeconds());

		if (stopped) {
			player?.pauseVideo?.();
		} else {
			player?.seekTo?.(position, true);
			player?.playVideo?.();
		}
	});

	onMount(() => {
		function load() {
			player = new YT.Player(id, {
				videoId,
				height: '100%',
				width: '100%',
				playerVars: {
					autoplay: 1,
					controls: 0,
					disablekb: 1,
					enablejsapi: 1,
					fs: 0,
					start: current.position.asSeconds()
				}
			});

			loaded = true;
		}

		if (window.YT) {
			load();
		} else {
			window.onYouTubeIframeAPIReady = load;
		}

		return () => {
			player?.destroy?.();
			player = undefined;
			loaded = false;
		};
	});
</script>

<svelte:head>
	<!--Youtube embedded js-->
	<script src="https://www.youtube.com/iframe_api"></script>
</svelte:head>

<div {id}></div>
