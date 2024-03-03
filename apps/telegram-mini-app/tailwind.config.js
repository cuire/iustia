/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        "base-100": "var(--tg-theme-secondary-background-color)",
        "base-200": "var(--tg-theme-background-color)",
        "base-content": "var(--tg-theme-text-color)",
        primary: "var(--tg-theme-button-color)",
        "primary-focus": "var(--tg-theme-button-color)",
        "primary-content": "var(--tg-theme-button-text-color)",
        secondary: "var(--tg-theme-link-color)",
        "secondary-focus": "var(--tg-theme-link-color)",
        "secondary-content": "var(--tg-theme-link-color)",
        info: "var(--tg-theme-hint-color)",
        error: "var(--tg-theme-destructive-text-color)",
        accent: "var(--tg-theme-accent-text-color)",
      },
    },
  },
  plugins: [],
};
