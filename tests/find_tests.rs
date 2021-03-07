use memquery::{doc, errors::Error, memdb::MemDb, query};

#[tokio::test]
async fn test_simple_query() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
  coll.insert(doc!({ "name": "Rob", "age": 25 })).await?;
  coll.insert(doc!({ "name": "Bob", "age": 20 })).await?;
  coll.insert(doc!({ "name": "Tom", "age": 30 })).await?;

  let docs = coll.find(query!({"name": "Bob"})).await?;

  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["name"], "Bob");
  Ok(())
}

#[tokio::test]
async fn test_simple_query_with_multiple_conditions() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
  coll.insert(doc!({ "name": "Rob", "age": 25 })).await?;
  coll.insert(doc!({ "name": "Bob", "age": 20 })).await?;
  coll.insert(doc!({ "name": "Tom", "age": 30 })).await?;
  coll.insert(doc!({ "name": "Victor", "age": 20 })).await?;

  let docs = coll.find(query!({"name": "Bob", "age": 20})).await?;

  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["name"], "Bob");
  Ok(())
}

#[tokio::test]
async fn test_nomatch_query_with_multiple_conditions() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
  coll.insert(doc!({ "name": "Rob", "age": 25 })).await?;
  coll.insert(doc!({ "name": "Bob", "age": 20 })).await?;
  coll.insert(doc!({ "name": "Tom", "age": 30 })).await?;

  let docs = coll.find(query!({"name": "Bob", "age": 21})).await?;

  assert_eq!(docs.len(), 0);
  Ok(())
}

#[tokio::test]
async fn test_query_match_with_and() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
  coll.insert(doc!({ "name": "Rob", "age": 25 })).await?;
  coll.insert(doc!({ "name": "Bob", "age": 20 })).await?;
  coll.insert(doc!({ "name": "Tom", "age": 30 })).await?;

  let docs = coll
    .find(query!({ "$and": [{ "name": "Bob" }, { "age": 20 }] }))
    .await?;

  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["name"], "Bob");
  Ok(())
}

#[tokio::test]
async fn test_query_nomatch_with_and() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
  coll.insert(doc!({ "name": "Rob", "age": 25 })).await?;
  coll.insert(doc!({ "name": "Bob", "age": 20 })).await?;
  coll.insert(doc!({ "name": "Tom", "age": 30 })).await?;

  let docs = coll
    .find(query!({ "$and": [{ "name": "Bob" }, { "age": 21 }] }))
    .await?;

  assert_eq!(docs.len(), 0);
  Ok(())
}

#[tokio::test]
async fn test_query_match_with_or() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
  coll.insert(doc!({ "name": "Rob", "age": 25 })).await?;
  coll.insert(doc!({ "name": "Bob", "age": 20 })).await?;
  coll.insert(doc!({ "name": "Tom", "age": 30 })).await?;

  let docs = coll
    .find(query!({ "$or": [{ "name": "Bob" }, { "age": 30 }] }))
    .await?;

  assert_eq!(docs.len(), 2);
  Ok(())
}

#[tokio::test]
async fn test_query_nomatch_with_or() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
  coll.insert(doc!({ "name": "Rob", "age": 25 })).await?;
  coll.insert(doc!({ "name": "Bob", "age": 20 })).await?;
  coll.insert(doc!({ "name": "Tom", "age": 30 })).await?;

  let docs = coll
    .find(query!({ "$or": [{ "name": "Toby" }, { "age": 40 }] }))
    .await?;

  assert_eq!(docs.len(), 0);
  Ok(())
}

#[tokio::test]
async fn test_eq_op() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
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

  let docs = coll.find(query!({ "qty": { "$eq": 20 } })).await?;

  assert_eq!(docs.len(), 2);
  assert_eq!(docs[0]["item"]["name"], "cd");
  assert_eq!(docs[1]["item"]["name"], "mn");

  Ok(())
}

