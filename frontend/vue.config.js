const { defineConfig } = require('@vue/cli-service')
module.exports = defineConfig({
  transpileDependencies: true
})

module.exports = {
  devServer: {
    proxy: {
      '/api': {
        target: 'http://localhost:8000',
        changeOrigin: true,
      },
    },
  },

  // ビルド時の設定オプション
  configureWebpack: {
    // 音声ファイルを含めるために必要なローダーの設定を追加
    module: {
      rules: [
        {
          test: /\.(mp3|ogg)$/,
          use: [
            {
              loader: 'file-loader',
              options: {
                name: '[name].[ext]',
                outputPath: 'assets/audio/', // 出力先のパス
              },
            },
          ],
        },
      ],
    },
  },
};
