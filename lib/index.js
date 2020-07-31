var addon = require('../native');

console.log(addon.hello());
console.log("Require", addon.semver_valid(require, "a.b.c"));