import assert from 'assert';
import { executeByteCode } from '../js';

describe('Smoke tests', () => {
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
    const result = await executeByteCode(bytecode);
    const expected = BigInt(a * b).toString(16).padStart(64, '0');
    assert.equal(Buffer.from(result).toString('hex'), expected);
  });
});
