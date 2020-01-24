import assert from 'assert';
import { executeByteCode } from '../js';

describe('Smoke tests', () => {
  it('basic execution smoke test', async () => {
    const bytecode = Buffer.from(
      [
        0x60,
        25,
        0x60,
        26,
        0x02,
        0x60,
        0x0,
        0x52,
        0x60,
        0x0,
        0x60,
        0x20,
        0xf3
      ]
    );
    const result = await executeByteCode(bytecode);
    const expected = '';
    assert.equal(result.toString('hex'), expected);
  });
});
