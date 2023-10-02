/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{html, js}"
  ],
  theme: {
    extend: {
      colors: {
        logo: {
          "primary": "#FFBC2B",
          "secondary": "#FFDC2B"
        }
      }
    },
  },
  plugins: [],
}

