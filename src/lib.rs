//! # MemQuery
//!
//! MemQuery is simple library for creating, querying, and updating in memory documents that are represented as JSON objects and queried using Mongodb like operators.
//!
//! This is not a database and it is not trying to do any optimizations.  It is meant for unit tests or simple projects that require small in memory document store.
//!
//! The library uses async API that support tokio.  The library also has a sync API that may be enabled via `sync` feature flag.
//!
//! # Example Usage
//!
//! ## Create Database
//!
//! ```rust
//! # #[cfg(not(feature = "sync"))]
//! use memquery::{errors::Error, memdb::MemDb};
//!
//! # #[cfg(not(feature = "sync"))]
//! # async fn play() -> Result<(), Error> {
//!     let memdb = MemDb::new();
//! #   Ok(())
//! # }
//! ```
//!
//! ## Create Collection
//!
//! ```rust
//! # #[cfg(not(feature = "sync"))]
//! use memquery::{errors::Error, memdb::MemDb};
//!
//! # #[cfg(not(feature = "sync"))]
//! # async fn play() -> Result<(), Error> {
//!     let memdb = MemDb::new();
//!     memdb.create_collection("TestCollection").await;
//! #   Ok(())
//! # }
//! ```
//!
//! ## Get Collection Handle
//!
//! ```rust
//! # #[cfg(not(feature = "sync"))]
//! use memquery::{errors::Error, memdb::MemDb};
//!
//! # #[cfg(not(feature = "sync"))]
//! # async fn play() -> Result<(), Error> {
//! #   let memdb = MemDb::new();
//! #   memdb.create_collection("TestCollection").await;
//!     let coll = memdb.collection("TestCollection").await?;
//! #   Ok(())
//! # }
//! ```
//!
//! ## Insert Document
//!
//! ```rust
//! # #[cfg(not(feature = "sync"))]
//! use memquery::{errors::Error, memdb::MemDb, doc};
//!
//! # #[cfg(not(feature = "sync"))]
//! # async fn play() -> Result<(), Error> {
//! #   let memdb = MemDb::new();
//! #   memdb.create_collection("TestCollection").await;
//!     let coll = memdb.collection("TestCollection").await?;
//!     coll.insert(doc!({ "name": "Tom", "age": 25 })).await?;
//! #   Ok(())
//! # }
//! ```
//!
//! ## Find Document
//!
//! ```rust
//! # #[cfg(not(feature = "sync"))]
//! use memquery::{errors::Error, memdb::MemDb, doc, query};
//!
//! # #[cfg(not(feature = "sync"))]
//! # async fn play() -> Result<(), Error> {
//! #   let memdb = MemDb::new();
//! #   memdb.create_collection("TestCollection").await;
//!     let coll = memdb.collection("TestCollection").await?;
//!     coll.insert(doc!({ "name": "Tom", "age": 25 })).await?;
//!     let docs = coll.find(query!({"name": "Tom", "age": 25})).await?;
//! #   Ok(())
//! # }
//! ```
//!
//! ## Logical Query Operators
//!
//! ### $and
//!
//! ```rust
//! # #[cfg(not(feature = "sync"))]
//! use memquery::{errors::Error, memdb::MemDb, query};
//!
//! # #[cfg(not(feature = "sync"))]
//! # async fn play() -> Result<(), Error> {
//! #   let memdb = MemDb::new();
//! #   memdb.create_collection("TestCollection").await;
//! #   let coll = memdb.collection("TestCollection").await?;
//!     let docs = coll
//!         .find(query!({ "$and": [{ "name": "Bob" }, { "age": 20 }] }))
//!         .await?;
//! #   Ok(())
//! # }
//! ```
//!
//! ### $or
//!
//! ```rust
//! # #[cfg(not(feature = "sync"))]
//! use memquery::{errors::Error, memdb::MemDb, query};
//!
//! # #[cfg(not(feature = "sync"))]
//! # async fn play() -> Result<(), Error> {
//! #   let memdb = MemDb::new();
//! #   memdb.create_collection("TestCollection").await;
//! #   let coll = memdb.collection("TestCollection").await?;
//!     let docs = coll
//!       .find(query!({ "$or": [{ "name": "Bob" }, { "age": 30 }] }))
//!       .await?;
//! #   Ok(())
//! # }
//! ```
//!
//! ## Comparison Query Operators
//!
//! ### $eq
//!
//! Compare on field:
//!
//! ```rust
//! # #[cfg(not(feature = "sync"))]
//! use memquery::{errors::Error, memdb::MemDb, query};
//!
//! # #[cfg(not(feature = "sync"))]
//! # async fn play() -> Result<(), Error> {
//! #   let memdb = MemDb::new();
//! #   memdb.create_collection("TestCollection").await;
//! #   let coll = memdb.collection("TestCollection").await?;
//!     let docs = coll.find(query!({ "qty": { "$eq": 20 } })).await?;
//! #   Ok(())
//! # }
//! ```
//!
//! Or in embedded document:
//!
//! ```rust
//! # #[cfg(not(feature = "sync"))]
//! use memquery::{errors::Error, memdb::MemDb, query};
//!
//! # #[cfg(not(feature = "sync"))]
//! # async fn play() -> Result<(), Error> {
//! #   let memdb = MemDb::new();
//! #   memdb.create_collection("TestCollection").await;
//! #   let coll = memdb.collection("TestCollection").await?;
//!     let docs = coll.find(query!({ "item.name": { "$eq": "ab" } })).await?;
//! #   Ok(())
//! # }
//! ```
//!
//! You can also compare array with embedded arrays:
//!
//! ```rust
//! # #[cfg(not(feature = "sync"))]
//! use memquery::{errors::Error, memdb::MemDb, doc, query};
//!
//! # #[cfg(not(feature = "sync"))]
//! # async fn play() -> Result<(), Error> {
//! #   let memdb = MemDb::new();
//! #   memdb.create_collection("TestCollection").await;
//! #   let coll = memdb.collection("TestCollection").await?;
//!     coll
//!       .insert(doc!({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] }))
//!       .await?;
//!     coll
//!       .insert(doc!({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] }))
//!       .await?;
//!     coll
//!       .insert(doc!({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] }))
//!       .await?;
//!     coll
//!       .insert(doc!({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] }))
//!       .await?;
//!     coll
//!       .insert(
//!         doc!({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
//!       )
//!       .await?;
//!
//!     let docs = coll
//!       .find(query!({ "tags": { "$eq": [ "A", "B" ] } }))
//!       .await?;
//!
//!     assert_eq!(docs.len(), 2);
//!     assert_eq!(docs[0]["item"]["name"], "ij");
//!     assert_eq!(docs[1]["item"]["name"], "mn");
//! #   Ok(())
//! # }
//! ```
//!
//! Or value in the embedded array:
//!
//! ```rust
//! # #[cfg(not(feature = "sync"))]
//! use memquery::{errors::Error, memdb::MemDb, doc, query};
//!
//! # #[cfg(not(feature = "sync"))]
//! # async fn play() -> Result<(), Error> {
//! #   let memdb = MemDb::new();
//! #   memdb.create_collection("TestCollection").await;
//! #   let coll = memdb.collection("TestCollection").await?;
//!     coll
//!       .insert(doc!({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] }))
//!       .await?;
//!     coll
//!       .insert(doc!({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] }))
//!       .await?;
//!     coll
//!       .insert(doc!({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] }))
//!       .await?;
//!     coll
//!       .insert(doc!({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] }))
//!       .await?;
//!     coll
//!       .insert(
//!         doc!({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
//!       )
//!       .await?;
//!
//!     let docs = coll.find(query!({ "tags": { "$eq": "B" } })).await?;
//!
//!     assert_eq!(docs.len(), 4);
//!     assert_eq!(docs[0]["item"]["name"], "ab");
//!     assert_eq!(docs[1]["item"]["name"], "cd");
//!     assert_eq!(docs[2]["item"]["name"], "ij");
//!     assert_eq!(docs[3]["item"]["name"], "xy");
//! #   Ok(())
//! # }
//! ```
//!
//! ### $gt
//!
//! ```rust
//! # #[cfg(not(feature = "sync"))]
//! use memquery::{errors::Error, memdb::MemDb, query};
//!
//! # #[cfg(not(feature = "sync"))]
//! # async fn play() -> Result<(), Error> {
//! #   let memdb = MemDb::new();
//! #   memdb.create_collection("TestCollection").await;
//! #   let coll = memdb.collection("TestCollection").await?;
//!     let docs = coll.find(query!({ "qty": { "$gt": 20 } })).await?;
//! #   Ok(())
//! # }
//! ```
//!
//! ### $gte
//!
//! ```rust
//! # #[cfg(not(feature = "sync"))]
//! use memquery::{errors::Error, memdb::MemDb, query};
//!
//! # #[cfg(not(feature = "sync"))]
//! # async fn play() -> Result<(), Error> {
//! #   let memdb = MemDb::new();
//! #   memdb.create_collection("TestCollection").await;
//! #   let coll = memdb.collection("TestCollection").await?;
//!     let docs = coll.find(query!({ "qty": { "$gte": 20 } })).await?;
//! #   Ok(())
//! # }
//! ```
//!
//! ### $lt
//!
//! ```rust
//! # #[cfg(not(feature = "sync"))]
//! use memquery::{errors::Error, memdb::MemDb, query};
//!
//! # #[cfg(not(feature = "sync"))]
//! # async fn play() -> Result<(), Error> {
//! #   let memdb = MemDb::new();
//! #   memdb.create_collection("TestCollection").await;
//! #   let coll = memdb.collection("TestCollection").await?;
//!     let docs = coll.find(query!({ "qty": { "$lt": 20 } })).await?;
//! #   Ok(())
//! # }
//! ```
//!
//! ### $lte
//!
//! ```rust
//! # #[cfg(not(feature = "sync"))]
//! use memquery::{errors::Error, memdb::MemDb, query};
//!
//! # #[cfg(not(feature = "sync"))]
//! # async fn play() -> Result<(), Error> {
//! #   let memdb = MemDb::new();
//! #   memdb.create_collection("TestCollection").await;
//! #   let coll = memdb.collection("TestCollection").await?;
//!     let docs = coll.find(query!({ "qty": { "$lte": 20 } })).await?;
//! #   Ok(())
//! # }
//! ```
//!
//! ### Find All Documents
//!
//! ```rust
//! # #[cfg(not(feature = "sync"))]
//! use memquery::{errors::Error, memdb::MemDb, query};
//!
//! # #[cfg(not(feature = "sync"))]
//! # async fn play() -> Result<(), Error> {
//! #   let memdb = MemDb::new();
//! #   memdb.create_collection("TestCollection").await;
//! #   let coll = memdb.collection("TestCollection").await?;
//!     let docs = coll.find(query!({})).await?;
//! #   Ok(())
//! # }
//! ```
//!
//! ## Update Document
//!
//! This shows examples how to use `find_and_update` API.
//!
//! Update document by replacing entire document:
//!
//! ```rust
//! # #[cfg(not(feature = "sync"))]
//! use memquery::{errors::Error, memdb::MemDb, doc, query, update};
//!
//! # #[cfg(not(feature = "sync"))]
//! # async fn play() -> Result<(), Error> {
//! #   let memdb = MemDb::new();
//! #   memdb.create_collection("TestCollection").await;
//! #   let coll = memdb.collection("TestCollection").await?;
//!     coll.insert(doc!({ "name": "Rob", "age": 25 })).await?;
//!     coll.insert(doc!({ "name": "Bob", "age": 20 })).await?;
//!     coll.insert(doc!({ "name": "Tom", "age": 30 })).await?;
//!
//!     let docs_updated = coll
//!       .find_and_update(
//!         query!({"name": "Bob"}),
//!         update!({"nickname": "Bobcat", "voice": "meow"}),
//!       )
//!     .await?;
//!
//!     assert_eq!(docs_updated, 1);
//!
//!     let docs = coll.find(query!({"nickname": "Bobcat"})).await?;
//!     assert_eq!(docs.len(), 1);
//!     assert_eq!(docs[0]["voice"], "meow");
//! #   Ok(())
//! # }
//! ```
//!
//! Update specific field(s) in the document:
//!
//! ```rust
//! # #[cfg(not(feature = "sync"))]
//! use memquery::{errors::Error, memdb::MemDb, doc, query, update};
//!
//! # #[cfg(not(feature = "sync"))]
//! # async fn play() -> Result<(), Error> {
//! #   let memdb = MemDb::new();
//! #   memdb.create_collection("TestCollection").await;
//! #   let coll = memdb.collection("TestCollection").await?;
//!     coll.insert(doc!({ "name": "Rob", "age": 25 })).await?;
//!     coll.insert(doc!({ "name": "Bob", "age": 20 })).await?;
//!     coll.insert(doc!({ "name": "Tom", "age": 30 })).await?;
//!
//!     let docs_updated = coll
//!       .find_and_update(
//!         query!({"name": "Bob"}),
//!         update!({"$set": { "name": "Roy", "age": 21, "email": "test@test.com"}}),
//!       )
//!       .await?;
//!
//!     assert_eq!(docs_updated, 1);
//!
//!     let docs = coll.find(query!({"name": "Roy"})).await?;
//!     assert_eq!(docs.len(), 1);
//!     assert_eq!(docs[0]["age"], 21);
//!     assert_eq!(docs[0]["email"], "test@test.com");
//! #   Ok(())
//! # }
//! ```
//!
//! Update document to remove field:
//!
//! ```rust
//! # #[cfg(not(feature = "sync"))]
//! use memquery::{errors::Error, memdb::MemDb, doc, query, update};
//!
//! # #[cfg(not(feature = "sync"))]
//! # async fn play() -> Result<(), Error> {
//! #   let memdb = MemDb::new();
//! #   memdb.create_collection("TestCollection").await;
//! #   let coll = memdb.collection("TestCollection").await?;
//!     coll.insert(doc!({ "name": "Rob", "age": 25 })).await?;
//!     coll.insert(doc!({ "name": "Bob", "age": 20 })).await?;
//!     coll.insert(doc!({ "name": "Tom", "age": 30 })).await?;
//!
//!     let docs_updated = coll
//!       .find_and_update(
//!         query!({ "name": "Bob" }),
//!         update!({ "$set": { "name": "Roy", "age": 21, "email": "test@test.com" }}),
//!       )
//!       .await?;
//!
//!     assert_eq!(docs_updated, 1);
//!
//!     let docs = coll.find(query!({"name": "Roy"})).await?;
//!     assert_eq!(docs.len(), 1);
//!     assert_eq!(docs[0]["age"], 21);
//!     assert_eq!(docs[0]["email"], "test@test.com");
//!
//!     let docs_updated2 = coll
//!       .find_and_update(
//!         query!({ "name": "Roy" }),
//!         update!({ "$unset": { "email": "" }}),
//!       )
//!     .await?;
//!
//!     assert_eq!(docs_updated2, 1);
//!
//!     let docs = coll.find(query!({"name": "Roy"})).await?;
//!     assert_eq!(docs.len(), 1);
//!     assert_eq!(docs[0]["age"], 21);
//!     assert_eq!(docs[0]["email"], serde_json::Value::Null);
//! #   Ok(())
//! # }
//! ```
//!
//! Increment value of the field in the document:
//!
//! ```rust
//! # #[cfg(not(feature = "sync"))]
//! use memquery::{errors::Error, memdb::MemDb, doc, query, update};
//!
//! # #[cfg(not(feature = "sync"))]
//! # async fn play() -> Result<(), Error> {
//! #   let memdb = MemDb::new();
//! #   memdb.create_collection("TestCollection").await;
//! #   let coll = memdb.collection("TestCollection").await?;
//!     coll.insert(doc!({ "name": "Rob", "age": 25 })).await?;
//!     coll.insert(doc!({ "name": "Bob", "age": 20 })).await?;
//!     coll.insert(doc!({ "name": "Tom", "age": 30 })).await?;
//!
//!     let docs_updated = coll
//!       .find_and_update(query!({"name": "Bob"}), update!({"$inc": { "age": -5 }}))
//!       .await?;
//!
//!     assert_eq!(docs_updated, 1);
//!
//!     let docs = coll.find(query!({"name": "Bob"})).await?;
//!     assert_eq!(docs.len(), 1);
//!     assert_eq!(docs[0]["age"], 15.0);
//! #   Ok(())
//! # }
//! ```
//!
//! Multiply value of a field in the document:
//!
//! ```rust
//! # #[cfg(not(feature = "sync"))]
//! use memquery::{errors::Error, memdb::MemDb, doc, query, update};
//!
//! # #[cfg(not(feature = "sync"))]
//! # async fn play() -> Result<(), Error> {
//! #   let memdb = MemDb::new();
//! #   memdb.create_collection("TestCollection").await;
//! #   let coll = memdb.collection("TestCollection").await?;
//!     coll.insert(doc!({ "name": "Rob", "age": 25 })).await?;
//!     coll.insert(doc!({ "name": "Bob", "age": 20 })).await?;
//!     coll.insert(doc!({ "name": "Tom", "age": 30 })).await?;
//!
//!     let docs_updated = coll
//!       .find_and_update(query!({"name": "Bob"}), update!({"$mul": { "age": 5}}))
//!       .await?;
//!
//!     assert_eq!(docs_updated, 1);
//!
//!     let docs = coll.find(query!({"name": "Bob"})).await?;
//!     assert_eq!(docs.len(), 1);
//!     assert_eq!(docs[0]["age"], 100.0);
//! #   Ok(())
//! # }
//! ```
//!
//! # Delete Documents
//!
//! ```rust
//! # #[cfg(not(feature = "sync"))]
//! use memquery::{errors::Error, memdb::MemDb, doc, query};
//!
//! # #[cfg(not(feature = "sync"))]
//! # async fn play() -> Result<(), Error> {
//! #   let memdb = MemDb::new();
//! #   memdb.create_collection("TestCollection").await;
//! #   let coll = memdb.collection("TestCollection").await?;
//!     coll.insert(doc!({ "name": "Rob", "age": 25 })).await?;
//!     coll.insert(doc!({ "name": "Bob", "age": 20 })).await?;
//!     coll.insert(doc!({ "name": "Tom", "age": 30 })).await?;
//!
//!     let docs = coll.find_and_delete(query!({})).await?;
//!     assert_eq!(docs.len(), 3);
//!
//!     let docs_remaining = coll.find(query!({})).await?;
//!     assert_eq!(docs_remaining.len(), 0);
//! #   Ok(())
//! # }
//! ```
//!
//! # Sync API
//!
//! To use sync API you need to enable it using `sync` feature flag.
//!
//! ```
//! # #[cfg(feature = "sync")]
//! use memquery::{doc, errors::Error, query, memdb::MemDb};
//!
//! # #[cfg(feature = "sync")]
//! # fn play() -> Result<(), Error> {
//! #   let memdb = MemDb::new();
//! #   memdb.create_collection("TestCollection");
//! #   let coll = memdb.collection("TestCollection")?;
//!     coll.insert(
//!       doc!({ "item": { "name": "ab", "code": 123 }, "qty": 15, "tags": [ "A", "B", "C" ] }),
//!     )?;
//!     coll.insert(doc!({ "item": { "name": "cd", "code": 123 }, "qty": 20, "tags": [ "B" ] }))?;
//!     coll.insert(doc!({ "item": { "name": "ij", "code": 456 }, "qty": 25, "tags": [ "A", "B" ] }))?;
//!     coll.insert(doc!({ "item": { "name": "xy", "code": 456 }, "qty": 30, "tags": [ "B", "A" ] }))?;
//!     coll.insert(
//!       doc!({ "item": { "name": "mn", "code": 000 }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
//!     )?;
//!
//!     let docs = coll.find(query!({ "item.code": { "$lte": 123 } }))?;
//!
//!     assert_eq!(docs.len(), 3);
//!     assert_eq!(docs[0]["item"]["name"], "ab");
//!     assert_eq!(docs[1]["item"]["name"], "cd");
//!     assert_eq!(docs[2]["item"]["name"], "mn");
//! #   Ok(())
//! # }
//! ```
pub mod collection;
mod engine;
pub mod errors;
pub mod macros;
pub mod memdb;
mod utils;

pub use engine::{DocumentCollection, Documents};
