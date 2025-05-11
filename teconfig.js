/** @type {import('vite').UserConfigExport} */
export default async ({ host = process.env.TAURI_DEV_HOST }) => ({
  clearScreen: false,
  plugins: [(await import('@vitejs/plugin-react')).default()],
  resolve: {
    alias: { '@': (await import('path')).resolve(import.meta.dirname, 'app') },
  },
  server: {
    strictPort: true,
    host: host || false,
    port: 1420,
    hmr: !host
      ? undefined
      : {
          host: host,
          port: 1421,
          protocol: 'ws',
        },
    watch: {
      ignored: ['**/lib/**'],
    },
  },
})
