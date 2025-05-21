<script lang="ts">
	import type { ThumbnailQueryDto } from '$lib/apis/apelle';

	const {
		thumbnails
	}: {
		thumbnails: ThumbnailQueryDto[];
	} = $props();

	function thumbScore(thumb: ThumbnailQueryDto, height: number, width: number) {
		return -((thumb.height - height) ** 2 + (thumb.width - width) ** 2);
	}

	let offsetHeight: number = $state(0);
	let offsetWidth: number = $state(0);

	const src = $derived(
		thumbnails.reduce((t1, t2) =>
			thumbScore(t1, offsetHeight, offsetWidth) > thumbScore(t2, offsetHeight, offsetWidth)
				? t1
				: t2
		).url
	);
</script>

<img alt="thumbnail" {src} bind:offsetHeight bind:offsetWidth />

<style lang="scss">
	img {
		width: var(--thumb-width, 100%);
		height: var(--thumb-height, 100%);

		display: block;
	}
</style>
