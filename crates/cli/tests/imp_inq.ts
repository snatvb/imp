import inq from "imp:inq"

console.log(await inq.prompt("Your name:"))
console.log(await inq.select("Which one?", ["Under 18", "Adult"]))
console.log(await inq.multiSelect("Friut?", ["Apple", "Mango", "Banana", "Orange"]))
