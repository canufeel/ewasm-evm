/* eslint-disable camelcase */
import { EeiBase } from '../../js/eei-base';
import { fromHex, toHex, zeroBuffer } from './common';

export default class EthereumEnvironmentInterfaceMock extends EeiBase {
  constructor ({
    state,
    ctx,
  }) {
    super();
    this.state = state;
    this.ctx = ctx;
  }

  get execBytecode () {
    return fromHex(this.state[this.ctx.address].code);
  }

  ethereum_useGas(amount) {

  }
  ethereum_getGasLeft() {

  }
  ethereum_getAddress(resultOffset) {

  }
  ethereum_getExternalBalance(addressOffset, resultOffset) {

  }
  ethereum_getBlockCoinbase(resultOffset) {

  }
  ethereum_getBlockDifficulty(resultOffset) {

  }
  ethereum_getBlockGasLimit() {}
  ethereum_getBlockHash(number, resultOffset) {}
  ethereum_getBlockNumber() {}
  ethereum_getBlockTimestamp() {}
  ethereum_getTxGasPrice(valueOffset) {}
  ethereum_getTxOrigin(resultOffset) {}
  ethereum_log(
    dataOffset,
    length,
    numberOfTopics,
    topic1,
    topic2,
    topic3,
    topic4
  ) {}
  ethereum_call(
    gas,
    addressOffset,
    valueOffset,
    dataOffset,
    dataLength
  ) {}
  ethereum_callCode(
    gas,
    addressOffset,
    valueOffset,
    dataOffset,
    dataLength
  ) {}
  ethereum_callDelegate(
    gas,
    addressOffset,
    dataOffset,
    dataLength
  ) {}
  ethereum_callStatic(
    gas,
    addressOffset,
    dataOffset,
    dataLength
  ) {}
  ethereum_create(
    valueOffset,
    dataOffset,
    dataLength,
    resultOffset
  ) {}
  ethereum_returnDataCopy(resultOffset, dataOffset, length) {}
  ethereum_getReturnDataSize() {}
  ethereum_finish (dataOffset, length) {
    this.resolve(this.memory.slice(dataOffset, dataOffset + length));
  }
  ethereum_revert (dataOffset, length) {
    this.reject(this.memory.slice(dataOffset, dataOffset + length));
  }
  ethereum_callDataCopy(resultOffset, dataOffset, length) {}
  ethereum_getCallDataSize() {}
  ethereum_getCaller(resultOffset) {}
  ethereum_getCallValue(resultOffset) {}
  ethereum_codeCopy(resultOffset, codeOffset, length) {}
  ethereum_getCodeSize() {}
  ethereum_externalCodeCopy(
    addressOffset,
    resultOffset,
    codeOffset,
    length
  ) {}
  ethereum_getExternalCodeSize(addressOffset) {}
  ethereum_storageLoad (keyOffset, resultOffset) {
    const { memory } = this;
    const key = memory.slice(keyOffset, keyOffset + 32);
    const value = fromHex(this.state[this.ctx.address].storage[toHex(key)]);
    for (let i = resultOffset; i < resultOffset + value.length; i++) {
      memory[i] = value[i];
    }
  }
  ethereum_storageStore (keyOffset, valueOffset) {
    const key = this.memory.slice(keyOffset, keyOffset + 32);
    const value = this.memory.slice(valueOffset, valueOffset + 32);
    if (Buffer.compare(Buffer.from(value), zeroBuffer) === 0) {
      delete this.state[this.ctx.address].storage[toHex(key)];
    } else {
      this.state[this.ctx.address].storage[toHex(key)] = toHex(value);
    }
  }
  ethereum_selfDestruct(addressOffset) {} // -> !
}
