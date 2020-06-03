// @ts-ignore
class Node {
  constructor(data, next = null, prev = null) {
    this.data = data;
    this.next = next;
    this.prev = prev;
  }

  swap(other) {
    let temp = this.data;
    this.data = other.data;
    other.data = temp;
  }

  insertAfter(value) {
    let next = this.next;
    this.next = new Node(value, next, this);
    if (next) next.prev = this.next;
  }

  insertBefore(value) {
    let prev = this.prev;
    this.prev = new Node(value, this, prev);
    if (prev) prev.next = this.prev;
  }
}

export class DoublyLinkedListJs {
  head;
  tail;
  length = 0;

  insertHead(data) {
    let newNode = new Node(data);
    this.length += 1;
    if (!this.head && !this.tail) {
      this.head = newNode;
      this.tail = newNode;
    } else {
      newNode.next = this.head;
      if (this.head) {
        this.head.prev = newNode;
      }
      this.head = newNode;
    }
  }
  removeHead() {
    if (!this.head) {
      return null;
    } else {
      this.length--;
      let value = this.head.data;
      let newHead = this.head.next;
      if (newHead) newHead.prev = undefined;
      this.head = newHead;
      return value;
    }
  }
  removeTail() {
    if (!this.head && !this.tail) {
      return null;
    } else {
      this.length -= 1;
      let value = this.tail?.data;
      let newTail = this.tail?.prev;
      if (newTail) newTail.next = undefined;
      this.tail = newTail;
      return value;
    }
  }

  insertTail(data) {
    let newNode = new Node(data);
    this.length += 1;
    if (!this.head) {
      this.head = newNode;
      this.tail = newNode;
    } else if (!this.tail) {
      this.tail = newNode;
    } else {
      newNode.prev = this.tail;
      this.tail.next = newNode;
      this.tail = newNode;
    }
  }

  // Inserts node at a position or at the end if position is not provided
  insertNode(data, position = 0) {
    this.length += 1;
    if (!this.head) {
      this.insertHead(data);
      return;
    }
    if (!position) {
      this.insertTail(data);
    } else {
      let node = new Node(data);
      let current = this.head;
      let index = 1;
      while (current.next) {
        if (index === position) {
          node.prev = current;
          if (current.next) {
            node.next = current.next;
            node.next.prev = node;
          } else {
            this.tail = node;
          }
          current.next = node;
          break;
        }
        current = current.next;
        index += 1;
      }
    }
  }

  // Insert in sorted position
  sortedInsert(data) {
    this.length++;
    if (!this.head) {
      this.head = new Node(data);
    } else if (this.head.data >= data) {
      let newNode = new Node(data, this.head);
      if (newNode.next) {
        newNode.next.prev = newNode;
      }
    } else {
      let current = this.head;
      while (current !== null) {
        if (current.data <= data && !current.next) {
          let newNode = new Node(data, current.next, current);
          current.next = newNode;
          break;
        }
        if (
          current.data <= data &&
          current.next &&
          current.next?.data >= data
        ) {
          let newNode = new Node(data, current.next, current);
          if (current.next) {
            current.next.prev = newNode;
          }
          current.next = newNode;
          break;
        }
        current = current.next;
      }
    }
  }

  // delete node at position
  deleteNode(position = 0) {
    let index = 1;
    let current = this.head;
    while (current?.next) {
      if (index === position) {
        this.length--;
        current.next = current.next.next;
        if (current.next) {
          current.next.prev = current;
        }
      }
      current = current.next;
      index++;
    }
  }

  deleteNodeFromBack(position = 0) {
    let pointer = this.tail;
    let index = 0;
    while (pointer?.prev) {
      if (index === position) {
        this.length--;
        pointer.prev.next = pointer.next;
        if (pointer.next) {
          pointer.next.prev = pointer.prev;
        }
        break;
      }
      pointer = pointer.prev;
      index++;
    }
  }

  /**
   * @param {(data, index) => any} fn
   * @memberof DoublyLinkedListJs
   *
   * @example <caption>DoublyLinkedList.map</caption>
   * ```ts
   * import { DoublyLinkedListJs } from './js_test/nested/nested/doublylinkedlist.js'
   * const testArr = [1, 2, 3, 4, 5, 6, 78, 9, 0, 65];
   * const testList = new DoublyLinkedListJs();
   * for (let data of testArr) {
   *   testList.insertNode(data);
   * }
   * testList.map((c: number) => c ** 2);
   * testList.forEach((c: number, i: number) => assertEquals(c, testArr[i] ** 2));
   * ```
   */
  map(fn) {
    let index = 0;
    let current = this.head;
    while (current) {
      if (fn(current.data, index)) {
        current.data = fn(current.data, index);
      }
      current = current.next;
      index++;
    }
  }

  // print the nodes in reverse
  printReverse() {
    let collector = [];
    let current = this.head;
    while (current) {
      collector.push(current.data);
      current = current.next;
    }
    for (let data of collector.reverse()) {
      console.log(data);
    }
  }

  // runs a fn for each of  the nodes in
  forEach(fn) {
    let current = this.head;
    let i = 0;
    while (current) {
      fn(current.data, i);
      i++;
      current = current.next;
    }
  }

  // reverse
  reverse() {
    let current = this.head;
    let prevNode;
    while (current) {
      let temp = current.next;
      current.next = prevNode;
      prevNode = current;
      current = temp;
    }
    this.head = prevNode;
  }
}
