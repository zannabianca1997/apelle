<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	/* import SigninForm from './SigninForm.svelte';
	import SignupForm from './SignupForm.svelte'; */

	function onsuccess() {
		const originalUrl = page.url.searchParams.get('original') ?? '/';
		goto(originalUrl);
	}
</script>

<main class="flex gap-9">
	<!-- <SigninForm {onsuccess} />
	<SignupForm {onsuccess} /> -->
</main>
