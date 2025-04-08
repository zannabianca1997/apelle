<script lang="ts">
	import type { CurrentSongQueryDto } from '$lib/apis/apelle';

	const { current }: { current: CurrentSongQueryDto & { kind: 'Youtube' } } = $props();
</script>

<div></div>

<style lang="scss">
	div {
		width: 244px;
		height: 137px;

		background-color: #d6d6d6;
	}
</style>
