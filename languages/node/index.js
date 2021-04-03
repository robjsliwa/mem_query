const { MemoryDB } = require('../../wasm/pkg/wsmemquery');

console.log(MemoryDB)

let memDB = new MemoryDB();

console.log(memDB);

memDB.create_collection('TestCollection');

console.log('Get collection', memDB.collection('TestCollection'));