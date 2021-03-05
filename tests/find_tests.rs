use memquery::{doc, errors::Error, memdb::MemDb, query};

#[test]
fn test_simple_query() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  memdb.insert("TestCollection", doc!({ "name": "Rob", "age": 25 }))?;
  memdb.insert("TestCollection", doc!({ "name": "Bob", "age": 20 }))?;
  memdb.insert("TestCollection", doc!({ "name": "Tom", "age": 30 }))?;

  let docs = memdb.find("TestCollection", query!({"name": "Bob"}))?;

  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["name"], "Bob");
  Ok(())
}

#[test]
fn test_simple_query_with_multiple_conditions() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  memdb.insert("TestCollection", doc!({ "name": "Rob", "age": 25 }))?;
  memdb.insert("TestCollection", doc!({ "name": "Bob", "age": 20 }))?;
  memdb.insert("TestCollection", doc!({ "name": "Tom", "age": 30 }))?;
  memdb.insert("TestCollection", doc!({ "name": "Victor", "age": 20 }))?;

  let docs = memdb.find("TestCollection", query!({"name": "Bob", "age": 20}))?;

  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["name"], "Bob");
  Ok(())
}

#[test]
fn test_nomatch_query_with_multiple_conditions() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  memdb.insert("TestCollection", doc!({ "name": "Rob", "age": 25 }))?;
  memdb.insert("TestCollection", doc!({ "name": "Bob", "age": 20 }))?;
  memdb.insert("TestCollection", doc!({ "name": "Tom", "age": 30 }))?;

  let docs = memdb.find("TestCollection", query!({"name": "Bob", "age": 21}))?;

  assert_eq!(docs.len(), 0);
  Ok(())
}

#[test]
fn test_query_match_with_and() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  memdb.insert("TestCollection", doc!({ "name": "Rob", "age": 25 }))?;
  memdb.insert("TestCollection", doc!({ "name": "Bob", "age": 20 }))?;
  memdb.insert("TestCollection", doc!({ "name": "Tom", "age": 30 }))?;

  let docs = memdb.find(
    "TestCollection",
    query!({ "$and": [{ "name": "Bob" }, { "age": 20 }] }),
  )?;

  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["name"], "Bob");
  Ok(())
}

#[test]
fn test_query_nomatch_with_and() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  memdb.insert("TestCollection", doc!({ "name": "Rob", "age": 25 }))?;
  memdb.insert("TestCollection", doc!({ "name": "Bob", "age": 20 }))?;
  memdb.insert("TestCollection", doc!({ "name": "Tom", "age": 30 }))?;

  let docs = memdb.find(
    "TestCollection",
    query!({ "$and": [{ "name": "Bob" }, { "age": 21 }] }),
  )?;

  assert_eq!(docs.len(), 0);
  Ok(())
}

#[test]
fn test_query_match_with_or() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  memdb.insert("TestCollection", doc!({ "name": "Rob", "age": 25 }))?;
  memdb.insert("TestCollection", doc!({ "name": "Bob", "age": 20 }))?;
  memdb.insert("TestCollection", doc!({ "name": "Tom", "age": 30 }))?;

  let docs = memdb.find(
    "TestCollection",
    query!({ "$or": [{ "name": "Bob" }, { "age": 30 }] }),
  )?;

  assert_eq!(docs.len(), 2);
  Ok(())
}

#[test]
fn test_query_nomatch_with_or() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  memdb.insert("TestCollection", doc!({ "name": "Rob", "age": 25 }))?;
  memdb.insert("TestCollection", doc!({ "name": "Bob", "age": 20 }))?;
  memdb.insert("TestCollection", doc!({ "name": "Tom", "age": 30 }))?;

  let docs = memdb.find(
    "TestCollection",
    query!({ "$or": [{ "name": "Toby" }, { "age": 40 }] }),
  )?;

  assert_eq!(docs.len(), 0);
  Ok(())
}

#[test]
fn test_eq_op() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
  )?;

  let docs = memdb.find("TestCollection", query!({ "qty": { "$eq": 20 } }))?;

  assert_eq!(docs.len(), 2);
  assert_eq!(docs[0]["item"]["name"], "cd");
  assert_eq!(docs[1]["item"]["name"], "mn");

  Ok(())
}

#[test]
fn test_eq_nomatch_op() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
  )?;

  let docs = memdb.find("TestCollection", query!({ "qty": { "$eq": 200 } }))?;

  assert_eq!(docs.len(), 0);

  Ok(())
}

#[test]
fn test_eq_op_single_entry_embedded_doc() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
  )?;

  let docs = memdb.find("TestCollection", query!({ "item.name": { "$eq": "ab" } }))?;

  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["item"]["name"], "ab");

  Ok(())
}

#[test]
fn test_eq_op_to_match_array_to_array() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
  )?;

  let docs = memdb.find(
    "TestCollection",
    query!({ "tags": { "$eq": [ "A", "B"  ] } }),
  )?;

  assert_eq!(docs.len(), 2);
  assert_eq!(docs[0]["item"]["name"], "ij");
  assert_eq!(docs[1]["item"]["name"], "mn");

  Ok(())
}

#[test]
fn test_eq_op_to_nomatch_array_to_array() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
  )?;

  let docs = memdb.find(
    "TestCollection",
    query!({ "tags": { "$eq": [ "C", "D"  ] } }),
  )?;

  assert_eq!(docs.len(), 0);

  Ok(())
}