#[tokio::test]
async fn test_eq_nomatch_op() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
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

  let docs = coll.find(query!({ "qty": { "$eq": 200 } })).await?;

  assert_eq!(docs.len(), 0);

  Ok(())
}

#[tokio::test]
async fn test_eq_op_single_entry_embedded_doc() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
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

  let docs = coll.find(query!({ "item.name": { "$eq": "ab" } })).await?;

  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["item"]["name"], "ab");

  Ok(())
}

#[tokio::test]
async fn test_eq_op_to_match_array_to_array() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
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
    .find(query!({ "tags": { "$eq": [ "A", "B"  ] } }))
    .await?;

  assert_eq!(docs.len(), 2);
  assert_eq!(docs[0]["item"]["name"], "ij");
  assert_eq!(docs[1]["item"]["name"], "mn");

  Ok(())
}

#[tokio::test]
async fn test_eq_op_to_nomatch_array_to_array() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
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
    .find(query!({ "tags": { "$eq": [ "C", "D"  ] } }))
    .await?;

  assert_eq!(docs.len(), 0);

  Ok(())
}

#[tokio::test]
async fn test_eq_op_to_match_array_to_value() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
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

  Ok(())
}

#[tokio::test]
async fn test_gt_match() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
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

  let docs = coll.find(query!({ "qty": { "$gt": 20 } })).await?;

  assert_eq!(docs.len(), 2);
  assert_eq!(docs[0]["item"]["name"], "ij");
  assert_eq!(docs[1]["item"]["name"], "xy");

  Ok(())
}

#[tokio::test]
async fn test_gt_no_match() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
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

  let docs = coll.find(query!({ "qty": { "$gt": 200 } })).await?;

  assert_eq!(docs.len(), 0);

  Ok(())
}

#[tokio::test]
async fn test_gt_match_embedded_doc() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
  coll
    .insert(doc!({ "item": { "name": "ab", "code": 123 }, "qty": 15, "tags": [ "A", "B", "C" ] }))
    .await?;
  coll
    .insert(doc!({ "item": { "name": "cd", "code": 123 }, "qty": 20, "tags": [ "B" ] }))
    .await?;
  coll
    .insert(doc!({ "item": { "name": "ij", "code": 456 }, "qty": 25, "tags": [ "A", "B" ] }))
    .await?;
  coll
    .insert(doc!({ "item": { "name": "xy", "code": 456 }, "qty": 30, "tags": [ "B", "A" ] }))
    .await?;
  coll
    .insert(
      doc!({ "item": { "name": "mn", "code": 000 }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
    )
    .await?;

  let docs = coll.find(query!({ "item.code": { "$gt": 400 } })).await?;

  assert_eq!(docs.len(), 2);
  assert_eq!(docs[0]["item"]["name"], "ij");
  assert_eq!(docs[1]["item"]["name"], "xy");

  Ok(())
}

#[tokio::test]
async fn test_gte_match() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
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

  let docs = coll.find(query!({ "qty": { "$gte": 20 } })).await?;

  assert_eq!(docs.len(), 4);
  assert_eq!(docs[0]["item"]["name"], "cd");
  assert_eq!(docs[1]["item"]["name"], "ij");
  assert_eq!(docs[2]["item"]["name"], "xy");
  assert_eq!(docs[3]["item"]["name"], "mn");

  Ok(())
}

#[tokio::test]
async fn test_gte_no_match() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
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

  let docs = coll.find(query!({ "qty": { "$gte": 200 } })).await?;

  assert_eq!(docs.len(), 0);

  Ok(())
}

