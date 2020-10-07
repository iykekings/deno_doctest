/**
 * Helps in running tests
 *
 * @export
 * @class Test ✅
 * @example
 * import { Test } from './test.ts';
 * const test = new Test();
 * console.log('test works)
 */
export class Test {
  /**
   * gives back 4
   * @returns {number}
   * @memberof Test ✅
   * @example
   * import { Test } from './test.ts';
   * let t = new Test();
   * assert.assertEquals(t.test(), 4);
   */
  test(): number {
    return 2 + 2;
  }

  /**
   * gives back 4
   * @returns {number}
   * @memberof Test ✅
   * @example
   * import { Test } from './test.ts';
   * let t = new Test();
   * assert.assertEquals(t.test2(), 4);
   */
  test2(): number {
    return 2 + 2;
  }
}

/**
 * Helps in running another tests
 *
 * @export
 * @class AnotherTest ✅
 * @example
 * import { AnotherTest } from './test.ts';
 * const test = new AnotherTest();
 * console.log('test works)
 */
class AnotherTest {}

/**
 * function test
 * @function funcTest ✅
 * @example
 * import { funcTest } from './test.ts';
 * const test = funcTest();
 * console.log('test works)
 */
function funcTest() {}

/**
 * function test
 * @function funcTestEx ✅
 * @example
 * import { funcTestEx } from './test.ts';
 * const test = funcTestEx();
 * console.log('test works)
 */
export function funcTestEx() {}

/**
 * ObjectWithMethods
 */
const ObjectWithMethods = class {
  /**
   * returns my name
   * @memberof ObjectWithMethods
   * @example
   * import { ObjectWithMethods } from './test.ts';
   * assert.assertEquals(ObjectWithMethods.name(), 'ike');
   */
  name() {
    return 'ike';
  }
  /**
   * returns where I'm from
   * @memberof ObjectWithMethods
   * @example
   * import { ObjectWithMethods } from './test.ts';
   * assert.assertEquals(ObjectWithMethods.from(), 'Enugu');
   */
  from() {
    return 'Enugu';
  }
};

/**
 * ExportedObjectWithMethods
 */
export const ExportedObjectWithMethods = {
  /**
   * returns my name
   * @memberof ExportedObjectWithMethods
   * @example
   * import { ExportedObjectWithMethods } from './test.ts';
   * assert.assertEquals(ExportedObjectWithMethods.name(), 'ike');
   */
  name() {
    return 'ike';
  },
  /**
   * returns where I'm from
   * @memberof ExportedObjectWithMethods
   * @example
   * import { ExportedObjectWithMethods } from './test.ts';
   * assert.assertEquals(ExportedObjectWithMethods.from(), 'Enugu');
   */
  from() {
    return 'Enugu';
  },
};
