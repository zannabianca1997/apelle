<script lang="ts">
    import type { Snippet } from 'svelte';
    import type { ClassValue } from 'svelte/elements';

    const {
        host = 'span',
        children,
        class: additionalClass = ''
    }: {
        host?: string;
        children: Snippet;
        class?: ClassValue;
    } = $props();

    let content: HTMLElement | null = $state(null);

    let containerWidth = $state(0);

    let isOverflowing = $derived.by(() => {
        if (!content) return false;
        // Check if the scrollable width is greater than the visible width (horizontal overflow)
        const overflowX = content.scrollWidth > containerWidth;

        return overflowX;
    });
</script>

<svelte:element
    this={host}
    bind:offsetWidth={containerWidth}
    class={[
        'title-marquee',
        additionalClass,
        {
            isOverflowing
        }
    ]}
>
    <span class="title-content" bind:this={content}>{@render children()}</span>
    {#if isOverflowing}
        <span class="title-content">{@render children()}</span>
    {/if}
</svelte:element>

<style lang="scss">
    @keyframes scroll {
        0% {
            transform: translateX(0);
        }
        100% {
            transform: translateX(-100%);
        }
    }

    .title-marquee {
        overflow: hidden;
        white-space: nowrap;
        min-width: 0;

        .title-content {
            display: inline-block;
            white-space: nowrap;
        }

        &.isOverflowing:hover {
            .title-content {
                animation: scroll 15s linear infinite;
                display: inline-block;
            }
        }
    }
</style>
