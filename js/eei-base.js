
export class EeiBase {
  intrinsic = {};

  get resolve () {
    return this.intrinsic.resolve;
  }

  get reject () {
    return this.intrinsic.reject;
  }

  get memory () {
    return this.intrinsic.memory;
  }
}
