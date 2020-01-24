require('@babel/core');
require('@babel/register')({
  presets: [
    ['@babel/preset-env', {
      targets: { node: 10 },
    }],
  ],
  plugins: [
    '@babel/transform-runtime'
  ],
});
