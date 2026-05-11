import type { Config } from 'tailwindcss';
const config: Config = {
  content: ['./app/**/*.{js,ts,jsx,tsx}', './components/**/*.{js,ts,jsx,tsx}'],
  theme: { extend: {
    colors: { devil: { DEFAULT: '#CC0000', dark: '#990000', light: '#FF3333' } },
    fontFamily: { mono: ['Courier New', 'monospace'] },
  }},
  plugins: [],
};
export default config;
