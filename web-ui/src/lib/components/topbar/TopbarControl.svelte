<script lang="ts">
    /**
     * This element do not render anything on the dom, but register it to be
     * rendered on the topbar or menu.
     */

    import { onMount } from 'svelte';
    import { register, type TopbarControlProps } from './controls.svelte';

    const props: TopbarControlProps = $props();

    onMount(() => register(props));
</script>
