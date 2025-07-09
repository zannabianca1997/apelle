import { Config, defineConfig } from 'orval';
import axios from 'axios';
import type { OpenAPIObject } from 'openapi3-ts/oas30';

const specs = axios
	.get<OpenAPIObject>('http://localhost:8080/api-docs/openapi.json', {
		auth: {
			username: 'admin',
			password: 'password'
		}
	})
	.then((res) => res.data);

const config: Promise<Config> = specs.then((target) => ({
	apelle: {
		input: { target },
		output: './src/lib/apis/apelle.ts',
		hooks: {
			afterAllFilesWrite: 'prettier --write'
		}
	}
}));

export default defineConfig(config);
