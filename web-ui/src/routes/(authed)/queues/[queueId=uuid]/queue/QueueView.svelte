<script lang="ts">
	import type { QueueQueue } from '$lib/apis/apelle';
	import QueuedSongCard from './QueuedSongCard.svelte';
	import {_} from "svelte-i18n"

	let {
		songs
	}: {
		songs: QueueQueue;
	} = $props();
</script>

{#if Object.entries(songs).length > 0}
<ol class="flex flex-col gap-3 list-none">
    {#each Object.entries(songs) as [id, song] (id)}
        <li class="h-[99px] w-full">
            <QueuedSongCard {song} />
        </li>
    {/each}
</ol>
{:else}
	<span>{$_("backoffice.queue.empty")}</span>
{/if}
