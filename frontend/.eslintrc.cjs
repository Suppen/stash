module.exports = {
	root: true,
	env: {
		es6: true,
		jest: true,
		node: true
	},
	extends: [
		"eslint:recommended",
		"plugin:@typescript-eslint/recommended",
		"plugin:@typescript-eslint/recommended-requiring-type-checking",
		"prettier",
		"plugin:prettier/recommended"
	],
	parser: "@typescript-eslint/parser",
	parserOptions: {
		"ecmaVersion": 2020,
		"sourceType": "module",
		"project": [
			"./tsconfig.json"
		]
	},
	plugins: [
		"@typescript-eslint",
		"prettier",
		"react-hooks"
	],
	rules: {
		"no-var": "error",
		"prefer-const": "warn",
		"eqeqeq": "error",
		"class-methods-use-this": "warn",
		"prettier/prettier": "error",
		"no-eval": "error",
		"no-multi-spaces": "error",
		"react-hooks/rules-of-hooks": "error",
		"react-hooks/exhaustive-deps": "warn"
	}
}
