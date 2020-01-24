module.exports = function (api) {
  api.cache.forever();
  const presets = [
    ['@babel/preset-env', {
      targets: { node: 10 },
    }],
  ];
  const plugins = [
    '@babel/transform-runtime',
  ];
  return {
    presets,
    plugins,
  };
};
