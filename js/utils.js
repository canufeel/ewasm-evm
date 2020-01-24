
export const wrapBind = ({
  obj,
  skip,
}) => Object.entries(obj)
  .reduce(
    (
      acc,
      [
        key,
        method
      ]
    ) => !!skip[key] ? acc : {
      ...acc,
      [key]: (...args) => method.call(obj, args),
    },
    {}
  );
