import { defineConfig } from "vite";
import solid from "vite-plugin-solid";

export default defineConfig(() => {
  return {
    plugins: [solid()],
    build: {
      assetsInlineLimit: 0,
      outDir: "../backend/static",
      emptyOutDir: true,
    },
  };
});
