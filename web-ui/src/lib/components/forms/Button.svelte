<script lang="ts">
    import type { Component, Snippet } from 'svelte';
    import type {
        ClassValue,
        HTMLButtonAttributes,
        SVGAttributes
    } from 'svelte/elements';

    interface Props extends Omit<HTMLButtonAttributes, 'class'> {
        children?: Snippet;
        class?: ClassValue | null | undefined;
        icon?: Component<
            Pick<SVGAttributes<SVGSVGElement>, 'height' | 'width'>
        >;
        tight?: boolean;
    }

    const {
        children,
        class: clazz,
        icon: Icon,
        tight = false,
        ...buttonAttributes
    }: Props = $props();
</script>

<button
    {...buttonAttributes}
    class={[
        'flex flex-row items-center justify-center gap-2 rounded-md border-2 ',
        tight ? 'p-1' : 'p-3',
        'transition duration-300 ease-in-out hover:bg-gray-100/20 hover:shadow-md',
        clazz
    ]}
>
    {#if Icon}
        <Icon width="24px" height="24px" />
    {/if}
    {@render children?.()}
</button>
