/** @type {import('tailwindcss').Config} */
import themes from "./theme.config.js"

export default {
    content: ['./src/**/*.{html,svelte,js,ts}'],
    theme: {
        extend: {},
    },
    plugins: [
        require('@tailwindcss/typography'),
        require('daisyui'),
    ],
    daisyui: {
        themes: [themes],
    },
}
