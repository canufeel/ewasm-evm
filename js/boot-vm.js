import fs from 'fs';
import path from 'path';
import { promisify } from 'util';
import {
  EthereumEnvironmentInterfaceIntrinsic,
  skipWrapProperties,
} from './eei-intrinsic';
import { wrapBind } from './utils';
import { AuxApi } from './aux-api';

const readFile = promisify(fs.readFile);

const getImportObject = ({
  memory,
  ...rest
}) => ({
  env: {
    memory,
    ...rest,
  }
});

const wrapWasmCodeWithPromise = ({
  eeiInitObj,
  method,
}) => async (...args) => {
  const runPromise = new Promise((resolve, reject) => {
    eeiInitObj.resolve = resolve;
    eeiInitObj.reject = reject;
  });
  method(...args);
  if (!eeiInitObj.resolved) {
    eeiInitObj.resolve([]); // TODO: is this what should actually happen?
  }
  return runPromise;
};

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

  const eeiIntrinsic = new EthereumEnvironmentInterfaceIntrinsic(eeiInitObj);
  const eeiApi = wrapBind({
    instance: eeiImpl,
    skip: skipWrapProperties,
  });
  const auxApi = wrapBind({
    instance: new AuxApi(eeiIntrinsic),
    skip: skipWrapProperties,
  });
  const importObject = getImportObject({
    memory,
    ...eeiApi,
    ...auxApi,
    // eslint-disable-next-line no-console
    logDebug: (arg) => console.log(arg),
  });
  const results = await WebAssembly.instantiate(bytes, importObject);
  const {
    instance: {
      exports: {
        runBytecode,
        humanizeBytecode,
      }
    }
  } = results;
  return {
    run: wrapWasmCodeWithPromise({
      method: runBytecode,
      eeiInitObj,
    }),
    humanizeBytecode: wrapWasmCodeWithPromise({
      method: humanizeBytecode,
      eeiInitObj,
    }),
    eei: eeiIntrinsic,
  };
};

export const prepareRunEnv = async ({
  eeiImpl,
  bytecode = eeiImpl.execBytecode,
}) => {
  const {
    eei,
    run,
    humanizeBytecode,
  } = await boot(eeiImpl);
  return {
    run: () => {
      const runBytecodeArgs = eei.prepareBytecodeArgs(bytecode);
      return run(...runBytecodeArgs);
    },
    humanizeBytecode: () => {
      const parseBytecodeArgs = eei.prepareBytecodeArgs(bytecode);
      return humanizeBytecode(...parseBytecodeArgs);
    },
    eei
  };
};
