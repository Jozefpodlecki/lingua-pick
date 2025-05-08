import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'
import tsconfigPaths from 'vite-tsconfig-paths'
import tailwindcss from '@tailwindcss/vite'
import mkcert from'vite-plugin-mkcert'

export default defineConfig({
    plugins: [
        react(),
        tailwindcss(),
        tsconfigPaths(),
        mkcert(),
        {
            name: "markdown-loader",
            transform(code, id) {
                if (id.slice(-3) === ".md") {
                return `export default ${JSON.stringify(code)};`;
                }
            }
        }
    ],
    // test: {
    //     globals: true,
    //     environment: 'happy-dom',
    //     setupFiles: '.vitest/setup',
    //     include: ['**/*.test.{ts,tsx}']
    // }
})