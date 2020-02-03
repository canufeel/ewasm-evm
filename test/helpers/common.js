
export const zeroBuffer = Buffer.alloc(32);
export const toHex = (u8Arr) => `0x${Buffer.from(u8Arr).toString('hex')}`;
export const toThirtyTwoByteHex = (u8Arr) => `0x${Buffer.from(u8Arr).toString('hex').padStart(64, '0')}`;
export const fromHex = (hex) => Buffer.from(hex.slice(2), 'hex');

