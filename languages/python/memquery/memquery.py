"""MemQuery is a library for operating on in-memory documents.

MemQuery is simple library for creating, querying and updating
in memory documents that are represented as JSON objects
and queried using Mongodb like operators.

This is not a database and it is not trying to do any optimizations.
It is meant for unit tests or simple projects that require small
in memory document store.
"""

import wasmtime.loader
import wsmemquery as instance
import json
from errors import CreateCollectionFailed, InsertDocumentFailed, FindCollectionError
from membind import write_str, linear_mem_addr, ptr_to_str, result_ptr_to_value

# memquery API


class Collection:
    """Collection stores documents.

    Collection documents are stored as JSON objects.
    """

    def __init__(self, name):
        """Creates Collection with provided name.

        Args:
          name:
            Collection name.
        """
        self._name = name

    def insert(self, doc):
        """Add document to this collection.

        Args:
          doc:
            JSON object to add to this collection.

        Returns:
          None if success or throws exception in case of an error.
        """
        name_ptr, name_len = write_str(instance, self._name)
        docstr = json.dumps(doc)
        doc_ptr, doc_len = write_str(instance, docstr)
        res_ptr = None
        res_len = 0

        try:
            res_ptr = instance.insert(name_ptr, name_len, doc_ptr, doc_len)
            result, res_len = ptr_to_str(linear_mem_addr(instance), res_ptr)
            res_json = json.loads(result)
            if res_json.get('error', None) is not None:
                raise InsertDocumentFailed(json.dumps(res_json['error']))

        except Exception as e:
            raise InsertDocumentFailed(e)
        finally:
            if res_ptr is not None:
                instance.dealloc(res_ptr, res_len)

    def find(self, query):
        """Find document(s) based on provided query.

        Args:
          query:
            JSON object that specifies query criteria.

        Returns:
          Returns list of JSON objects. If query did not match anything
          it returns empty list.  Throws exception if there was a problem
          getting data.
        """
        name_ptr, name_len = write_str(instance, self._name)
        querystr = json.dumps(query)
        query_ptr, query_len = write_str(instance, querystr)
        res_ptr = None
        res_len = 0

        res_json = []

        try:
            res_ptr = instance.find(name_ptr, name_len, query_ptr, query_len)
            res_json, err = result_ptr_to_value(
                linear_mem_addr(instance), res_ptr)
            if err is not None:
                raise FindCollectionError(err)
        except Exception as e:
            raise FindCollectionError(e)

        return res_json
    
    def find_and_update(self, query, update):
        """Find and update document(s) based on provided query with values in update variable.

        Args:
          query:
            JSON object that specifies query criteria.

        Returns:
          Returns number of documents updated. If query did not match anything
          it returns 0.  Throws exception if there was a problem
          getting data.
        """
        name_ptr, name_len = write_str(instance, self._name)
        querystr = json.dumps(query)
        query_ptr, query_len = write_str(instance, querystr)
        updatestr = json.dumps(update)
        update_ptr, update_len = write_str(instance, updatestr)
        res_ptr = None
        res_len = 0

        res_json = []

        try:
            res_ptr = instance.find_and_update(
              name_ptr,
              name_len,
              query_ptr,
              query_len,
              update_ptr,
              update_len
            )
            number_updated, err = result_ptr_to_value(
                linear_mem_addr(instance), res_ptr)
            if err is not None:
                raise FindCollectionError(err)
        except Exception as e:
            raise FindCollectionError(e)

        return number_updated
    
    def find_and_delete(self, query):
        """Find and delete document(s) based on provided query.

        Args:
          query:
            JSON object that specifies query criteria.

        Returns:
          Returns list of JSON objects that were deleted. If query 
          did not match anything it returns empty list.  Throws exception
          if there was a problem getting data.
        """
        name_ptr, name_len = write_str(instance, self._name)
        querystr = json.dumps(query)
        query_ptr, query_len = write_str(instance, querystr)
        res_ptr = None
        res_len = 0

        res_json = []

        try:
            res_ptr = instance.find_and_delete(name_ptr, name_len, query_ptr, query_len)
            res_json, err = result_ptr_to_value(
                linear_mem_addr(instance), res_ptr)
            if err is not None:
                raise FindCollectionError(err)
        except Exception as e:
            raise FindCollectionError(e)

        return res_json


def create_collection(name):
    """Create new collection.

    Args:
      name:
        Name of the collection.

    Returns:
      None on success or throws error if collection could not be created.
    """
    name_ptr, name_len = write_str(instance, name)

    try:
        instance.create_collection(name_ptr, name_len)
    except Exception as e:
        raise CreateCollectionFailed(e)


def collection(name):
    """Get Collection object.

    Args:
      name:
        Get collection object specified by name.

    Returns:
      Instance of Collection object or raises error if collection
      was not found.
    """
    name_ptr, name_len = write_str(instance, name)

    try:
        res = instance.collection(name_ptr, name_len)
        if res == 0:
            raise FindCollectionError('Collection not found')
    except Exception as e:
        raise FindCollectionError(e)

    return Collection(name)
