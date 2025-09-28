<script lang="ts">
  import type { QueuedSong } from '$lib/apis/apelle';
  import isString from '$lib/utils/isString';

  let {
    song
  }: {
    song: QueuedSong;
  } = $props();
</script>

<div>
  {#if !isString(song.song)}
    <span>{song.song.title}</span>
  {:else}
    <span>Loading {song.song}</span>
  {/if}
</div>
