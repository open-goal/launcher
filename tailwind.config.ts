import type {Config} from 'tailwindcss';
import flowBite from "flowbite/plugin";

export default {
  content: [
    "./src/**/*.{html,js,svelte,ts}",
    "./node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}",
  ],

  theme: {
    extend: {
      backgroundColor: {
        background: 'rgb(26, 18, 14)',
        'background-hover': 'rgb(26, 18, 14)',
        primary: '#f77223',
        'primary-hover': '#f75c00',
        secondary: '#311c10',
        'secondary-hover': '#4d2a16'
      },
      textColor: {
        'background': 'rgb(227, 227, 227)',
        'background-hover': 'rgb(255, 90, 31)',
        primary: '#000',
        'primary-hover': '#000',
        secondary: '#f77223',
        'secondary-hover': '#f77223'

      },
      dropShadow: {
        text: "5px 5px 5px rgba(0, 0, 0, 1)",
      },
    },
  },

  plugins: [flowBite],
  darkMode: "class",
} satisfies Config;


