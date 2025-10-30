<script lang="ts">
    import type { Component, Snippet } from 'svelte';
    import IconOnDefault from '~icons/mdi/checkbox-marked-circle-outline';
    import IconOffDefault from '~icons/mdi/checkbox-blank-circle-outline';
    import TopBarButton from './TopBarButton.svelte';
    import type { SVGAttributes } from 'svelte/elements';

    type IconComponent = Component<
        Pick<SVGAttributes<SVGSVGElement>, 'height' | 'width'>
    >;

    let {
        value = $bindable(false),
        icons = true,
        children
    }: {
        value?: boolean;
        children?: Snippet<[{ value: boolean }]>;
        icons?:
            | boolean
            | {
                  on?: IconComponent;
                  off?: IconComponent;
              };
    } = $props();

    const { on: IconOn = IconOnDefault, off: IconOff = IconOffDefault } =
        typeof icons === 'boolean' || !icons ? {} : icons;

    const Icon = $derived(value ? IconOn : IconOff);

    function onclick() {
        value = !value;
    }
</script>

<TopBarButton icon={icons ? Icon : undefined} {onclick}>
    {@render children?.({ value })}
</TopBarButton>
