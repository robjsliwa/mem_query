const { MemoryDB } = require('../../wasm/pkg/memquery_wasm');

console.log(MemoryDB)

let memDB = new MemoryDB();

console.log(memDB);

memDB.create_collection('TestCollection');

console.log('Get collection', memDB.collection('TestCollection'));