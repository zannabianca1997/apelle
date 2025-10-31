<script lang="ts">
    import type { Component, Snippet } from 'svelte';
    import type { HTMLButtonAttributes, SVGAttributes } from 'svelte/elements';

    interface Props extends Omit<HTMLButtonAttributes, 'class'> {
        children?: Snippet;
        icon?: Component<
            Pick<SVGAttributes<SVGSVGElement>, 'height' | 'width'>
        >;
    }

    const { children, icon: Icon, ...buttonAttributes }: Props = $props();
</script>

<button
    {...buttonAttributes}
    class="flex w-full flex-row-reverse items-center justify-between gap-2 rounded px-2 py-1 font-semibold shadow-md transition duration-300 hover:bg-gray-800"
>
    {@render children?.()}
    {#if Icon}
        <Icon width="24px" height="24px" />
    {/if}
</button>
