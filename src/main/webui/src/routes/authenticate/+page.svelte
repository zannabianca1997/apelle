<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import SigninForm from './SigninForm.svelte';
	import SignupForm from './SignupForm.svelte';

	function onsuccess() {
		const originalUrl = page.url.searchParams.get('original') ?? '/';
		goto(originalUrl);
	}
</script>

<main>
	<SigninForm {onsuccess} --form-color="var(--blue-pill)" />
	<SignupForm {onsuccess} --form-color="var(--red-pill)" />
</main>

<style lang="scss">
	main {
		display: flex;
		gap: 37px;
	}
</style>