#[test]
fn test_eq_op_to_match_array_to_value() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
  )?;

  let docs = memdb.find("TestCollection", query!({ "tags": { "$eq": "B" } }))?;

  assert_eq!(docs.len(), 4);
  assert_eq!(docs[0]["item"]["name"], "ab");
  assert_eq!(docs[1]["item"]["name"], "cd");
  assert_eq!(docs[2]["item"]["name"], "ij");
  assert_eq!(docs[3]["item"]["name"], "xy");

  Ok(())
}

#[test]
fn test_gt_match() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
  )?;

  let docs = memdb.find("TestCollection", query!({ "qty": { "$gt": 20 } }))?;

  assert_eq!(docs.len(), 2);
  assert_eq!(docs[0]["item"]["name"], "ij");
  assert_eq!(docs[1]["item"]["name"], "xy");

  Ok(())
}

#[test]
fn test_gt_no_match() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
  )?;

  let docs = memdb.find("TestCollection", query!({ "qty": { "$gt": 200 } }))?;

  assert_eq!(docs.len(), 0);

  Ok(())
}

#[test]
fn test_gt_match_embedded_doc() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ab", "code": 123 }, "qty": 15, "tags": [ "A", "B", "C" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "cd", "code": 123 }, "qty": 20, "tags": [ "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ij", "code": 456 }, "qty": 25, "tags": [ "A", "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "xy", "code": 456 }, "qty": 30, "tags": [ "B", "A" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "mn", "code": 000 }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
  )?;

  let docs = memdb.find("TestCollection", query!({ "item.code": { "$gt": 400 } }))?;

  assert_eq!(docs.len(), 2);
  assert_eq!(docs[0]["item"]["name"], "ij");
  assert_eq!(docs[1]["item"]["name"], "xy");

  Ok(())
}

#[test]
fn test_gte_match() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
  )?;

  let docs = memdb.find("TestCollection", query!({ "qty": { "$gte": 20 } }))?;

  assert_eq!(docs.len(), 4);
  assert_eq!(docs[0]["item"]["name"], "cd");
  assert_eq!(docs[1]["item"]["name"], "ij");
  assert_eq!(docs[2]["item"]["name"], "xy");
  assert_eq!(docs[3]["item"]["name"], "mn");

  Ok(())
}

#[test]
fn test_gte_no_match() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
  )?;

  let docs = memdb.find("TestCollection", query!({ "qty": { "$gte": 200 } }))?;

  assert_eq!(docs.len(), 0);

  Ok(())
}

#[test]
fn test_gte_match_embedded_doc() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ab", "code": 123 }, "qty": 15, "tags": [ "A", "B", "C" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "cd", "code": 123 }, "qty": 20, "tags": [ "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ij", "code": 456 }, "qty": 25, "tags": [ "A", "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "xy", "code": 456 }, "qty": 30, "tags": [ "B", "A" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "mn", "code": 000 }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
  )?;

  let docs = memdb.find("TestCollection", query!({ "item.code": { "$gte": 456 } }))?;

  assert_eq!(docs.len(), 2);
  assert_eq!(docs[0]["item"]["name"], "ij");
  assert_eq!(docs[1]["item"]["name"], "xy");

  Ok(())
}

#[test]
fn test_lt_match() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
  )?;

  let docs = memdb.find("TestCollection", query!({ "qty": { "$lt": 20 } }))?;

  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["item"]["name"], "ab");

  Ok(())
}

#[test]
fn test_lt_no_match() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
  )?;

  let docs = memdb.find("TestCollection", query!({ "qty": { "$lt": 2 } }))?;

  assert_eq!(docs.len(), 0);

  Ok(())
}

#[test]
fn test_lt_match_embedded_doc() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ab", "code": 123 }, "qty": 15, "tags": [ "A", "B", "C" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "cd", "code": 123 }, "qty": 20, "tags": [ "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ij", "code": 456 }, "qty": 25, "tags": [ "A", "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "xy", "code": 456 }, "qty": 30, "tags": [ "B", "A" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "mn", "code": 000 }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
  )?;

  let docs = memdb.find("TestCollection", query!({ "item.code": { "$lt": 400 } }))?;

  assert_eq!(docs.len(), 3);
  assert_eq!(docs[0]["item"]["name"], "ab");
  assert_eq!(docs[1]["item"]["name"], "cd");
  assert_eq!(docs[2]["item"]["name"], "mn");

  Ok(())
}

#[test]
fn test_lte_match() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
  )?;

  let docs = memdb.find("TestCollection", query!({ "qty": { "$lte": 20 } }))?;

  assert_eq!(docs.len(), 3);
  assert_eq!(docs[0]["item"]["name"], "ab");
  assert_eq!(docs[1]["item"]["name"], "cd");
  assert_eq!(docs[2]["item"]["name"], "mn");

  Ok(())
}

#[test]
fn test_lte_no_match() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
  )?;

  let docs = memdb.find("TestCollection", query!({ "qty": { "$lte": 2 } }))?;

  assert_eq!(docs.len(), 0);

  Ok(())
}

#[test]
fn test_lte_match_embedded_doc() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ab", "code": 123 }, "qty": 15, "tags": [ "A", "B", "C" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "cd", "code": 123 }, "qty": 20, "tags": [ "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "ij", "code": 456 }, "qty": 25, "tags": [ "A", "B" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "xy", "code": 456 }, "qty": 30, "tags": [ "B", "A" ] }),
  )?;
  memdb.insert(
    "TestCollection",
    doc!({ "item": { "name": "mn", "code": 000 }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
  )?;

  let docs = memdb.find("TestCollection", query!({ "item.code": { "$lte": 123 } }))?;

  assert_eq!(docs.len(), 3);
  assert_eq!(docs[0]["item"]["name"], "ab");
  assert_eq!(docs[1]["item"]["name"], "cd");
  assert_eq!(docs[2]["item"]["name"], "mn");

  Ok(())
}
