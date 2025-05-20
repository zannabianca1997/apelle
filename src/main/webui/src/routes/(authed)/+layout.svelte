<script lang="ts">
	import { _ } from 'svelte-i18n';
	import NavBar from '$lib/components/navbar/NavBar.svelte';
	import NavBarButton from '$lib/components/navbar/elements/NavBarButton.svelte';
	import authService from '$lib/auth.svelte';
	import IconMusicNote from '~icons/mdi/music-note';
	import IconLogout from '~icons/mdi/logout-variant';
	import { goto } from '$app/navigation';
	import type { Snippet } from 'svelte';
	import NavBarSection from '$lib/components/navbar/NavBarSection.svelte';

	let { children }: { children: Snippet } = $props();

	async function home() {
		await goto('/');
	}
	async function logout() {
		await authService.signout();
		await goto('/authenticate');
	}
</script>

<NavBarSection order={0}>
	<NavBarButton icon={IconMusicNote} onclick={home}>{$_('navbar.title')}</NavBarButton>
</NavBarSection>
<NavBarSection menu order={-1}>
	<NavBarButton icon={IconLogout} onclick={logout}>{$_('navbar.logout')}</NavBarButton>
</NavBarSection>

<NavBar />
{@render children()}
