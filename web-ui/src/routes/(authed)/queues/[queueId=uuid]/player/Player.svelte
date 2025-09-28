<script lang="ts">
  import type { Current, IdOrRepSongOneOf, QueueCurrent, TimeRef } from '$lib/apis/apelle';
  import { _ } from 'svelte-i18n';
  import CurrentSongCard from './CurrentSongCard.svelte';
  import isString from '$lib/utils/isString';

  let {
    song
  }: {
    song?: QueueCurrent;
  } = $props();

  function isLoaded(current: Current): current is TimeRef & {
    song: IdOrRepSongOneOf;
  } {
    return !isString(current.song);
  }
</script>

{#if !song || !isLoaded(song)}
  <span>
    {song ? $_('backoffice.currentSong.loading') : $_('backoffice.currentSong.nothingPlaying')}
  </span>
{:else}
  <CurrentSongCard {song} />
{/if}
