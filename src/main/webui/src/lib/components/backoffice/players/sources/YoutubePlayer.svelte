<script lang="ts">
	import { _ } from 'svelte-i18n';
	import type { CurrentSong } from '$lib/models/Queue.svelte';
	import { onMount, untrack } from 'svelte';
	import config from '$lib/config';
	import type { Snapshot } from '@sveltejs/kit';

	const id = $props.id();
	let { current = $bindable(), volume = $bindable(50) }: { current: CurrentSong; volume?: number } =
		$props();

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

	let desyncCheckerId: NodeJS.Timeout | undefined;

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

			desyncCheckerId = setInterval(() => {
				const time = player?.getCurrentTime?.();
				if (!time) return;

				if (Math.abs(time - current.position.asSeconds()) > config.player.allowedDesync) {
					player?.seekTo?.(current.position.asSeconds(), true);
				}

				const state = player?.getPlayerState?.();
				if (!state) return;
				const isPlayerRunning = [YT.PlayerState.PLAYING, YT.PlayerState.BUFFERING].includes(state);

				if (current.stopped) {
					if (isPlayerRunning) {
						player?.stopVideo?.();
					}
				} else {
					if (!isPlayerRunning) {
						player?.playVideo?.();
					}
				}
			}, 1000);

			loaded = true;
		}

		if (window.YT) {
			load();
		} else {
			window.onYouTubeIframeAPIReady = load;
		}

		return () => {
			clearInterval(desyncCheckerId);
			player?.destroy?.();
			player = undefined;
			loaded = false;
		};
	});

	$effect(() => {
		const newVolume = volume;
		player?.setVolume?.(newVolume);
	});

	export const snapshot: Snapshot<{}> = {
		capture: () => ({}),
		restore: (value) => {}
	};
</script>

<svelte:head>
	<!--Youtube embedded js-->
	<script src="https://www.youtube.com/iframe_api"></script>
</svelte:head>

<div {id}></div>
