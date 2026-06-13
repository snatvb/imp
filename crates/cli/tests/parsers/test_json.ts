import { json } from 'imp:parsers';

{
    const input = '{"name":"test","value":42,"nested":{"a":true,"b":null}}';
    const parsed = json.parse(input) as any;
    console.assert(parsed.name === "test", "name should be test");
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

console.log("ALL PARSERS JSON TESTS PASSED");
