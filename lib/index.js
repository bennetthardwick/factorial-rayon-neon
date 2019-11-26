var addon = require('../native');

addon.factorial(25, (_error, value) => console.log(value));
