import assert from 'assert';
import { prepareRunEnv } from '../js';
import EthereumEnvironmentInterfaceMock from './helpers/eei-mock';
import { getTestsList, transformPostStorage } from './helpers/get-evm-tests';
import { toHex } from './helpers/common';

describe('Smoke tests', () => {
  describe('custom', () => {
    it('basic execution smoke test', async () => {
      const a = 25;
      const b = 26;
      const bytecode = Buffer.from(
        [
          0x60,
          a,
          0x60,
          b,
          0x02,
          0x60,
          0x0,
          0x52,
          0x60,
          0x20,
          0x60,
          0x0,
          0xf3
        ]
      );
      const eeiImpl = new EthereumEnvironmentInterfaceMock({});
      const {
        run,
      } = await prepareRunEnv({
        bytecode,
        eeiImpl
      });
      const result = await run();
      const expected = BigInt(a * b).toString(16).padStart(64, '0');
      assert.equal(Buffer.from(result).toString('hex'), expected);
    });
  });

  describe('Ethereum', () => {
    const testData = getTestsList();
    testData.forEach(({
      fileName,
      data: {
        exec,
        post,
        pre,
        out
      },
    }) => {
      it(`${fileName}`, async () => {
        const eeiImpl = new EthereumEnvironmentInterfaceMock({
          state: pre,
          ctx: exec,
        });
        const {
          run,
        } = await prepareRunEnv({
          eeiImpl,
        });
        const result = await run();
        assert.deepEqual(eeiImpl.state, transformPostStorage(post));
        assert.equal(toHex(result), out);
      });
    });
  });
});
