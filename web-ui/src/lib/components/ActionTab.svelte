<script lang="ts">
    import type { QueueUserAction } from '$lib/apis/apelle';
    import type { Component, ComponentProps } from 'svelte';
    import type { ClassValue, SVGAttributes } from 'svelte/elements';

    export interface Action {
        permission: QueueUserAction;
        label: string;
        onclick?: () => void;
        Icon: Component<Pick<SVGAttributes<SVGSVGElement>, 'height' | 'width'>>;
    }

    const {
        actions,
        permissions,
        iconsSize: iconsSizeProps = 24,
        direction = 'row',
        class: clazz
    }: {
        actions: Action[];
        permissions: QueueUserAction[];
        iconsSize?:
            | ComponentProps<Action['Icon']>['height']
            | ComponentProps<Action['Icon']>;
        direction?: 'col' | 'row';
        class?: ClassValue;
    } = $props();

    const iconsSize: ComponentProps<Action['Icon']> = $derived.by(() => {
        if (
            typeof iconsSizeProps === 'number' ||
            typeof iconsSizeProps === 'string'
        ) {
            return {
                height: iconsSizeProps,
                width: iconsSizeProps
            };
        }

        if (iconsSizeProps === null) {
            return {};
        }

        return iconsSizeProps;
    });
</script>

{#if actions.length > 0}
    <div
        class={[
            'flex gap-2',
            direction === 'col' ? 'flex-col' : 'flex-row',
            clazz
        ]}
    >
        {#each actions as { permission, label, onclick, Icon }, i (i)}
            {#if permissions.includes(permission)}
                <button
                    aria-label={label}
                    {onclick}
                    class={[
                        'cursor-pointer rounded-lg border-0 p-1 shadow-lg',
                        'transition-all hover:bg-[#2e7d37]',
                        'focus:ring-4 focus:ring-[#379B46]/50 focus:outline-none',
                        'group relative'
                    ]}
                >
                    <Icon {...iconsSize} />
                    <span
                        class={[
                            'absolute z-10 hidden rounded bg-gray-800 px-2 py-1 text-xs text-white group-hover:block',
                            'top-full left-1/2 mt-2 -translate-x-1/2 transform'
                        ]}
                    >
                        {label}
                    </span>
                </button>
            {/if}
        {/each}
    </div>
{/if}
