const fs = require('fs');
const path = require('path');
const { promisify } = require('util');

const readFile = promisify(fs.readFile);

const memory = new WebAssembly.Memory({ initial: 16 });

const file = readFile(path.resolve(__dirname, '../out/main.wasm'));

file.then(bytes => WebAssembly.instantiate(bytes, { env: { log_val: log, memory }})).then(results => {
  const i32 = new Int32Array(memory.buffer);

  for (let i = 0; i < 10; i++) {
    i32[i] = i;
  }
  instance = results.instance;
  instance.exports.add_one(0, 10);
}).catch(console.error);
