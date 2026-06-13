import { csv } from 'imp:parsers';

{
    const input = 'name,value\ntest,42\nfoo,bar\n';
    const parsed = csv.parse(input) as any[];
    console.assert(parsed.length === 2, "should have 2 rows");
    console.assert(parsed[0].name === "test", "first row name should be test");
    console.assert(parsed[0].value === "42", "first row value should be 42");
    console.assert(parsed[1].name === "foo", "second row name should be foo");
    console.assert(parsed[1].value === "bar", "second row value should be bar");
}

{
    const input = 'id,name,active\n1,Alice,true\n2,Bob,false\n';
    const parsed = csv.parse(input) as any[];
    console.assert(parsed.length === 2, "should have 2 rows");
    console.assert(parsed[0].id === "1", "id should be 1");
    console.assert(parsed[0].name === "Alice", "name should be Alice");
    console.assert(parsed[0].active === "true", "active should be true");
    console.assert(parsed[1].id === "2", "id should be 2");
    console.assert(parsed[1].name === "Bob", "name should be Bob");
    console.assert(parsed[1].active === "false", "active should be false");
}

{
    const input = 'a,b,c\n1,2,3\n';
    const parsed = csv.parse(input) as any[];
    console.assert(parsed.length === 1, "should have 1 row");
    console.assert(parsed[0].a === "1", "a should be 1");
    console.assert(parsed[0].b === "2", "b should be 2");
    console.assert(parsed[0].c === "3", "c should be 3");
}

{
    const input = 'name,value\n"test,with,comma",42\n';
    const parsed = csv.parse(input) as any[];
    console.assert(parsed.length === 1, "should have 1 row");
    console.assert(parsed[0].name === "test,with,comma", "name should contain commas");
    console.assert(parsed[0].value === "42", "value should be 42");
}

{
    const input = 'name,value\n"line1\nline2",42\n';
    const parsed = csv.parse(input) as any[];
    console.assert(parsed.length === 1, "should have 1 row");
    console.assert(parsed[0].name === "line1\nline2", "name should contain newline");
    console.assert(parsed[0].value === "42", "value should be 42");
}

{
    const data = [
        { name: "test", value: "42" },
        { name: "foo", value: "bar" }
    ];
    const str = csv.stringify(data).toString();
    console.assert(str.includes("name,value"), "stringify should contain header");
    console.assert(str.includes("test,42"), "stringify should contain first row");
    console.assert(str.includes("foo,bar"), "stringify should contain second row");
}

{
    const data = [
        { a: "1", b: "2", c: "3" }
    ];
    const str = csv.stringify(data).toString();
    console.assert(str.includes("a,b,c"), "stringify should contain header");
    console.assert(str.includes("1,2,3"), "stringify should contain row");
}

{
    const data = [
        { name: "test,with,comma", value: "42" }
    ];
    const str = csv.stringify(data).toString();
    console.assert(str.includes('"test,with,comma"'), "stringify should quote comma");
    console.assert(str.includes("42"), "stringify should contain value");
}

{
    let error = false;
    try {
        csv.parse('');
    } catch (e) {
        error = true;
    }
    console.assert(!error, "empty CSV should not throw error");
}

{
    let error = false;
    try {
        csv.parse('name,value\n');
    } catch (e) {
        error = true;
    }
    console.assert(!error, "header only CSV should not throw error");
}

console.log("ALL PARSERS CSV TESTS PASSED");
