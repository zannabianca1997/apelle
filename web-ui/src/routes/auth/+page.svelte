<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import TextInput from '$lib/components/forms/TextInput.svelte';
	import { error } from '$lib/errors.svelte';
	import Form from './Form.svelte';

	function onsuccess() {
		const originalUrl = page.url.searchParams.get('original') ?? '/';
		goto(originalUrl);
	}
</script>

<main class="flex flex-col gap-9 sm:flex-row">
	<Form color="red">
		<TextInput label={$_('login.signinForm.username')} />
		<TextInput label={$_('login.signinForm.password')} password />
	</Form>
	<Form color="blue">
		<TextInput label={$_('login.signupForm.username')} />
		<TextInput label={$_('login.signupForm.password')} password />
		<TextInput label={$_('login.signupForm.passwordCheck')} password />
	</Form>
</main>
