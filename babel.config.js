module.exports = function (api) {
  api.cache.forever();
  const presets = [
    ['@babel/preset-env', {
      targets: { node: 10 },
    }],
  ];
  const plugins = [
    '@babel/transform-runtime',
    '@babel/plugin-proposal-class-properties'
  ];
  return {
    presets,
    plugins,
  };
};
