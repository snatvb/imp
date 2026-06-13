import { xml } from 'imp:parsers';

{
    const input = '<person><name>test</name><value>42</value></person>';
    const parsed = xml.parse(input) as any;
    console.assert(parsed.name.$text === "test", "name should be test");
    console.assert(parsed.value.$text === "42", "value should be 42");
}

{
    const obj = { person: { name: "test", age: 30 } };
    const str = xml.stringify(obj, "root").toString();
    console.assert(str.includes("<root>"), "stringify should contain <root>");
    console.assert(str.includes("<person>"), "stringify should contain <person>");
    console.assert(str.includes("<name>test</name>"), "stringify should contain <name>test</name>");
    console.assert(str.includes("<age>30</age>"), "stringify should contain <age>30</age>");
}

{
    const input = '<items><item>1</item><item>2</item><item>3</item></items>';
    const parsed = xml.parse(input) as any;
    console.assert(parsed.item !== undefined, "item should exist");
}

{
    const input = '<data><float>3.14</float><negative>-10</negative></data>';
    const parsed = xml.parse(input) as any;
    console.assert(parsed.float.$text === "3.14", "float should work");
    console.assert(parsed.negative.$text === "-10", "negative should work");
}

{
    let error = false;
    try {
        xml.parse('<unclosed>');
    } catch (e) {
        error = true;
    }
    console.assert(error, "unclosed XML should throw error");
}

{
    let error = false;
    try {
        xml.parse('');
    } catch (e) {
        error = true;
    }
    console.assert(error, "empty XML should throw error");
}

{
    let error = false;
    try {
        xml.parse('<root><unclosed></root>');
    } catch (e) {
        error = true;
    }
    console.assert(error, "mismatched tags should throw error");
}

{
    let error = false;
    try {
        xml.parse('not xml at all');
    } catch (e) {
        error = true;
    }
    console.assert(error, "non-XML should throw error");
}

console.log("ALL PARSERS XML TESTS PASSED");
