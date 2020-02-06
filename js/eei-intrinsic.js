
export const skipWrapProperties = {
  constructor: true,
  resolve: true,
  reject: true,
  memory: true,
  initObject: true,
  prepareBytecodeArgs: true,
  skipProperties: true,
  _skipProperties: true,
};

export class EthereumEnvironmentInterfaceIntrinsic {
  _skipProperties = skipWrapProperties;

  get skipProperties () {
    return this._skipProperties;
  }

  set skipProperties (additionalProperties) {
    this._skipProperties = {
      ...this.skipProperties,
      ...additionalProperties,
    };
  }

  constructor (initObject) {
    this.initObject = initObject;
    this.initObject.eei.intrinsic = this;
  }

  get resolve () {
    return (...args) => {
      this.initObject.resolved = true;
      this.initObject.resolve(...args);
    };
  }

  get reject () {
    return (...args) => {
      this.initObject.resolved = true;
      this.initObject.reject(...args);
    };
  }

  get memory () {
    return new Uint8Array(this.initObject.memory.buffer);
  }

  prepareBytecodeArgs (bytecode) {
    const { memory } = this;
    for (let i = 0; i < bytecode.length; i++) {
      memory[i] = bytecode[i];
    }
    return [0, bytecode.length];
  }
}
