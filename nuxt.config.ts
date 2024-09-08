// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2024-04-03',
  devtools: {enabled: true},
  ssr: false,
  devServer: {host: '0.0.0.0'},

  vite: {
      clearScreen: false,
      envPrefix: ['VITE_', 'TAURI_'],
      server: {
          strictPort: true,
          hmr: {
              protocol: 'ws',
              host: '0.0.0.0',
              port: 5183,
          },
      },
  },

  modules: ['@pinia/nuxt', '@nuxtjs/tailwindcss'],
})