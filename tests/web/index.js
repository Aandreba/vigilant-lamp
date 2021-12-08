const path = require('path')
const express = require("express")

const app = express()
const port = 8080;

app.use('/', express.static(__dirname))
app.listen(port)

console.log(`Listening on port ${port}`)