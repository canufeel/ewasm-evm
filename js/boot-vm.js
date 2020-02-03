import fs from 'fs';
import path from 'path';
import { promisify } from 'util';
import {
  EthereumEnvironmentInterfaceIntrinsic,
  skipWrapProperties,
} from './eei-intrinsic';
import { wrapBind } from './utils';

const readFile = promisify(fs.readFile);

const getImportObject = ({
  memory,
  api,
  ...rest
}) => ({
  env: {
    memory,
    ...api,
    ...rest,
  }
});

export const boot = async (eeiImpl) => {
  const memory = new WebAssembly.Memory({ initial: 20 });

  const bytes = await readFile(path.resolve(__dirname, '../out/main.wasm'));
  const eeiInitObj = {
    memory,
    resolve: null,
    reject: null,
    resolved: false,
    eei: eeiImpl,
  };
  const runPromise = new Promise((resolve, reject) => {
    eeiInitObj.resolve = resolve;
    eeiInitObj.reject = reject;
  });
  const eei = new EthereumEnvironmentInterfaceIntrinsic(eeiInitObj);
  const api = wrapBind({
    instance: eeiImpl,
    skip: skipWrapProperties,
  });
  const importObject = getImportObject({
    memory,
    api,
    // eslint-disable-next-line no-console
    logDebug: (arg) => console.log(arg),
  });
  const results = await WebAssembly.instantiate(bytes, importObject);
  const {
    instance: {
      exports: {
        runBytecode,
      }
    }
  } = results;
  return {
    run: async (...args) => {
      runBytecode(...args);
      if (!eeiInitObj.resolved) {
        eeiInitObj.resolve([]); // TODO: is this what should actually happen?
      }
      return runPromise;
    },
    eei
  };
};

export const prepareRunEnv = async ({
  eeiImpl,
  bytecode = eeiImpl.execBytecode,
}) => {
  const {
    eei,
    run,
  } = await boot(eeiImpl);
  const args = eei.prepareEntryArgs(bytecode);
  return {
    run: () => run(...args),
    eei
  };
};
