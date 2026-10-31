import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

const configuredPort = Number(process.env.VITE_DEV_PORT || process.env.TEMPLATE_APP_WEB_PORT || process.env.PORT || 61002);

export default defineConfig({
  plugins: [vue()],
  server: {
    host: process.env.VITE_DEV_HOST || process.env.TEMPLATE_APP_WEB_HOST || "127.0.0.1",
    port: Number.isFinite(configuredPort) ? configuredPort : 61002,
    strictPort: false,
  },
  preview: {
    host: process.env.VITE_PREVIEW_HOST || "127.0.0.1",
    port: Number(process.env.VITE_PREVIEW_PORT || process.env.TEMPLATE_APP_WEB_PORT || 61002),
    strictPort: false,
  },
});
