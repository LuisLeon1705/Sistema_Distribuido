import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

export default defineConfig({
    plugins: [vue()],
    server: {
        host: '0.0.0.0',
        port: 3000,
        proxy: {
            '/api/auth': {
                target: 'http://localhost:8001',
                changeOrigin: true,
                rewrite: (path) => path.replace(/^\/api\/auth/, '')
            },
            '/api/inventory': {
                target: 'http://localhost:8002',
                changeOrigin: true,
                rewrite: (path) => path.replace(/^\/api\/inventory/, '')
            },
            '/api/products': {
                target: 'http://localhost:8003',
                changeOrigin: true,
                rewrite: (path) => path.replace(/^\/api\/products/, '/api')
            }
        }
    },
    build: {
        outDir: 'dist'
    }
})
