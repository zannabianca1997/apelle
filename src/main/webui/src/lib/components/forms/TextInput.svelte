<script lang="ts">
	let id = $props.id();
	let {
		label = '',
		isPassword = false,
		placeholder = label,
		name,
		value = $bindable(),
		error = null
	}: {
		label?: string;
		isPassword?: boolean;
		name?: string;
		placeholder?: string;
		value?: string | null;
		error?: string | null;
	} = $props();
</script>

<div class={{ errored: error != null }}>
	<label for="input-{id}">{label}</label>
	<input id="input-{id}" type={isPassword ? 'password' : 'text'} {name} {placeholder} bind:value />
	{#if !!error}
		<span class="error">{error}</span>
	{/if}
</div>

<style lang="scss">
	div {
		display: flex;
		flex-direction: column;
		gap: 4px;
		flex-grow: var(--input-flex-grow, 0);

		label {
			width: 100%;

			font-weight: 300;
			font-size: 16px;
			line-height: 150%;
			letter-spacing: 1%;
		}

		input {
			width: 100%;
			border: 1px solid #122a42;
			border-radius: 4px;
			border-width: 1px;
			padding: 12px;

			font-weight: 300;
			font-size: 16px;
			line-height: 150%;
			letter-spacing: 1%;

			&::placeholder {
				color: #122a4282;
			}
		}

		&.errored {
			input {
				border: 1px solid red;
			}

			.error {
				width: 100%;
				text-align: center;

				border: 1px solid red;
				border-radius: 2px;

				margin-top: auto;

				font-style: normal;
				color: red;
				font-weight: 300;
				font-size: 12px;
				line-height: 150%;
				letter-spacing: 1%;
			}
		}
	}
</style>
