import { toml } from 'imp:parsers';

{
    const input = 'name = "test"\nvalue = 42\n';
    const parsed = toml.parse(input) as any;
    console.assert(parsed.name === "test", "name should be test");
    console.assert(parsed.value === 42, "value should be 42");
}

{
    const input = '[nested]\na = true\nb = "hello"\n';
    const parsed = toml.parse(input) as any;
    console.assert(parsed.nested.a === true, "nested.a should be true");
    console.assert(parsed.nested.b === "hello", "nested.b should be hello");
}

{
    const input = 'array = [1, 2, 3]\n';
    const parsed = toml.parse(input) as any;
    console.assert(Array.isArray(parsed.array), "array should be array");
    console.assert(parsed.array.length === 3, "array length should be 3");
    console.assert(parsed.array[0] === 1 && parsed.array[1] === 2 && parsed.array[2] === 3, "array values");
}

{
    const input = 'float = 3.14\nnegative = -10\n';
    const parsed = toml.parse(input) as any;
    console.assert(parsed.float === 3.14, "float should work");
    console.assert(parsed.negative === -10, "negative should work");
}

{
    const obj = { name: "test", value: 42 };
    const str = toml.stringify(obj).toString();
    console.assert(str.includes('name = "test"'), "stringify should contain name");
    console.assert(str.includes('value = 42'), "stringify should contain value");
}

{
    const obj = { nested: { a: true, b: "hello" } };
    const str = toml.stringify(obj).toString();
    console.assert(str.includes('[nested]'), "stringify should contain [nested]");
    console.assert(str.includes('a = true'), "stringify should contain a = true");
    console.assert(str.includes('b = "hello"'), "stringify should contain b = hello");
}

{
    let error = false;
    try {
        toml.parse('invalid = = =');
    } catch (e) {
        error = true;
    }
    console.assert(error, "invalid TOML should throw error");
}

{
    let error = false;
    try {
        toml.parse('');
    } catch (e) {
        error = true;
    }
    console.assert(!error, "empty TOML should not throw error");
}

{
    let error = false;
    try {
        toml.parse('key = "unclosed string');
    } catch (e) {
        error = true;
    }
    console.assert(error, "unclosed string should throw error");
}

console.log("ALL PARSERS TOML TESTS PASSED");
