import { 
      assert,
      assertArrayContains,
      assertEquals,
      assertMatch,
      assertNotEquals,
      assertStrContains,
      assertStrictEq,
      assertThrows,
      assertThrowsAsync,
      equal,
      unimplemented,
      unreachable,
     } from "https://deno.land/std/testing/asserts.ts";
import { DoublyLinkedList } from './js_test/doublylinkedlist.ts'
import { LinkedList } from './js_test/linkedlist.ts'
import { LinkedLists } from './js_test/nested/linkedlist.ts'
import { DoublyLinkedListJs } from './js_test/nested/nested/doublylinkedlist.js'

Deno.test("js_test/doublylinkedlist.ts ->  (line 203)", () => {
  const testArr = [1, 2, 3, 4, 5, 6, 78, 9, 0, 65];
  const testList = new DoublyLinkedList<number>();
  for (let data of testArr) {
  testList.insertNode(data);
  }
  testList.map((c: number) => c ** 2);
  testList.forEach((c: number, i: number) => assertEquals(c, testArr[i] ** 2));
});

Deno.test("js_test/linkedlist.ts -> Linkedlist.map (line 85)", () => {
  const testArr = [1, 2, 3, 4, 5, 6, 78, 9, 0, 65];
  const testList = new LinkedList<number>();
  for (let data of testArr) {
  testList.insertNode(data);
  }
  testList.map((c: number) => c ** 2);
  testList.forEach((c: number, i: number) => assertEquals(c, testArr[i] ** 2));
});

Deno.test("js_test/linkedlist.ts -> Linkedlist.map 2 (line 97)", () => {
  const testArr = [1, 2, 3, 4, 5];
  const testList = new LinkedList<number>();
  for (let data of testArr) {
  testList.insertNode(data);
  }
  testList.map((c: number) => c ** 2);
  testList.forEach((c: number, i: number) => assertEquals(c, testArr[i] ** 2));
});

Deno.test("js_test/linkedlist.ts -> Linkedlists.compareWith (line 160)", () => {
  const testArr = [1, 2, 3, 4, 5, 6, 78, 9, 0, 65];
  const firstList = new LinkedList<number>();
  const secondList = new LinkedList<number>();
  for (let data of testArr) {
  firstList.insertNode(data);
  secondList.insertNode(data);
  }
  const result = firstList.compareWith(secondList);
  assert(result);
});

Deno.test("js_test/nested/linkedlist.ts -> Linkedlists.map (line 85)", () => {
  const testArr = [1, 2, 3, 4, 5, 6, 78, 9, 0, 65];
  const testList = new LinkedLists<number>();
  for (let data of testArr) {
  testList.insertNode(data);
  }
  testList.map((c: number) => c ** 2);
  testList.forEach((c: number, i: number) => assertEquals(c, testArr[i] ** 2));
});

Deno.test("js_test/nested/linkedlist.ts -> Linkedlist.compareWith (line 148)", () => {
  const testArr = [1, 2, 3, 4, 5, 6, 78, 9, 0, 65];
  const firstList = new LinkedLists<number>();
  const secondList = new LinkedLists<number>();
  for (let data of testArr) {
  firstList.insertNode(data);
  secondList.insertNode(data);
  }
  const result = firstList.compareWith(secondList);
  assert(result);
});

Deno.test("js_test/nested/nested/doublylinkedlist.js -> DoublyLinkedList.map (line 192)", () => {
  const testArr = [1, 2, 3, 4, 5, 6, 78, 9, 0, 65];
  const testList = new DoublyLinkedListJs();
  for (let data of testArr) {
  testList.insertNode(data);
  }
  testList.map((c: number) => c ** 2);
  testList.forEach((c: number, i: number) => assertEquals(c, testArr[i] ** 2));
});

// @ts-ignore
Deno[Deno.internal].runTests({"failFast":true,"reportToConsole":false,"disableLog":true});
