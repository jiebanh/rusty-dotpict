module.exports = {
  mode: "jit",
  content: {
    files: [
      "src/**/*.rs",
      "index.html",
    ],
  },
  // darkMode: "media",  // "media" or "class"
  theme: {
    // this section define all available colors
    // colors: {
    //   transparent: 'transparent',
    //   current: 'currentColor',
    //   'black-navbar': '#1c1c1d',
    // },
    extend: {
      'black-navbar': '#1c1c1d',
      'sample-nest': {
        100: '#cffafe',
        200: '#a5f3fc',
      },
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
};
