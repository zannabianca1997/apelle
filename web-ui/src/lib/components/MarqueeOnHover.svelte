<script lang="ts">
    import type { Snippet } from 'svelte';
    import type { ClassValue } from 'svelte/elements';

    const {
        host,
        children,
        class: additionalClass = ''
    }: {
        host: string;
        children: Snippet;
        class?: ClassValue;
    } = $props();

    let element: HTMLElement | null = $state(null);
    let isOverflowing = $derived.by(() => {
        if (!element) return false;
        // Check if the scrollable width is greater than the visible width (horizontal overflow)
        const overflowX = element.scrollWidth > element.offsetWidth;
        // Check if the scrollable height is greater than the visible height (vertical overflow)
        const overflowY = element.scrollHeight > element.offsetHeight;

        return overflowX || overflowY;
    });
</script>

<svelte:element
    this={host}
    bind:this={element}
    class={[
        'title-marquee',
        additionalClass,
        {
            isOverflowing
        }
    ]}
>
    <span class="title-content">{@render children()}</span>
    {#if isOverflowing}
        <span class="title-content">{@render children()}</span>
    {/if}
</svelte:element>

<style lang="scss">
    /* 1. Define the custom CSS Keyframes for Marquee */
    @keyframes scroll {
        0% {
            transform: translateX(0);
        }
        100% {
            transform: translateX(-100%);
        }
    }

    /* 2. Style the H3 container (the viewport) */
    .title-marquee {
        /* Standard truncation properties for non-hovered state */
        overflow: hidden;
        white-space: nowrap;
        min-width: 0; /* Important for flex/grid layouts to prevent the element from forcing its full width */

        /* 3. Style the inner SPAN content (the moving element) */
        .title-content {
            display: inline-block;
            /* Ensure the content naturally extends beyond the container if necessary */
            white-space: nowrap;
        }

        &.isOverflowing:hover {
            .title-content {
                /* Apply the animation for 15 seconds (linear, infinite, and scrolls back and forth) */
                animation: scroll 15s linear infinite;
                /* Force the inner content to be the size of the text */
                display: inline-block;
            }
        }
    }
</style>
