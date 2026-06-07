import inq from "imp:inq"

// console.log(await inq.prompt("Your name:"))
// console.log(await inq.select("Which one?", ["Under 18", "Adult"]))
// console.log(await inq.multiSelect("Friut?", ["Apple", "Mango", "Banana", "Orange"]))
// console.log(await inq.password("Write your password"))
// console.log(await inq.passwordWithConfirm("Write your password"))
// console.log(await inq.editor("Describe"))
// const tomorrow = new Date()
// tomorrow.setDate(tomorrow.getDate() + 1)
// console.log(await inq.dateSelect("Describe", { maxDate: tomorrow, default: tomorrow }))
console.log(await inq.confirm("Delete it?"))
