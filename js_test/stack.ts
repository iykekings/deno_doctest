export class Stack<T> {
  storage: Array<T> = [];
  size() {
    return this.storage.length;
  }
  push(value: T) {
    this.storage.push(value);
  }
  pop() {
    this.storage.pop();
  }
}
