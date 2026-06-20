const url = "https://jsonplaceholder.typicode.com/users/1"

export {}

console.log(`Fetching ${url}...`)
const start = performance.now()

const r = await fetch(url)
const elapsed = (performance.now() - start).toFixed(1)

console.assert(r.ok, "response ok")
console.assert(r.status === 200, "status 200")

const user = await r.json()
console.assert(typeof user.name === "string", "user has name")
console.assert(typeof user.email === "string", "user has email")

console.log("")
console.log("User:")
console.log(`  id:       ${user.id}`)
console.log(`  name:     ${user.name}`)
console.log(`  username: ${user.username}`)
console.log(`  email:    ${user.email}`)
console.log(`  phone:    ${user.phone}`)
console.log(`  website:  ${user.website}`)
console.log(`  company:  ${user.company.name}`)
console.log(`  address:  ${user.address.city}, ${user.address.street}`)
console.log("")
console.log(`Fetched in ${elapsed}ms`)
