import {defineConfig, loadEnv} from "vite";
import react from "@vitejs/plugin-react";
import vitePluginImp from 'vite-plugin-imp'
import path from 'path'

// https://vitejs.dev/config/
export default ({mode}) => {
  const env = loadEnv(mode,process.cwd())
  return defineConfig({
    plugins: [
      react(),
      vitePluginImp({
        optimize: true,
        libList: [
          {
            libName: 'antd',
            libDirectory: 'es',
            // style: (name) => `antd/es/${name}/style`,
            style: (name) => {
              return true;//`antd/es/${name}/style`
            }
          },
        ],
      }),
    ],
    css: {
      preprocessorOptions: {
        less: {
          modifyVars: {
            '@primary-color': '#b59afe',
          },
          javascriptEnabled: true
        }
      }
    },

    resolve:{
      alias:{
        '@':path.resolve(__dirname,'./src')
      }
    },
    // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
    // prevent vite from obscuring rust errors
    clearScreen: false,
    // tauri expects a fixed port, fail if that port is not available
    server: {
      port: 1420,
      strictPort: true,
      proxy: {
        '/backend': {
          target: env.VITE_API,
          changeOrigin: true,
          //rewrite: path => path.replace(/^\/api/, '')
        },
        '/warehouse/picture':{
          target: env.VITE_API,
          changeOrigin: true,
        }
      }
    },
    // to make use of `TAURI_DEBUG` and other env variables
    // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
    envPrefix: ["VITE_", "TAURI_"],
    build: {
      // Tauri supports es2021
      target: ["es2021", "chrome100", "safari13"],
      // don't minify for debug builds
      minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
      // produce sourcemaps for debug builds
      sourcemap: !!process.env.TAURI_DEBUG,
    },
  });
}