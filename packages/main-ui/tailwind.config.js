/** @type {import('tailwindcss').Config} */
module.exports = {
  // darkMode: "selector",
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {
      screens: {
        "aspect-ratio-landscape": { raw: "(min-aspect-ratio: 1/1)" },
        "aspect-ratio-portrait": { raw: "(max-aspect-ratio: 0.9999/1)" },
      },
      width: {
        610: "610px",
      },
    },
  },

  plugins: [],
};
