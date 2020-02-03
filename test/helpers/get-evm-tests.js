import fs from 'fs';
import path from 'path';
import { fromHex, toThirtyTwoByteHex } from './common';

const testsRootPath = path.resolve(__dirname, '../../tests/VMTests/vmArithmeticTest');

const parseTestPayload = (testName, {
  [testName]: {
    exec,
    post,
    pre,
    out
  }
}) => ({
  exec,
  post,
  pre,
  out
});

const skipFilePatterns = [
  'exp'
];

export const transformPostStorage = (post) => Object.entries(post)
  .reduce(
    (
      acc,
      [
        addr,
        accountInfo
      ]
    ) => ({
      ...acc,
      [addr]: {
        ...accountInfo,
        storage: Object.entries(accountInfo.storage)
          .reduce(
            (
              innerAcc,
              [
                key,
                value
              ]
            ) => ({
              ...innerAcc,
              [toThirtyTwoByteHex(fromHex(key))]: toThirtyTwoByteHex(fromHex(value)),
            }),
            {}
          )
      }
    }),
    {}
  );

export const getTestsList = () => {
  const files = fs.readdirSync(testsRootPath);
  return files.filter(fileName => skipFilePatterns.findIndex(
    pattern => fileName.includes(pattern)
  ) === -1)
    .map(
      fileName => ({
        fileName,
        data: parseTestPayload(
          fileName.split('.')[0],
          JSON.parse(
            fs.readFileSync(
              path.join(
                testsRootPath,
                fileName
              )
            )
          )
        )
      })
    );
};
