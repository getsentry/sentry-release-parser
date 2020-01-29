const fs = require("fs");
const parser = require("./src/parser.js");

test("parse snapshots", () => {
  fs.readdirSync("tests/snapshots");
});
