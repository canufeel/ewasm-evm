import fs from 'fs';
import path from 'path';
import { promisify } from 'util';
import {
  EthereumEnvironmentInterface,
  skipWrapProperties,
} from './eei';
import { wrapBind } from './utils';

const readFile = promisify(fs.readFile);

const getImportObject = ({
  memory,
  api,
}) => ({
  env: {
    memory,
    ...api,
  }
});

export const boot = async () => {
  const memory = new WebAssembly.Memory({ initial: 20 });

  const bytes = await readFile(path.resolve(__dirname, '../out/main.wasm'));
  const eeiInitObj = {
    memory,
    resolve: null,
    reject: null
  };
  const runPromise = new Promise((resolve, reject) => {
    eeiInitObj.resolve = resolve;
    eeiInitObj.reject = reject;
  });
  const eei = new EthereumEnvironmentInterface(eeiInitObj);
  const api = wrapBind({
    instance: eei,
    skip: skipWrapProperties,
  });
  const importObject = getImportObject({
    memory,
    api,
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
      return runPromise;
    },
    eei
  };
};

export const executeByteCode = async (bytecode) => {
  const {
    eei,
    run,
  } = await boot();
  const args = eei.prepareEntryArgs(bytecode);
  return run(...args);
};
