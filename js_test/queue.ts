export class Queue<T> {
  storage: Array<T> = [];
  size() {
    return this.storage.length;
  }
  enqeue(value: T) {
    this.storage.push(value);
  }
  deqeue() {
    if(this.size() > 0) return this.storage.splice(0, 1)[0];
  }
}
