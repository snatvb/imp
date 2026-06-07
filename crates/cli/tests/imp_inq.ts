import inq from "imp:inq"

console.log(await inq.prompt("How are you?"))
console.log(await inq.select("Which one?", ["Under 18", "Adult"]))
