/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {
      fontFamily: {
        "figtree": ['Figtree', 'sans-serif']
      },
      colors: {
        'black-bean': { DEFAULT: '#301714', 100: '#090404', 200: '#130908', 300: '#1c0d0c', 400: '#251210', 500: '#301714', 600: '#6d342e', 700: '#ac5248', 800: '#cb8982', 900: '#e5c4c0' }, 'burnt-orange': { DEFAULT: '#b85e2a', 100: '#251308', 200: '#492511', 300: '#6e3819', 400: '#924b21', 500: '#b85e2a', 600: '#d57a45', 700: '#df9b73', 800: '#eabca2', 900: '#f4ded0' }, 'rust': { DEFAULT: '#bc3411', 100: '#250a03', 200: '#4b1407', 300: '#701f0a', 400: '#95290e', 500: '#bc3411', 600: '#ea481f', 700: '#ef7557', 800: '#f5a38f', 900: '#fad1c7' }, 'english-violet': { DEFAULT: '#4e4d64', 100: '#101014', 200: '#201f28', 300: '#2f2f3d', 400: '#3f3e51', 500: '#4e4d64', 600: '#6c6a8a', 700: '#908fa8', 800: '#b5b4c5', 900: '#dadae2' }, 'dark-purple': { DEFAULT: '#1c192d', 100: '#060509', 200: '#0b0a12', 300: '#110f1c', 400: '#171425', 500: '#1c192d', 600: '#3f3867', 700: '#62589f', 800: '#958ec1', 900: '#cac6e0' }
      }
    },
  },
  plugins: [

    require('daisyui'),
  ],
  daisyui: {
    themes: [
      {
        rustic: {
          "primary": "#301714",
          "secondary": "#B85E2A",
          "accent": "#bc3411",
          "neutral": "#4E4D64",
          "dark": "#1C192D",
          "base-100": "#201f28",
        },
      },
    ]
  }
};
