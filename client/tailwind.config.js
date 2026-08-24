export default {
  content: ["./src/**/*.rs", "./index.html"],
  theme: {
    extend: {
      fontFamily: {
        koseka: ["Koseka", "sans-serif"],
        tiempos: ["Tiempos", "serif"],
      },
      keyframes: {
        "ping-sm": {
          "75%, 100%": { transform: "scale(1.5)", opacity: "0" },
        },
        "pulse-strong": {
          "50%": { opacity: "1" },
        }
      },
      animation: {
        "ping-sm": "ping-sm 1.6s cubic-bezier(0, 0, 0.2, 1) infinite",
        "pulse-strong": "pulse 1.6s cubic-bezier(0.4, 0, 0.6, 1) infinite",
        "spin-slow": "spin 3s linear infinite",
      },
    },
  },
  plugins: [],
};
