import wasmtime.loader
import wsmemquery as instance
import json
from errors import CreateCollectionFailed, InsertDocumentFailed, FindCollectionError
from membind import write_str, linear_mem_addr, ptr_to_str

# memquery API

class Collection:
  def __init__(self, name):
    self._name = name
  
  def insert(self, doc):
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
    pass

def create_collection(name):
  name_ptr, name_len = write_str(instance, name)

  try:
    instance.create_collection(name_ptr, name_len)
  except Exception as e:
    raise CreateCollectionFailed(e)

def collection(name):
  name_ptr, name_len = write_str(instance, name)

  try:
    res = instance.collection(name_ptr, name_len)
    if res == 0:
      raise FindCollectionError('Collection not found')
  except Exception as e:
    raise FindCollectionError(e)

  return Collection(name)


if __name__ == '__main__':
  create_collection('TestCollection')
  create_collection('TestCollection1')

  test_coll = collection('TestCollection')
  test_coll.insert({ "name": "Tomek" })

  test_coll1 = collection('TestCollection1')
  test_coll1.insert({ "name": "Tomeczek" })
