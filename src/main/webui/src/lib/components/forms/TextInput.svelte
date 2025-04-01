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
</div>

<style lang="scss">
	div {
		display: flex;
		flex-direction: column;
		gap: 4px;
		width: 100%;

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
		}
	}
</style>
