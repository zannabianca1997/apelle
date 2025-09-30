<script lang="ts">
    import SearchBar from '$lib/components/forms/SearchBar.svelte';
    import { _ } from 'svelte-i18n';
</script>

<form>
    <SearchBar
        label={$_('backoffice.search.label')}
        submitTxt={$_('backoffice.search.submit')}
    />
</form>
