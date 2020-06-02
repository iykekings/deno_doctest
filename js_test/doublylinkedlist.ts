class Node<T> {
  constructor(public data: T, public next?: Node<T>, public prev?: Node<T>) {}

  swap(other: Node<T>) {
    let temp = this.data;
    this.data = other.data;
    other.data = temp;
  }

  insertAfter(value: T) {
    let next = this.next;
    this.next = new Node(value, next, this);
    if (next) next.prev = this.next;
  }

  insertBefore(value: T) {
    let prev = this.prev;
    this.prev = new Node(value, this, prev);
    if (prev) prev.next = this.prev;
  }
}

export class DoublyLinkedList<T> {
  head?: Node<T>;
  tail?: Node<T>;
  length = 0;

  insertHead(data: T) {
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

  insertTail(data: T) {
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
  insertNode(data: T, position?: number) {
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
  sortedInsert(data: T) {
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
          current.data <= data && current.next && current.next?.data >= data
        ) {
          let newNode = new Node(data, current.next, current);
          if (current.next) {
            current.next.prev = newNode;
          }
          current.next = newNode;
          break;
        }
        current = current.next as Node<T>;
      }
    }
  }

  // delete node at position
  deleteNode(position: number) {
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

  // // Getting a node at an index from the back in O(n)
  // deleteNodeFromBack(position = 0) {
  //   let pointer1 = this.head;
  //   let index = 0;
  //   let pointer2: Node<T>;
  //   while (pointer1.next) {
  //     if (index === position + 1) {
  //       pointer2 = this.head;
  //     }
  //     if (pointer2) {
  //       pointer2 = pointer2.next;
  //     }
  //     pointer1 = pointer1.next;
  //     index++;
  //   }
  //   // delete node
  //   pointer2.next = pointer2.next.next;
  // }
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
   * @param {(data: T, index: number) => any} fn
   * @memberof DoublyLinkedList
   * 
   * @example
   * ```ts
   * import { DoublyLinkedList } from './js_test/ddublylinkedlist.ts'
   * const testArr = [1, 2, 3, 4, 5, 6, 78, 9, 0, 65];
   * const testList = new DoublyLinkedList<number>();
   * for (let data of testArr) {
   *   testList.insertNode(data);
   * }
   * testList.map((c: number) => c ** 2);
   * testList.forEach((c: number, i: number) => assertEquals(c, testArr[i] ** 2));
   * ```
   */
  map(fn: (data: T, index: number) => any) {
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
    let collector: Array<T> = [];
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
  forEach(fn: (data: T, index: number) => void) {
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
