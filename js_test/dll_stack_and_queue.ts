import { DoublyLinkedList } from "./doublylinkedlist.ts";

export class DllQueue<T> {
  storage = new DoublyLinkedList<T>();
  size() {
    return this.storage.length;
  }
  enqeue(value: T) {
      this.storage.insertTail(value);
  }
  deqeue() {
    if(this.size() > 0) return this.storage.removeHead();
  }
}

export class DllStack<T> {
  storage = new DoublyLinkedList<T>();
  size() {
    return this.storage.length;
  }
  push(value: T) {
    this.storage.insertTail(value);
  }
  pop() {
    return this.storage.removeTail();
  }
}
