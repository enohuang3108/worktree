export default {
  input: 'dist/cli.js',
  output: {
    dir: 'build',
    format: 'cjs',
    chunkFileNames: '[name].js',
    entryFileNames: 'wt.js'
  },
  external: [],
  platform: 'node',
  banner: '#!/usr/bin/env node'
}