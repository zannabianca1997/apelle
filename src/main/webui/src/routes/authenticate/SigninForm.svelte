<script lang="ts">
	import './form.scss';

	import { _ } from 'svelte-i18n';
	import TextInput from '$lib/components/forms/TextInput.svelte';
	import { signin } from '$lib/authenticate/signin';

	let username: string | null = null;
	let usernameError: string | null = null;

	let password: string | null = null;
	let passwordError: string | null = null;

	let signinError: string | null = null;

	async function onSubmit() {
		usernameError = !username ? $_('login.signinForm.errors.usernameRequired') : null;
		passwordError = !password ? $_('login.signinForm.errors.passwordRequired') : null;
		if (!!usernameError || !!passwordError) {
			signinError = null;
			return;
		}
		const signinResult = await signin(username!, password!);
		signinError = signinResult?.error || null;
	}
</script>

<form on:submit|preventDefault={onSubmit}>
	<input type="submit" value={$_('login.signinForm.submit')} />
	<TextInput label={$_('login.signinForm.username')} bind:value={username} error={usernameError} />
	<TextInput
		label={$_('login.signinForm.password')}
		isPassword
		bind:value={password}
		error={passwordError}
	/>
	{#if signinError}
		<em class="error">{signinError}</em>
	{/if}
</form>

<style lang="scss">
	form {
		--form-color: var(--blue-pill);
	}
</style>
