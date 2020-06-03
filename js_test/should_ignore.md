````js
/**
*
* @param fn - (data: T, index: number) => T
* @example <caption>Linkedlist.map</caption>
* ```ts
* import { LinkedList } from './js_test/linkedlist.ts'
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
````
