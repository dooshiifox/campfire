/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			fontFamily: {
				brand: ['Sigmar', 'sans-serif']
			}
		}
	},
	plugins: []
};
