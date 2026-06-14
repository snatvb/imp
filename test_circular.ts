import { json } from "imp:parsers";
const obj: any = {};
obj.self = obj;
let error = false;
try {
    json.stringify(obj);
} catch (e) {
    error = true;
}
console.assert(error, "circular reference should throw error");
console.log("OK");
