import { DllQueue, DllStack } from "./mod.ts";

export class BinarySearchTree<T> {
  constructor(
    public value: T,
    public left?: BinarySearchTree<T>,
    public right?: BinarySearchTree<T>,
  ) {}

  insert(value: T) {
    if (value < this.value) {
      if (this.left) {
        this.left.insert(value);
      } else {
        this.left = new BinarySearchTree(value);
      }
    } else {
      if (this.right) {
        this.right.insert(value);
      } else {
        this.right = new BinarySearchTree(value);
      }
    }
  }

  contains(value: T): boolean {
    if (value === this.value) return true;
    if (value < this.value && this.left) return this.left.contains(value);
    if (value > this.value && this.right) return this.right.contains(value);
    return false;
  }

  getMax(): T {
    if (!this.right) return this.value;
    return this.right.getMax();
  }

  forEach(cb: Function) {
    cb(this.value);
    if (this.left) {
      this.left.forEach(cb);
    }
    if (this.right) {
      this.right.forEach(cb);
    }
  }

  inOrderPrint() {
    if (this.left) {
      this.left.inOrderPrint();
    }
    console.log(this.value);
    if (this.right) {
      this.right.inOrderPrint();
    }
  }

  bftPrint() {
    let level = new DllQueue<BinarySearchTree<T>>();
    level.enqeue(this);
    while (level.size() > 0) {
      const c = level.deqeue();
      if (c?.left) {
        level.enqeue(c.left);
      }
      if (c?.right) {
        level.enqeue(c.right);
      }
      console.log(c?.value);
    }
  }

  dftPrint() {
    const level = new DllStack<BinarySearchTree<T>>();
    level.push(this);
    while (level.size() > 0) {
      const c = level.pop();
      if (c?.left) {
        level.push(c.left);
      }
      if (c?.right) {
        level.push(c.right);
      }
      console.log(c?.value);
    }
  }
}
