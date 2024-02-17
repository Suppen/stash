/// <reference types="vitest" />
/// <reference types="vite/client" />

import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  build: {
	  target: "esnext",
  },
  server: {
	  proxy: {
		  "/api": {
			  target: "http://localhost:8080",
			  changeOrigin: true,
			  rewrite: (path) => path.replace(/^\/api/, "/v1")
		  }
	  }
  },
  test: {
	globals: true,
    environment: "jsdom",
	setupFiles: ["src/test-setup.ts"],
	css: true
  }
})
