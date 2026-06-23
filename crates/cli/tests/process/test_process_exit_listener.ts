process.on("exit", (code) => {
  console.log("exit listener called with code:", code)
})

process.exit(99)