#[tokio::test]
async fn test_gte_match_embedded_doc() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
  coll
    .insert(doc!({ "item": { "name": "ab", "code": 123 }, "qty": 15, "tags": [ "A", "B", "C" ] }))
    .await?;
  coll
    .insert(doc!({ "item": { "name": "cd", "code": 123 }, "qty": 20, "tags": [ "B" ] }))
    .await?;
  coll
    .insert(doc!({ "item": { "name": "ij", "code": 456 }, "qty": 25, "tags": [ "A", "B" ] }))
    .await?;
  coll
    .insert(doc!({ "item": { "name": "xy", "code": 456 }, "qty": 30, "tags": [ "B", "A" ] }))
    .await?;
  coll
    .insert(
      doc!({ "item": { "name": "mn", "code": 000 }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
    )
    .await?;

  let docs = coll.find(query!({ "item.code": { "$gte": 456 } })).await?;

  assert_eq!(docs.len(), 2);
  assert_eq!(docs[0]["item"]["name"], "ij");
  assert_eq!(docs[1]["item"]["name"], "xy");

  Ok(())
}

#[tokio::test]
async fn test_lt_match() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
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

  let docs = coll.find(query!({ "qty": { "$lt": 20 } })).await?;

  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["item"]["name"], "ab");

  Ok(())
}

#[tokio::test]
async fn test_lt_no_match() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
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

  let docs = coll.find(query!({ "qty": { "$lt": 2 } })).await?;

  assert_eq!(docs.len(), 0);

  Ok(())
}

#[tokio::test]
async fn test_lt_match_embedded_doc() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
  coll
    .insert(doc!({ "item": { "name": "ab", "code": 123 }, "qty": 15, "tags": [ "A", "B", "C" ] }))
    .await?;
  coll
    .insert(doc!({ "item": { "name": "cd", "code": 123 }, "qty": 20, "tags": [ "B" ] }))
    .await?;
  coll
    .insert(doc!({ "item": { "name": "ij", "code": 456 }, "qty": 25, "tags": [ "A", "B" ] }))
    .await?;
  coll
    .insert(doc!({ "item": { "name": "xy", "code": 456 }, "qty": 30, "tags": [ "B", "A" ] }))
    .await?;
  coll
    .insert(
      doc!({ "item": { "name": "mn", "code": 000 }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
    )
    .await?;

  let docs = coll.find(query!({ "item.code": { "$lt": 400 } })).await?;

  assert_eq!(docs.len(), 3);
  assert_eq!(docs[0]["item"]["name"], "ab");
  assert_eq!(docs[1]["item"]["name"], "cd");
  assert_eq!(docs[2]["item"]["name"], "mn");

  Ok(())
}

#[tokio::test]
async fn test_lte_match() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
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

  let docs = coll.find(query!({ "qty": { "$lte": 20 } })).await?;

  assert_eq!(docs.len(), 3);
  assert_eq!(docs[0]["item"]["name"], "ab");
  assert_eq!(docs[1]["item"]["name"], "cd");
  assert_eq!(docs[2]["item"]["name"], "mn");

  Ok(())
}

#[tokio::test]
async fn test_lte_no_match() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
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

  let docs = coll.find(query!({ "qty": { "$lte": 2 } })).await?;

  assert_eq!(docs.len(), 0);

  Ok(())
}

#[tokio::test]
async fn test_lte_match_embedded_doc() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
  coll
    .insert(doc!({ "item": { "name": "ab", "code": 123 }, "qty": 15, "tags": [ "A", "B", "C" ] }))
    .await?;
  coll
    .insert(doc!({ "item": { "name": "cd", "code": 123 }, "qty": 20, "tags": [ "B" ] }))
    .await?;
  coll
    .insert(doc!({ "item": { "name": "ij", "code": 456 }, "qty": 25, "tags": [ "A", "B" ] }))
    .await?;
  coll
    .insert(doc!({ "item": { "name": "xy", "code": 456 }, "qty": 30, "tags": [ "B", "A" ] }))
    .await?;
  coll
    .insert(
      doc!({ "item": { "name": "mn", "code": 000 }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
    )
    .await?;

  let docs = coll.find(query!({ "item.code": { "$lte": 123 } })).await?;

  assert_eq!(docs.len(), 3);
  assert_eq!(docs[0]["item"]["name"], "ab");
  assert_eq!(docs[1]["item"]["name"], "cd");
  assert_eq!(docs[2]["item"]["name"], "mn");

  Ok(())
}
