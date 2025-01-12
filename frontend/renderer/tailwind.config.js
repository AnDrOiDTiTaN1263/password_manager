const colors = require('tailwindcss/colors')

module.exports = {
  content: [
    './renderer/pages/**/*.{js,ts,jsx,tsx}',
    './renderer/components/**/*.{js,ts,jsx,tsx}',
  ],
  theme: {
    extend: {
      animation:{
        'spin-slow':'spin 3s ease-in-out infinite'
      }
    },
  },
  plugins: [],
}
