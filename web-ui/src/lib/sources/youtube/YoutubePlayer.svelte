<script lang="ts">
    import { onMount, untrack, type ComponentProps } from 'svelte';
    import type { YoutubeSongData } from './types';
    import type { PlayerElement } from '../types';
    import config from '$lib/config';

    const {
        song,
        height,
        width,
        position,
        stopped,
        volume
    }: ComponentProps<PlayerElement<YoutubeSongData>> = $props();
    const id = $props.id();

    const seconds = $derived(position.asSeconds());

    let videoId = $derived(song.details.video_id);

    let player: YT.Player | undefined = $state();

    let loaded: boolean = $state(false);

    $effect(() => {
        if (!loaded) return;

        const newId = videoId;
        // do not track the position, or the player would be reset every frame
        const position = untrack(() => seconds);

        player?.loadVideoById?.(newId, position);
    });

    $effect(() => {
        if (!loaded) return;

        // do not track the position, or the player would be reset every frame
        const position = untrack(() => seconds);

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
                height,
                width,
                playerVars: {
                    autoplay: 1,
                    controls: 0,
                    disablekb: 1,
                    enablejsapi: 1,
                    fs: 0,
                    start: seconds
                }
            });

            desyncCheckerId = setInterval(() => {
                const time = player?.getCurrentTime?.();
                if (!time) return;

                if (Math.abs(time - seconds) > config.player.allowedDesync) {
                    player?.seekTo?.(seconds, true);
                }

                const state = player?.getPlayerState?.();
                if (!state) return;
                const isPlayerRunning = [
                    YT.PlayerState.PLAYING,
                    YT.PlayerState.BUFFERING
                ].includes(state);

                if (stopped) {
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
        const newVolume = volume * 100;
        player?.setVolume?.(newVolume);
    });
</script>

<svelte:head>
    <!-- Youtube embedded js, loaded as soon as the component loads -->
    <script src="https://www.youtube.com/iframe_api"></script>
</svelte:head>

<div {id}></div>
