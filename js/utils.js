
export const wrapBind = ({
  instance,
  skip,
}) => Object.getOwnPropertyNames(
  Object.getPrototypeOf(instance)
)
  .reduce(
    (
      acc,
      key
    ) => !!skip[key] ? acc : {
      ...acc,
      [key]: (...args) => instance[key].call(instance, ...args),
    },
    {}
  );
