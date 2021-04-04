from unittest import TestCase
from memquery import Collection, create_collection,\
    collection


class TestFindAPI(TestCase):
    def test_create_collection(self):
        create_collection('TestCollection')
        test_coll = None
        try:
            test_coll = collection('TestCollection')
        except Exception as e:
            pass

        self.assertTrue(test_coll is not None)

    def test_create_collection_not_found(self):
        create_collection('TestCollection')
        test_coll = None
        try:
            _ = collection('TestCollection1')
        except Exception as e:
            pass

        self.assertTrue(test_coll is None)

    def test_simple_query(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "name": "Rob", "age": 25 })
        coll.insert({ "name": "Bob", "age": 20 })
        coll.insert({ "name": "Tom", "age": 30 })

        docs = coll.find({"name": "Bob"})

        self.assertTrue(len(docs), 1);
        self.assertTrue(docs[0]["name"] == "Bob");

    def test_simple_query_with_multiple_conditions(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "name": "Rob", "age": 25 })
        coll.insert({ "name": "Bob", "age": 20 })
        coll.insert({ "name": "Tom", "age": 30 })
        coll.insert({ "name": "Victor", "age": 20 })

        docs = coll.find({"name": "Bob", "age": 20})

        self.assertTrue(len(docs), 1)
        self.assertTrue(docs[0]["name"] == "Bob")

    def test_nomatch_query_with_multiple_conditions(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "name": "Rob", "age": 25 })
        coll.insert({ "name": "Bob", "age": 20 })
        coll.insert({ "name": "Tom", "age": 30 })

        docs = coll.find({"name": "Bob", "age": 21})

        self.assertTrue(len(docs) == 0)
    
    def test_query_match_with_and(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "name": "Rob", "age": 25 })
        coll.insert({ "name": "Bob", "age": 20 })
        coll.insert({ "name": "Tom", "age": 30 })

        docs = coll.find({ "$and": [{ "name": "Bob" }, { "age": 20 }] })

        self.assertTrue(len(docs) == 1)
        self.assertTrue(docs[0]["name"] == "Bob")
    
    def test_query_nomatch_with_and(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "name": "Rob", "age": 25 })
        coll.insert({ "name": "Bob", "age": 20 })
        coll.insert({ "name": "Tom", "age": 30 })

        docs = coll.find({ "$and": [{ "name": "Bob" }, { "age": 21 }] })

        self.assertTrue(len(docs) == 0)
    
    def test_query_match_with_or(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "name": "Rob", "age": 25 })
        coll.insert({ "name": "Bob", "age": 20 })
        coll.insert({ "name": "Tom", "age": 30 })

        docs = coll.find({ "$or": [{ "name": "Bob" }, { "age": 30 }] })

        self.assertTrue(len(docs) == 2)
    
    def test_query_nomatch_with_or(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "name": "Rob", "age": 25 })
        coll.insert({ "name": "Bob", "age": 20 })
        coll.insert({ "name": "Tom", "age": 30 })

        docs = coll.find({ "$or": [{ "name": "Toby" }, { "age": 40 }] })

        self.assertTrue(len(docs) == 0)
    
    def test_eq_op(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] })
        coll.insert({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] })
        coll.insert({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] })
        coll.insert({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] })
        coll.insert({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] })

        docs = coll.find({ "qty": { "$eq": 20 } })

        self.assertTrue(len(docs) == 2)
        self.assertTrue(docs[0]["item"]["name"] == "cd")
        self.assertTrue(docs[1]["item"]["name"] == "mn")
    
    def test_eq_nomatch_op(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] })
        coll.insert({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] })
        coll.insert({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] })
        coll.insert({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] })
        coll.insert({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] })

        docs = coll.find({ "qty": { "$eq": 200 } })

        self.assertTrue(len(docs) == 0)
    
    def test_eq_op_single_entry_embedded_doc(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] })
        coll.insert({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] })
        coll.insert({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] })
        coll.insert({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] })
        coll.insert({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] })

        docs = coll.find({ "item.name": { "$eq": "ab" } })

        self.assertTrue(len(docs) == 1)
        self.assertTrue(docs[0]["item"]["name"] == "ab")
    
    def test_eq_op_to_match_array_to_array(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] })
        coll.insert({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] })
        coll.insert({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] })
        coll.insert({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] })
        coll.insert({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] })

        docs = coll.find({ "tags": { "$eq": [ "A", "B"  ] } })

        self.assertTrue(len(docs) == 2)
        self.assertTrue(docs[0]["item"]["name"] == "ij")
        self.assertTrue(docs[1]["item"]["name"] == "mn")
    
    def test_eq_op_to_nomatch_array_to_array(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] })
        coll.insert({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] })
        coll.insert({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] })
        coll.insert({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] })
        coll.insert({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] })

        docs = coll.find({ "tags": { "$eq": [ "C", "D"  ] } })

        self.assertTrue(len(docs) == 0)
    
    def test_eq_op_to_match_array_to_value(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] })
        coll.insert({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] })
        coll.insert({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] })
        coll.insert({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] })
        coll.insert({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] })

        docs = coll.find({ "tags": { "$eq": "B" } })

        self.assertTrue(len(docs) == 4)
        self.assertTrue(docs[0]["item"]["name"] == "ab")
        self.assertTrue(docs[1]["item"]["name"] == "cd")
        self.assertTrue(docs[2]["item"]["name"] == "ij")
        self.assertTrue(docs[3]["item"]["name"] == "xy")
    
    def test_gt_match(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] })
        coll.insert({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] })
        coll.insert({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] })
        coll.insert({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] })
        coll.insert({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] })

        docs = coll.find({ "qty": { "$gt": 20 } })

        self.assertTrue(len(docs) == 2)
        self.assertTrue(docs[0]["item"]["name"] == "ij")
        self.assertTrue(docs[1]["item"]["name"] == "xy")
    
    def test_gt_no_match(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] })
        coll.insert({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] })
        coll.insert({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] })
        coll.insert({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] })
        coll.insert({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] })

        docs = coll.find({ "qty": { "$gt": 200 } })

        self.assertTrue(len(docs) == 0)
    
    def test_gt_match_embedded_doc(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "item": { "name": "ab", "code": 123 }, "qty": 15, "tags": [ "A", "B", "C" ] })
        coll.insert({ "item": { "name": "cd", "code": 123 }, "qty": 20, "tags": [ "B" ] })
        coll.insert({ "item": { "name": "ij", "code": 456 }, "qty": 25, "tags": [ "A", "B" ] })
        coll.insert({ "item": { "name": "xy", "code": 456 }, "qty": 30, "tags": [ "B", "A" ] })
        coll.insert({ "item": { "name": "mn", "code": 000 }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] })

        docs = coll.find({ "item.code": { "$gt": 400 } })

        self.assertTrue(len(docs) == 2)
        self.assertTrue(docs[0]["item"]["name"] == "ij")
        self.assertTrue(docs[1]["item"]["name"] == "xy")
    
    def test_gte_match(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] })
        coll.insert({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] })
        coll.insert({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] })
        coll.insert({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] })
        coll.insert({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] })

        docs = coll.find({ "qty": { "$gte": 20 } })

        self.assertTrue(len(docs) == 4)
        self.assertTrue(docs[0]["item"]["name"] == "cd")
        self.assertTrue(docs[1]["item"]["name"] == "ij")
        self.assertTrue(docs[2]["item"]["name"] == "xy")
        self.assertTrue(docs[3]["item"]["name"] == "mn")
    
    def test_gte_no_match(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] })
        coll.insert({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] })
        coll.insert({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] })
        coll.insert({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] })
        coll.insert({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] })

        docs = coll.find({ "qty": { "$gte": 200 } })

        self.assertTrue(len(docs) == 0)

    def test_gte_match_embedded_doc(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "item": { "name": "ab", "code": 123 }, "qty": 15, "tags": [ "A", "B", "C" ] })
        coll.insert({ "item": { "name": "cd", "code": 123 }, "qty": 20, "tags": [ "B" ] })
        coll.insert({ "item": { "name": "ij", "code": 456 }, "qty": 25, "tags": [ "A", "B" ] })
        coll.insert({ "item": { "name": "xy", "code": 456 }, "qty": 30, "tags": [ "B", "A" ] })
        coll.insert({ "item": { "name": "mn", "code": 000 }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] })

        docs = coll.find({ "item.code": { "$gte": 456 } })

        self.assertTrue(len(docs) == 2)
        self.assertTrue(docs[0]["item"]["name"] == "ij")
        self.assertTrue(docs[1]["item"]["name"] == "xy")
    
    def test_lt_match(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] })
        coll.insert({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] })
        coll.insert({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] })
        coll.insert({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] })
        coll.insert({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] })

        docs = coll.find({ "qty": { "$lt": 20 } })

        self.assertTrue(len(docs) == 1)
        self.assertTrue(docs[0]["item"]["name"] == "ab")
    
    def test_lt_no_match(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] })
        coll.insert({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] })
        coll.insert({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] })
        coll.insert({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] })
        coll.insert({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] })

        docs = coll.find({ "qty": { "$lt": 2 } })

        self.assertTrue(len(docs) == 0)
    
    def test_lt_match_embedded_doc(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "item": { "name": "ab", "code": 123 }, "qty": 15, "tags": [ "A", "B", "C" ] })
        coll.insert({ "item": { "name": "cd", "code": 123 }, "qty": 20, "tags": [ "B" ] })
        coll.insert({ "item": { "name": "ij", "code": 456 }, "qty": 25, "tags": [ "A", "B" ] })
        coll.insert({ "item": { "name": "xy", "code": 456 }, "qty": 30, "tags": [ "B", "A" ] })
        coll.insert({ "item": { "name": "mn", "code": 000 }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] })

        docs = coll.find({ "item.code": { "$lt": 400 } })

        self.assertTrue(len(docs) == 3)
        self.assertTrue(docs[0]["item"]["name"] == "ab")
        self.assertTrue(docs[1]["item"]["name"] == "cd")
        self.assertTrue(docs[2]["item"]["name"] == "mn")
    
    def test_lte_match(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] })
        coll.insert({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] })
        coll.insert({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] })
        coll.insert({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] })
        coll.insert({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] })

        docs = coll.find({ "qty": { "$lte": 20 } })

        self.assertTrue(len(docs) == 3)
        self.assertTrue(docs[0]["item"]["name"] == "ab")
        self.assertTrue(docs[1]["item"]["name"] == "cd")
        self.assertTrue(docs[2]["item"]["name"] == "mn")
    
    def test_lte_no_match(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] })
        coll.insert({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] })
        coll.insert({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] })
        coll.insert({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] })
        coll.insert({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] })

        docs = coll.find({ "qty": { "$lte": 2 } })

        self.assertTrue(len(docs) == 0)
    
    def test_lte_match_embedded_doc(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "item": { "name": "ab", "code": 123 }, "qty": 15, "tags": [ "A", "B", "C" ] })
        coll.insert({ "item": { "name": "cd", "code": 123 }, "qty": 20, "tags": [ "B" ] })
        coll.insert({ "item": { "name": "ij", "code": 456 }, "qty": 25, "tags": [ "A", "B" ] })
        coll.insert({ "item": { "name": "xy", "code": 456 }, "qty": 30, "tags": [ "B", "A" ] })
        coll.insert({ "item": { "name": "mn", "code": 000 }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] })

        docs = coll.find({ "item.code": { "$lte": 123 } })

        self.assertTrue(len(docs) == 3)
        self.assertTrue(docs[0]["item"]["name"] == "ab")
        self.assertTrue(docs[1]["item"]["name"] == "cd")
        self.assertTrue(docs[2]["item"]["name"] == "mn")