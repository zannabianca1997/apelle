<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import TextInput from '$lib/components/forms/TextInput.svelte';
	import { error } from '$lib/errors.svelte';
	/* import SigninForm from './SigninForm.svelte';
	import SignupForm from './SignupForm.svelte'; */

	function onsuccess() {
		const originalUrl = page.url.searchParams.get('original') ?? '/';
		goto(originalUrl);
	}
</script>

<main class="flex flex-col gap-9">
	<!-- <SigninForm {onsuccess} />
	<SignupForm {onsuccess} /> -->
</main>
