import { EeiBase } from './eei-base';

export class AuxApi extends EeiBase {
  constructor (intrinsic) {
    super();
    this.intrinsic = intrinsic;
  }

  humanizeBytecodeCaptureReturn (ptr, ptrLen) {
    const { memory } = this;
    this.resolve(Buffer.from(memory.slice(ptr, ptr + ptrLen)).toString('utf-8'));
  }
}
