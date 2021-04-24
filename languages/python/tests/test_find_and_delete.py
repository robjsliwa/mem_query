from unittest import TestCase
from memquery import Collection, create_collection,\
    collection


class TestFindAndDeleteAPI(TestCase):
    def test_simple_delete(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "name": "Rob", "age": 25 })
        coll.insert({ "name": "Bob", "age": 20 })
        coll.insert({ "name": "Tom", "age": 30 })

        docs = coll.find_and_delete(
            {"name": "Bob"})
        self.assertTrue(len(docs) == 1)
        self.assertTrue(docs[0]["age"] == 20)

        docs_remaining = coll.find({})
        self.assertTrue(len(docs_remaining) == 2)

    def test_delete_all_docs(self):
        create_collection("TestCollection")
        coll = collection("TestCollection")
        coll.insert({ "name": "Rob", "age": 25 })
        coll.insert({ "name": "Bob", "age": 20 })
        coll.insert({ "name": "Tom", "age": 30 })

        docs = coll.find_and_delete({})
        self.assertTrue(len(docs) == 3)

        docs_remaining = coll.find({})
        self.assertTrue(len(docs_remaining) == 0)
