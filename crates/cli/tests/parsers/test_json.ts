import { json } from 'imp:parsers';

{
    const input = '{"name":"test","value":42,"nested":{"a":true,"b":null}}';
    const parsed = json.parse(input) as any;
    console.assert(RsString.equals(parsed.name, "test"), "name should be test");
    console.assert(parsed.value === 42, "value should be 42");
    console.assert(parsed.nested.a === true, "nested.a should be true");
    console.assert(parsed.nested.b === null, "nested.b should be null");
}

{
    const obj = { x: 1, y: [1, 2, 3], z: { nested: "value" } };
    const str = json.stringify(obj).toString();
    console.assert(str === '{"x":1,"y":[1,2,3],"z":{"nested":"value"}}', "stringify should work");
}

{
    const arr = [1, "two", false, null];
    const str = json.stringify(arr).toString();
    console.assert(str === '[1,"two",false,null]', "array stringify should work");
}

{
    const input = '[1,2,3]';
    const parsed = json.parse(input) as any[];
    console.assert(Array.isArray(parsed), "should be array");
    console.assert(parsed.length === 3, "length should be 3");
    console.assert(parsed[0] === 1 && parsed[1] === 2 && parsed[2] === 3, "array values");
}

{
    const input = '{"float":3.14,"negative":-10}';
    const parsed = json.parse(input) as any;
    console.assert(parsed.float === 3.14, "float should work");
    console.assert(parsed.negative === -10, "negative should work");
}

{
    let error = false;
    try {
        json.parse('invalid json');
    } catch (e) {
        error = true;
    }
    console.assert(error, "invalid JSON should throw error");
}

{
    let error = false;
    try {
        json.parse('');
    } catch (e) {
        error = true;
    }
    console.assert(error, "empty string should throw error");
}

{
    let error = false;
    try {
        json.parse('{"unclosed": "string');
    } catch (e) {
        error = true;
    }
    console.assert(error, "unclosed string should throw error");
}

{
    let error = false;
    try {
        json.parse('{key: "value"}');
    } catch (e) {
        error = true;
    }
    console.assert(error, "unquoted key should throw error");
}

{
    const set = new Set([1, 2, 3]);
    const str = json.stringify(set as any).toString();
    const parsed = json.parse(str) as any[];
    console.assert(Array.isArray(parsed), "set should become array");
    console.assert(parsed.length === 3, "set length should be 3");
    console.assert(parsed[0] === 1 && parsed[1] === 2 && parsed[2] === 3, "set values");
}

{
    const set = new Set([{ a: 1 }, { b: 2 }]);
    const str = json.stringify(set as any).toString();
    const parsed = json.parse(str) as any[];
    console.assert(Array.isArray(parsed), "set of objects should become array");
    console.assert(parsed.length === 2, "set length should be 2");
    console.assert(parsed[0].a === 1, "set first object");
    console.assert(parsed[1].b === 2, "set second object");
}

{
    const map = new Map([["a", 1], ["b", 2]]);
    const str = json.stringify(map as any).toString();
    const parsed = json.parse(str) as any;
    console.assert(parsed.a === 1, "map key a");
    console.assert(parsed.b === 2, "map key b");
}

{
    const map = new Map([[1, "one"], [2, "two"]]);
    const str = json.stringify(map as any).toString();
    const parsed = json.parse(str) as any;
    console.assert(RsString.equals(parsed["1"], "one"), "map numeric key 1");
    console.assert(RsString.equals(parsed["2"], "two"), "map numeric key 2");
}

{
    const date = new Date("2025-01-01T00:00:00.000Z");
    const str = json.stringify(date as any).toString();
    console.assert(str === '"2025-01-01T00:00:00.000Z"', "date should become ISO string");
}

{
    const regexp = /hello/gi;
    const str = json.stringify(regexp as any).toString();
    console.assert(str.includes("hello"), "regexp should contain pattern");
}

{
    const obj = { fn: () => {}, value: 42 };
    const str = json.stringify(obj).toString();
    const parsed = json.parse(str) as any;
    console.assert(parsed.fn === undefined, "function should be omitted");
    console.assert(parsed.value === 42, "other values should work");
}

{
    const obj: any = {};
    obj.self = obj;
    let error = false;
    try {
        json.stringify(obj);
    } catch (e) {
        error = true;
    }
    console.assert(error, "circular reference should throw error");
}

{
    const input = '{"big":3000000000}';
    const parsed = json.parse(input) as any;
    console.assert(parsed.big === 3000000000, "large integer should not truncate");
}

{
    const input = '{"neg":-3000000000}';
    const parsed = json.parse(input) as any;
    console.assert(parsed.neg === -3000000000, "large negative integer should not truncate");
}

{
    const input = '{"max_i32":2147483647}';
    const parsed = json.parse(input) as any;
    console.assert(parsed.max_i32 === 2147483647, "max i32 should work");
}

{
    const input = '{"over_i32":2147483648}';
    const parsed = json.parse(input) as any;
    console.assert(parsed.over_i32 === 2147483648, "i32+1 should not truncate");
}

{
    const input = '{"text":"Привет мир"}';
    const parsed = json.parse(input) as any;
    console.assert(RsString.equals(parsed.text, "Привет мир"), "cyrillic should parse");
}

{
    const input = '{"emoji":"😀🚀🎉"}';
    const parsed = json.parse(input) as any;
    console.assert(RsString.equals(parsed.emoji, "😀🚀🎉"), "emoji should parse");
}

{
    const obj = { mixed: "hello мир こんにちは 🌍" };
    const str = json.stringify(obj).toString();
    const parsed = json.parse(str) as any;
    console.assert(RsString.equals(parsed.mixed, "hello мир こんにちは 🌍"), "unicode roundtrip");
}

{
    const emptyObj = {};
    const str = json.stringify(emptyObj).toString();
    const parsed = json.parse(str) as any;
    console.assert(JSON.stringify(parsed) === "{}", "empty object roundtrip");
}

{
    const emptyArr: any[] = [];
    const str = json.stringify(emptyArr).toString();
    const parsed = json.parse(str) as any[];
    console.assert(Array.isArray(parsed) && parsed.length === 0, "empty array roundtrip");
}

console.log("ALL PARSERS JSON TESTS PASSED");
