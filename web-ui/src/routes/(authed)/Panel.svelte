<script lang="ts">
    import Button from '$lib/components/forms/Button.svelte';
    import { onMount, type Component, type Snippet } from 'svelte';
    import type {
        ClassValue,
        HTMLFormAttributes,
        SVGAttributes
    } from 'svelte/elements';

    interface Props extends Pick<HTMLFormAttributes, 'onsubmit'> {
        icon: Component<Pick<SVGAttributes<SVGSVGElement>, 'height' | 'width'>>;
        title: string;
        activePanel: string;
        active?: boolean;
        children: Snippet;
        color: 'red' | 'blue';
        class?: ClassValue;
    }

    const id = $props.id();
    let {
        icon,
        title,
        activePanel = $bindable(),
        active = false,
        children,
        color,
        onsubmit,
        class: clazz
    }: Props = $props();

    onMount(() => {
        if (active && !activePanel) {
            activePanel = id;
        }
    });
    $effect(() => {
        active = activePanel === id;
    });
</script>

<div
    class={[
        'flex flex-col gap-2 rounded-md',
        color === 'red' ? 'border-redpill' : 'border-bluepill',
        active ? 'grow border-3 p-2' : 'p-0',
        clazz
    ]}
>
    <Button
        {icon}
        class={['w-full', color === 'red' ? 'bg-redpill' : 'bg-bluepill']}
        onclick={() => {
            activePanel = id;
        }}
    >
        {title}
    </Button>
    {#if active}
        <form
            class="flex flex-grow-1 flex-wrap items-center justify-stretch gap-2"
            {onsubmit}
        >
            {@render children()}
        </form>
    {/if}
</div>
