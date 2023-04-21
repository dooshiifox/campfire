module.exports = {
	root: true,
	parser: '@typescript-eslint/parser',
	extends: ['eslint:recommended', 'plugin:@typescript-eslint/recommended', 'prettier'],
	plugins: ['svelte3', '@typescript-eslint'],
	ignorePatterns: ['*.cjs'],
	overrides: [{ files: ['*.svelte'], processor: 'svelte3/svelte3' }],
	settings: {
		'svelte3/typescript': () => require('typescript')
	},
	parserOptions: {
		sourceType: 'module',
		ecmaVersion: 2020
	},
	env: {
		browser: true,
		es2017: true,
		node: true
	},
	rules: {
		"@typescript-eslint/no-inferrable-types": "off",
		// enforce boolean conditions
		'@typescript-eslint/strict-boolean-expressions': 'error'
		// '@typescript-eslint/strict-boolean-expressions': [
		// 	'error',
		// 	{
		// 		allowNullable: false,
		// 		allowString: false,
		// 		allowNumber: false,
		// 		allowNullableObject: false,
		// 		allowNullableBoolean: false,
		// 		allowNullableString: false,
		// 		allowNullableNumber: false,
		// 		allowNullableEnum: false,
		// 		allowAny: false
		// 	}
		// ]
	}
};
