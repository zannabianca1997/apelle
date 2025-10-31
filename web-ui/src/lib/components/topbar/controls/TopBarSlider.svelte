<script lang="ts">
    import type { Component, Snippet } from 'svelte';
    import type { HTMLInputAttributes } from 'svelte/elements';
    import { _ } from 'svelte-i18n';

    let {
        value = $bindable(0),
        children,
        ...inputAttributes
    }: {
        value?: number;
        children?: Snippet<[{ value: number }]>;
    } & Omit<HTMLInputAttributes, 'type' | 'value'> = $props();

    const id = $props.id();
</script>

<div
    class="flex w-full flex-row-reverse items-center justify-between gap-2 rounded px-2 py-1 font-semibold shadow-md transition duration-300 hover:bg-gray-800"
>
    {#if children}
        <label for="input-{id}">{@render children({ value })}</label>
    {/if}
    <input type="range" bind:value {...inputAttributes} id="input-{id}" />
</div>
