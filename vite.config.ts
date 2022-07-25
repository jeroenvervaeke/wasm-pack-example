import { defineConfig } from 'vite';
import wasmPack from 'vite-plugin-wasm-pack';

export default defineConfig({
  build: {
    minify: false
  },
  plugins: [wasmPack(['todos-rust/crates/todos-rust'])]
});