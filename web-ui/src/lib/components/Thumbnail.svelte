<script lang="ts">
    import type { HTMLImgAttributes } from 'svelte/elements';

    // A simple thumbnail implementation

    interface Thumbnail {
        width: number;
        height: number;
        url: string;
    }

    const {
        srcs,
        ...imgProps
    }: { srcs: Thumbnail[] } & Omit<HTMLImgAttributes, 'src'> = $props();

    let clientHeight: number | null = $state(null);
    let clientWidth: number | null = $state(null);

    const src: string | null = $derived.by(() => {
        if (clientHeight === null || clientWidth === null) {
            return null;
        }

        let min_dist = Number.POSITIVE_INFINITY;
        let best_url = null;
        for (const thumb of srcs) {
            const dist =
                (thumb.height - clientHeight) ** 2 +
                (thumb.width - clientWidth) ** 2;
            if (dist < min_dist) {
                min_dist = dist;
                best_url = thumb.url;
            }
        }

        return best_url;
    });
</script>

<img {...imgProps} bind:clientHeight bind:clientWidth {src} />
