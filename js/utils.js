
export const wrapBind = cls => Object.entries(cls)
  .reduce(
    (
      acc,
      [
        key,
        method
      ]
    ) => ({
      ...acc,
      [key]: (...args) => method.call(cls, args),
    }),
    {}
  );