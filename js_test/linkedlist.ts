type SortFunction<S> = (data1: S, data2: S) => boolean;
export class LinkedList<T> {
  head?: Node<T>;

  insertHead(data: T) {
    if (!this.head) {
      this.head = new Node(data);
    } else {
      let next = this.head;
      this.head = new Node(data, next);
    }
  }

  // Inserts node at a position or at the end if position is not provided
  insertNode(data: T, position?: number) {
    if (!this.head) {
      this.insertHead(data);
      return;
    }
    let current = this.head;
    if (!position) {
      while (current) {
        if (!current.next) {
          current.next = new Node(data);
          break;
        }
        current = current.next;
      }
    } else {
      let index = 1;
      if (position === 0) {
        this.insertHead(data);
        return;
      }
      while (current.next) {
        if (index === position) {
          let next = current.next;
          current.next = new Node(data, next);
          break;
        }
        current = current.next;
        index += 1;
      }
    }
  }

  // delete node at position
  deleteNode(position: number) {
    let index = 1;
    let current = this.head;
    while (current?.next) {
      if (index === position) {
        current.next = current.next.next;
      }
      current = current.next;
      index++;
    }
  }

  // Getting a node at an index from the back in O(n)
  deleteNodeFromBack(position = 0) {
    let pointer1 = this.head;
    let index = 0;
    let pointer2;
    while (pointer1?.next) {
      if (index === position + 1) {
        pointer2 = this.head;
      }
      if (pointer2) {
        pointer2 = pointer2.next;
      }
      pointer1 = pointer1.next;
      index++;
    }
    // delete node
    if (pointer2) {
      pointer2.next = pointer2.next?.next;
    }
  }

  /**
 * 
 * @param fn - (data: T, index: number) => T
 * @example
 * ```ts
 * import { Linkedlist } from './js_test/linkedlist.ts'
 * const testArr = [1, 2, 3, 4, 5, 6, 78, 9, 0, 65];
 * const testList = new LinkedList<number>();
 * for (let data of testArr) {
 *  testList.insertNode(data);
 * }
 * testList.map((c: number) => c ** 2);
 * testList.forEach((c: number, i: number) => assertEquals(c, testArr[i] ** 2));
 * ```
 */
  map(fn: (data: T, index: number) => T) {
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

  /**
 * 
 * @param list - LinkedList<T>
 * @example
 * ```ts
 * import { Linkedlist } from './js_test/linkedlist.ts'
 * const testArr = [1, 2, 3, 4, 5, 6, 78, 9, 0, 65];
 * const firstList = new LinkedList<number>();
 * const secondList = new LinkedList<number>();
 * for (let data of testArr) {
 *   firstList.insertNode(data);
 *   secondList.insertNode(data);
 * }
 * const result = firstList.compareWith(secondList);
 * assert(result);
 * ```
 * @returns boolean
 */
  compareWith(list: LinkedList<T>): boolean {
    let current1 = this.head;
    let current2 = list.head;
    while (current1 && current2) {
      if (current1.data !== current2.data) return false;
      if (current1.next && !current2.next && !current1.next && current2.next) {
        return false;
      }
      current1 = current1.next;
      current2 = current2.next;
    }
    return true;
  }

  sort(fn?: SortFunction<T>) {
    let pointer1 = this.head;
    while (pointer1) {
      let pointer2 = this.head;
      while (pointer2) {
        if (fn) {
          if (fn(pointer1.data, pointer2.data)) {
            pointer1.swap(pointer2);
          }
        } else {
          if (pointer1.data < pointer2.data) {
            pointer1.swap(pointer2);
          }
        }
        pointer2 = pointer2.next;
      }
      pointer1 = pointer1.next;
    }
  }
}
class Node<T> {
  constructor(public data: T, public next?: Node<T>) {}

  swap(other: Node<T>) {
    let temp = this.data;
    this.data = other.data;
    other.data = temp;
  }
}
