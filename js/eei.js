
export default class EthereumEnvironmentInterface {
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
  ethereum_finish(dataOffset, length) {} //-> !
  ethereum_revert(dataOffset, length) {} // -> !
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
  ethereum_getExternalCodeSize(addressOfset) {}
  ethereum_storageLoad(keyOffset, resultOffset) {}
  ethereum_storageStore(keyOffset, valueOffset) {}
  ethereum_selfDestruct(addressOffset) {} // -> !
}