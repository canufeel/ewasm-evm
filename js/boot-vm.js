import fs from 'fs';
import path from 'path';
import { promisify } from 'util';
import EthereumEnvironmentInterface from "./eei";
import { wrapBind } from "./utils";

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

const boot = async () => {
  const memory = new WebAssembly.Memory({ initial: 16 });

  const bytes = await readFile(path.resolve(__dirname, '../out/main.wasm'));
  const api = wrapBind(new EthereumEnvironmentInterface());
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
  return runBytecode;
};