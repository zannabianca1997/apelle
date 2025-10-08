<script lang="ts">
    import SearchBar from '$lib/components/forms/SearchBar.svelte';
    import { _ } from 'svelte-i18n';
    import SearchDialog from './SearchDialog.svelte';
    import { queuesEnqueue, type SearchResponseItem } from '$lib/apis/apelle';

    let {
        queueId
    }: {
        queueId: string;
    } = $props();

    let value = $state('');
    let dialog: SearchDialog;

    async function onsubmit(e: SubmitEvent) {
        e.preventDefault();

        await dialog.open(value);
    }

    function onSongChosen(item: SearchResponseItem): void {
        queuesEnqueue(queueId, item, {
            song: true
        });
    }
</script>

<form {onsubmit}>
    <SearchBar
        label={$_('backoffice.search.label')}
        submitTxt={$_('backoffice.search.submit')}
        bind:value
    />
</form>

<SearchDialog bind:this={dialog} {onSongChosen} />
