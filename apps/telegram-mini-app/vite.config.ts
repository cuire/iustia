import { defineConfig } from "vite";
import path from "path";
import react from "@vitejs/plugin-react-swc";

// Add the declaration for __dirname to avoid the error
declare const __dirname: string;

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      $lib: path.resolve(__dirname, "src/lib"),
    },
  },
});
