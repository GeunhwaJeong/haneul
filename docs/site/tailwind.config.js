// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

const defaultTheme = require("tailwindcss/defaultTheme");

module.exports = {
  corePlugins: {
    preflight: false, // disable Tailwind's reset
  },
  content: ["./src/**/*.{js,jsx,ts,tsx}", "./docs/**/*.mdx"], // my markdown stuff is in ../docs, not /src
  safelist: ["text-haneul-success-dark"],
  darkMode: ["class", '[data-theme="dark"]'], // hooks into docusaurus' dark mode settings
  theme: {
    extend: {
      fontFamily: {
        sans: ["Inter", ...defaultTheme.fontFamily.sans],
        twkeverett: ["Twkeverett"],
      },
      colors: {
        "haneul-black": "var(--haneul-black)",
        "haneul-blue-primary": "rgb(var(--haneul-blue-primary)/<alpha-value>)",
        "haneul-blue": "var(--haneul-blue)",
        "haneul-blue-bright": "rgb(var(--haneul-blue-bright)/<alpha-value>)",
        "haneul-blue-light": "rgb(var(--haneul-blue-light)/<alpha-value>)",
        "haneul-blue-lighter": "var(--haneul-blue-lighter)",
        "haneul-blue-dark": "rgb(var(--haneul-blue-dark)/<alpha-value>)",
        "haneul-blue-darker": "var(--haneul-blue-darker)",
        "haneul-hero": "var(--haneul-hero)",
        "haneul-hero-dark": "var(--haneul-hero-dark)",
        "haneul-steel": "var(--haneul-steel)",
        "haneul-steel-dark": "var(--haneul-steel-dark)",
        "haneul-steel-darker": "var(--haneul-steel-darker)",
        "haneul-header-nav": "var(--haneul-header-nav)",
        "haneul-success": "var(--haneul-success)",
        "haneul-success-dark": "var(--haneul-success-dark)",
        "haneul-success-light": "var(--haneul-success-light)",
        "haneul-issue": "var(--haneul-issue)",
        "haneul-issue-dark": "var(--haneul-issue-dark)",
        "haneul-issue-light": "var(--haneul-issue-light)",
        "haneul-warning": "var(--haneul-warning)",
        "haneul-warning-dark": "var(--haneul-warning-dark)",
        "haneul-warning-light": "var(--haneul-warning-light)",
        "haneul-code": "var(--haneul-code)",
        "haneul-gray-3s": "rgb(var(--haneul-gray-3s)/<alpha-value>)",
        "haneul-gray-5s": "rgb(var(--haneul-gray-5s)/<alpha-value>)",
        "haneul-gray": {
          35: "rgb(var(--haneul-gray-35)/<alpha-value>)",
          40: "rgb(var(--haneul-gray-40)/<alpha-value>)",
          45: "rgb(var(--haneul-gray-45)/<alpha-value>)",
          50: "var(--haneul-gray-50)",
          55: "rgb(var(--haneul-gray-55)/<alpha-value>)",
          60: "var(--haneul-gray-60)",
          65: "var(--haneul-gray-65)",
          70: "var(--haneul-gray-70)",
          75: "var(--haneul-gray-75)",
          80: "var(--haneul-gray-80)",
          85: "var(--haneul-gray-85)",
          90: "var(--haneul-gray-90)",
          95: "var(--haneul-gray-95)",
          100: "var(--haneul-gray-100)",
        },
        "haneul-grey": {
          35: "rgb(var(--haneul-gray-35)/<alpha-value>)",
          40: "rgb(var(--haneul-gray-40)/<alpha-value>)",
          45: "rgb(var(--haneul-gray-45)/<alpha-value>)",
          50: "var(--haneul-gray-50)",
          55: "rgb(var(--haneul-gray-55)/<alpha-value>)",
          60: "var(--haneul-gray-60)",
          65: "var(--haneul-gray-65)",
          70: "var(--haneul-gray-70)",
          75: "var(--haneul-gray-75)",
          80: "var(--haneul-gray-80)",
          85: "var(--haneul-gray-85)",
          90: "var(--haneul-gray-90)",
          95: "var(--haneul-gray-95)",
          100: "var(--haneul-gray-100)",
        },
        "haneul-disabled": "rgb(var(--haneul-disabled)/<alpha-value>)",
        "haneul-link-color-dark": "var(--haneul-link-color-dark)",
        "haneul-link-color-light": "var(--haneul-link-color-light)",
        "haneul-ghost-white": "var(--haneul-ghost-white)",
        "haneul-ghost-dark": "var(--haneul-ghost-dark)",
        "ifm-background-color-dark": "var(--ifm-background-color-dark)",
        "haneul-white": "rgb(var(--haneul-white)/<alpha-value>)",
        "haneul-card-dark": "rgb(var(--haneul-card-dark)/<alpha-value>)",
        "haneul-card-darker": "rgb(var(--haneul-card-darker)/<alpha-value>)",
      },
      borderRadius: {
        haneul: "40px",
      },
      boxShadow: {
        haneul: "0px 0px 4px rgba(0, 0, 0, 0.02)",
        "haneul-button": "0px 1px 2px rgba(16, 24, 40, 0.05)",
        "haneul-notification": "0px 0px 20px rgba(29, 55, 87, 0.11)",
      },
      gradientColorStopPositions: {
        36: "36%",
      },
    },
  },
  plugins: [
    function ({ addUtilities }) {
      const arrowMask = `url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><path d='M8.12 4.12a1 1 0 0 1 1.41 0l6.35 6.35a1 1 0 0 1 0 1.41l-6.35 6.35a1 1 0 1 1-1.41-1.41L13.59 12 8.12 6.53a1 1 0 0 1 0-1.41z'/></svg>") no-repeat center / contain`;

      addUtilities({
        ".mask-arrow": {
          transition: "transform 0.2s ease",
          background: "currentColor",
          WebkitMask: arrowMask,
          mask: arrowMask,
        },
      });
    },
  ],
};
