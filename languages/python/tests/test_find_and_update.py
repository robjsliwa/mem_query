from unittest import TestCase
from memquery import Collection, create_collection,\
    collection


class TestFindAndUpdateAPI(TestCase):
    def test_simple_update(self):
        create_collection("TestCollection");
        coll = collection("TestCollection")
        coll.insert({ "name": "Rob", "age": 25 })
        coll.insert({ "name": "Bob", "age": 20 })
        coll.insert({ "name": "Tom", "age": 30 })

        docs_updated = coll.find_and_update(
            {"name": "Bob"},
            {"nickname": "Bobcat", "voice": "meow"},
        )

        self.assertTrue(docs_updated == 1);

        docs = coll.find({"nickname": "Bobcat"})
        self.assertTrue(len(docs) == 1)
        self.assertTrue(docs[0]["voice"] == "meow")

    def test_set_op_update(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "name": "Rob", "age": 25 })
        coll.insert({ "name": "Bob", "age": 20 })
        coll.insert({ "name": "Tom", "age": 30 })

        docs_updated = coll.find_and_update(
            {"name": "Bob"},
            {"$set": { "name": "Roy", "age": 21, "email": "test@test.com"}},
        )

        self.assertTrue(docs_updated == 1)

        docs = coll.find({"name": "Roy"})
        self.assertTrue(len(docs) == 1)
        self.assertTrue(docs[0]["age"] == 21)
        self.assertTrue(docs[0]["email"] == "test@test.com")
    
    def test_set_op_invalid_value_update(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "name": "Rob", "age": 25 })
        coll.insert({ "name": "Bob", "age": 20 })
        coll.insert({ "name": "Tom", "age": 30 })

        try:
            coll.find_and_update(
                {"name": "Bob"},
                {"$set": { "$name": "Roy", "age": 21, "email": "test@test.com"}},
            )
        except Exception:
            return

        self.assertTrue("should get error" == "no error")

    def test_set_op_invalid_value_embedded_update(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "name": "Rob", "age": 25 })
        coll.insert({ "name": "Bob", "age": 20 })
        coll.insert({ "name": "Tom", "age": 30 })

        try:
            coll.find_and_update(
                {"name": "Bob"},
                {"$set": { "name": "Roy", "age.$set": 21, "email": "test@test.com"}}
            )
        except Exception:
            return

        self.assertTrue("should get error" == "no error")

    def test_unset_op_update(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "name": "Rob", "age": 25 })
        coll.insert({ "name": "Bob", "age": 20 })
        coll.insert({ "name": "Tom", "age": 30 })

        docs_updated = coll.find_and_update(
            { "name": "Bob" },
            { "$set": { "name": "Roy", "age": 21, "email": "test@test.com" }},
        )

        self.assertTrue(docs_updated == 1)

        docs = coll.find({"name": "Roy"})
        self.assertTrue(len(docs) == 1)
        self.assertTrue(docs[0]["age"] == 21)
        self.assertTrue(docs[0]["email"] == "test@test.com")

        docs_updated2 = coll.find_and_update(
            { "name": "Roy" },
            { "$unset": { "email": "" }},
        )

        self.assertTrue(docs_updated2 == 1)

        docs = coll.find({"name": "Roy"})
        self.assertTrue(len(docs) == 1)
        self.assertTrue(docs[0]["age"] == 21)
        self.assertTrue(docs[0].get("email", None) == None)

    def test_set_op_on_embedded_doc_update(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert(
            { "name": "Rob", "age": 25, "profile": { "email": "rob@test.com" } })
        coll.insert(
            { "name": "Bob", "age": 20, "profile": { "email": "bob@test.com" }  })
        coll.insert({ "name": "Tom", "age": 30, "profile": { "email": "tom@test.com" }  })

        docs_updated = coll.find_and_update(
            {"name": "Bob"},
            {"$set": { "profile.email": "tom@test.com"}},
        )

        self.assertTrue(docs_updated == 1)

        docs = coll.find({"name": "Bob"})
        self.assertTrue(len(docs) == 1)
        self.assertTrue(docs[0]["age"] == 20)
        self.assertTrue(docs[0]["profile"]["email"] == "tom@test.com")

    def test_unset_op_on_embedded_doc_update(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "name": "Rob", "age": 25, "profile": { "email": "rob@test.com" } })
        coll.insert({ "name": "Bob", "age": 20, "profile": { "email": "bob@test.com" }  })
        coll.insert({ "name": "Tom", "age": 30, "profile": { "email": "tom@test.com" }  })

        docs_updated = coll.find_and_update(
            {"name": "Bob"},
            {"$set": { "profile.email": "tom@test.com"}},
        )

        self.assertTrue(docs_updated == 1)

        docs = coll.find({"name": "Bob"})
        self.assertTrue(len(docs) == 1)
        self.assertTrue(docs[0]["age"] == 20)
        self.assertTrue(docs[0]["profile"]["email"] == "tom@test.com")

        docs_updated = coll.find_and_update(
            {"name": "Bob"},
            {"$unset": { "profile.email": "tom@test.com"}},
        )

        self.assertTrue(docs_updated == 1)

        docs = coll.find({"name": "Bob"})
        self.assertTrue(docs[0]["profile"].get("email", None) == None)

    def test_inc_positive_op_update(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "name": "Rob", "age": 25 })
        coll.insert({ "name": "Bob", "age": 20 })
        coll.insert({ "name": "Tom", "age": 30 })

        docs_updated = coll.find_and_update(
            {"name": "Bob"}, {"$inc": { "age": 5}})

        self.assertTrue(docs_updated == 1)

        docs = coll.find({"name": "Bob"})
        self.assertTrue(len(docs) == 1)
        self.assertTrue(docs[0]["age"] == 25.0)

    def test_inc_negative_op_update(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "name": "Rob", "age": 25 })
        coll.insert({ "name": "Bob", "age": 20 })
        coll.insert({ "name": "Tom", "age": 30 })

        docs_updated = coll.find_and_update(
            {"name": "Bob"}, {"$inc": { "age": -5 }})

        self.assertTrue(docs_updated == 1)

        docs = coll.find({"name": "Bob"})
        self.assertTrue(len(docs) == 1)
        self.assertTrue(docs[0]["age"] == 15.0)

    def test_mul_positive_op_update(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "name": "Rob", "age": 25 })
        coll.insert({ "name": "Bob", "age": 20 })
        coll.insert({ "name": "Tom", "age": 30 })

        docs_updated = coll.find_and_update(
            {"name": "Bob"}, {"$mul": { "age": 5}})

        self.assertTrue(docs_updated == 1)

        docs = coll.find({"name": "Bob"})
        self.assertTrue(len(docs) == 1)
        self.assertTrue(docs[0]["age"] == 100.0)

    def test_mul_negative_op_update(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "name": "Rob", "age": 25 })
        coll.insert({ "name": "Bob", "age": 20 })
        coll.insert({ "name": "Tom", "age": 30 })

        docs_updated = coll.find_and_update(
            {"name": "Bob"}, {"$mul": { "age": -5 }})

        self.assertTrue(docs_updated == 1)

        docs = coll.find({"name": "Bob"})
        self.assertTrue(len(docs) == 1)
        self.assertTrue(docs[0]["age"] == -100.0)
