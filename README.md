# MemQuery

[![memquery](https://github.com/robjsliwa/mem_query/actions/workflows/rust.yml/badge.svg)](memquery)
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

MemQuery is simple library for creating, querying, and updating in memory documents that are represented as JSON objects and queried using Mongodb like operators.

This is not a database and it is not trying to do any optimizations.  It is meant for unit tests or simple projects that require small in memory document store.

The library uses async API that support tokio.  The library also has a sync API that may be enabled via `sync` feature flag.

# Example Usage

## Create Database

```
use memquery::{doc, errors::Error, memdb::MemDb, query};
```

```
let memdb = MemDb::new();
memdb.create_collection("TestCollection").await;
```

## Create Collection

```
memdb.create_collection("TestCollection").await;
```

## Get Collection Handle

```
let coll = memdb.collection("TestCollection").await?;
```

## Insert Document

```
coll.insert(doc!({ "name": "Tom", "age": 25 })).await?;
```

## Find Document

```
let docs = coll.find(query!({"name": "Tom", "age": 25})).await?;

assert_eq!(docs.len(), 1);
assert_eq!(docs[0]["name"], "Tom");
```

## Logical Query Operators

### $and

```
let docs = coll
    .find(query!({ "$and": [{ "name": "Bob" }, { "age": 20 }] }))
    .await?;
```

### $or

```
let docs = coll
    .find(query!({ "$or": [{ "name": "Bob" }, { "age": 30 }] }))
    .await?;
```

## Comparison Query Operators

### $eq

Compare on field:

```
let docs = coll.find(query!({ "qty": { "$eq": 20 } })).await?;
```

Or in embedded document:

```
let docs = coll.find(query!({ "item.name": { "$eq": "ab" } })).await?;
```

You can also compare array with embedded arrays:

```
coll
    .insert(doc!({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] }))
    .await?;
  coll
    .insert(doc!({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] }))
    .await?;
  coll
    .insert(doc!({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] }))
    .await?;
  coll
    .insert(doc!({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] }))
    .await?;
  coll
    .insert(
      doc!({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
    )
    .await?;

  let docs = coll
    .find(query!({ "tags": { "$eq": [ "A", "B" ] } }))
    .await?;

  assert_eq!(docs.len(), 2);
  assert_eq!(docs[0]["item"]["name"], "ij");
  assert_eq!(docs[1]["item"]["name"], "mn");
```

Or value in the embedded array:

```
coll
    .insert(doc!({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] }))
    .await?;
  coll
    .insert(doc!({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] }))
    .await?;
  coll
    .insert(doc!({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] }))
    .await?;
  coll
    .insert(doc!({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] }))
    .await?;
  coll
    .insert(
      doc!({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
    )
    .await?;

  let docs = coll.find(query!({ "tags": { "$eq": "B" } })).await?;

  assert_eq!(docs.len(), 4);
  assert_eq!(docs[0]["item"]["name"], "ab");
  assert_eq!(docs[1]["item"]["name"], "cd");
  assert_eq!(docs[2]["item"]["name"], "ij");
  assert_eq!(docs[3]["item"]["name"], "xy");
```

### $gt

```
let docs = coll.find(query!({ "qty": { "$gt": 20 } })).await?;
```

### $gte

```
let docs = coll.find(query!({ "qty": { "$gte": 20 } })).await?;
```

### $lt

```
let docs = coll.find(query!({ "qty": { "$lt": 20 } })).await?;
```

### $lte

```
let docs = coll.find(query!({ "qty": { "$lte": 20 } })).await?;
```

## Update Document

This shows examples how to use `find_and_update` API.

Update document by replacing entire document:

```
let memdb = MemDb::new();
memdb.create_collection("TestCollection").await;
let coll = memdb.collection("TestCollection").await?;
coll.insert(doc!({ "name": "Rob", "age": 25 })).await?;
coll.insert(doc!({ "name": "Bob", "age": 20 })).await?;
coll.insert(doc!({ "name": "Tom", "age": 30 })).await?;

let docs_updated = coll
  .find_and_update(
    query!({"name": "Bob"}),
    update!({"nickname": "Bobcat", "voice": "meow"}),
  )
  .await?;

assert_eq!(docs_updated, 1);

let docs = coll.find(query!({"nickname": "Bobcat"})).await?;
assert_eq!(docs.len(), 1);
assert_eq!(docs[0]["voice"], "meow");
```

Update specific field(s) in the document:

```
let coll = memdb.collection("TestCollection").await?;
coll.insert(doc!({ "name": "Rob", "age": 25 })).await?;
coll.insert(doc!({ "name": "Bob", "age": 20 })).await?;
coll.insert(doc!({ "name": "Tom", "age": 30 })).await?;

let docs_updated = coll
  .find_and_update(
    query!({"name": "Bob"}),
    update!({"$set": { "name": "Roy", "age": 21, "email": "test@test.com"}}),
  )
  .await?;

assert_eq!(docs_updated, 1);

let docs = coll.find(query!({"name": "Roy"})).await?;
assert_eq!(docs.len(), 1);
assert_eq!(docs[0]["age"], 21);
assert_eq!(docs[0]["email"], "test@test.com");
```

Update document to remove field:

```
let memdb = MemDb::new();
memdb.create_collection("TestCollection").await;
let coll = memdb.collection("TestCollection").await?;
coll.insert(doc!({ "name": "Rob", "age": 25 })).await?;
coll.insert(doc!({ "name": "Bob", "age": 20 })).await?;
coll.insert(doc!({ "name": "Tom", "age": 30 })).await?;

let docs_updated = coll
  .find_and_update(
    query!({ "name": "Bob" }),
    update!({ "$set": { "name": "Roy", "age": 21, "email": "test@test.com" }}),
  )
  .await?;

assert_eq!(docs_updated, 1);

let docs = coll.find(query!({"name": "Roy"})).await?;
assert_eq!(docs.len(), 1);
assert_eq!(docs[0]["age"], 21);
assert_eq!(docs[0]["email"], "test@test.com");

let docs_updated2 = coll
  .find_and_update(
    query!({ "name": "Roy" }),
    update!({ "$unset": { "email": "" }}),
  )
  .await?;

assert_eq!(docs_updated2, 1);

let docs = coll.find(query!({"name": "Roy"})).await?;
assert_eq!(docs.len(), 1);
assert_eq!(docs[0]["age"], 21);
assert_eq!(docs[0]["email"], serde_json::Value::Null);
```

Increment value of the field in the document:

```
let memdb = MemDb::new();
memdb.create_collection("TestCollection").await;
let coll = memdb.collection("TestCollection").await?;
coll.insert(doc!({ "name": "Rob", "age": 25 })).await?;
coll.insert(doc!({ "name": "Bob", "age": 20 })).await?;
coll.insert(doc!({ "name": "Tom", "age": 30 })).await?;

let docs_updated = coll
  .find_and_update(query!({"name": "Bob"}), update!({"$inc": { "age": -5 }}))
  .await?;

assert_eq!(docs_updated, 1);

let docs = coll.find(query!({"name": "Bob"})).await?;
assert_eq!(docs.len(), 1);
assert_eq!(docs[0]["age"], 15.0);
```

Multiply value of a field in the document:

```
let memdb = MemDb::new();
memdb.create_collection("TestCollection").await;
let coll = memdb.collection("TestCollection").await?;
coll.insert(doc!({ "name": "Rob", "age": 25 })).await?;
coll.insert(doc!({ "name": "Bob", "age": 20 })).await?;
coll.insert(doc!({ "name": "Tom", "age": 30 })).await?;

let docs_updated = coll
  .find_and_update(query!({"name": "Bob"}), update!({"$mul": { "age": 5}}))
  .await?;

assert_eq!(docs_updated, 1);

let docs = coll.find(query!({"name": "Bob"})).await?;
assert_eq!(docs.len(), 1);
assert_eq!(docs[0]["age"], 100.0);
```

# Sync API

The sync API are found in `sync_memdb` and `sync_collection` modules.  To use sync API you need to enable it using `sync` feature flag.

```
use memquery::{doc, errors::Error, query, sync_memdb::MemDb};
```

```
let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  let coll = memdb.collection("TestCollection")?;
  coll.insert(
    doc!({ "item": { "name": "ab", "code": 123 }, "qty": 15, "tags": [ "A", "B", "C" ] }),
  )?;
  coll.insert(doc!({ "item": { "name": "cd", "code": 123 }, "qty": 20, "tags": [ "B" ] }))?;
  coll.insert(doc!({ "item": { "name": "ij", "code": 456 }, "qty": 25, "tags": [ "A", "B" ] }))?;
  coll.insert(doc!({ "item": { "name": "xy", "code": 456 }, "qty": 30, "tags": [ "B", "A" ] }))?;
  coll.insert(
    doc!({ "item": { "name": "mn", "code": 000 }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
  )?;

  let docs = coll.find(query!({ "item.code": { "$lte": 123 } }))?;

  assert_eq!(docs.len(), 3);
  assert_eq!(docs[0]["item"]["name"], "ab");
  assert_eq!(docs[1]["item"]["name"], "cd");
  assert_eq!(docs[2]["item"]["name"], "mn");
```


# Building and Running Tests

## Build Rust Library

To build library:

- cargo build

## Testing Rust Library

To test async APIs

- cargo t

To test sync APIs

- cargo t --features "sync"

## Building WASM (wsmemquery.wasm)

Add wasm32-unknown-unknown target

- rustup target add wasm32-unknown-unknown

To build memquery as WebAssembly

- cd wasm
- cargo build --target wasm32-unknown-unknown
