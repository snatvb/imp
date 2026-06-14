import { json } from "imp:parsers";
const r = json.parse('{"name":"test","value":42}');
console.assert(r.name === "test", "name");
console.assert(r.value === 42, "value");
console.log("OK");
