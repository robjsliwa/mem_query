import wasmtime.loader
import wsmemquery as instance
import json
from errors import CreateCollectionFailed, InsertDocumentFailed, FindCollectionError

# Memory and Types

def linear_mem_addr(instance, size=8):
  mem = instance.memory
  return mem.data_ptr

def write_to_memory(text, mem_addr, offset=0):
  bytestr = bytes(text, 'utf8')
  for i, c in enumerate(bytestr):
    mem_addr[i + offset] = c
  return mem_addr

def ptr_to_str_with_len(mem_addr, ptr, length):
  text = ''
  bytes = mem_addr[ptr:ptr+length]
  for _, c in enumerate(bytes):
    text += chr(c)
  return text

def ptr_to_str(mem_addr, ptr):
  '''ptr is null terminated'''
  text = ''
  i = 0
  c = mem_addr[ptr+i]
  while c != 0:
    text += chr(c)
    i += 1
    c = mem_addr[ptr+i]
  return text, len(text)
  
def write_str(name, mem_addr):
  name_len = len(name)
  name_ptr = instance.alloc(name_len)
  write_to_memory(name, linear_mem_addr(instance), name_ptr)
  return name_ptr, name_len

# memquery API

class Collection:
  def __init__(self, name):
    self._name = name
  
  def insert(self, doc):
    name_ptr, name_len = write_str(self._name, linear_mem_addr(instance))
    docstr = json.dumps(docs)
    doc_ptr, doc_len = write_str(docstr, linear_mem_addr(instance))
    res_ptr = None
    res_len = 0

    try:
      result = instance.insert(name_ptr, name_len, doc_ptr, doc_len)
      res, res_len = ptr_to_str(linear_mem_addr(instance), result)
      res_json = json.parse(res)
      print(f'res_json {res_json}')
      if res_json['error'] is None:
        raise InsertDocumentFailed(json.dumps(res_json['error']))

    except e:
      raise InsertDocumentFailed(e)
    finally:
      instance.dealloc(name_ptr, name_len)
      instance.dealloc(doc_ptr, doc_len)
      if res_ptr is not None:
        print(f'res_ptr {res_ptr}')
        instance.dealloc(res_ptr, res_len)

  def find(self, query):
    pass

def create_collection(name):
  name_ptr, name_len = write_str(name, linear_mem_addr(instance))

  try:
    instance.create_collection(name_ptr, name_len)
  except e:
    raise CreateCollectionFailed(e)
  finally:
    instance.dealloc(name_ptr, name_len)

def collection(name):
  name_ptr, name_len = write_str(name, linear_mem_addr(instance))

  try:
    res = instance.collection(name_ptr, name_len)
    if res == 0:
      raise FindCollectionError('Collection not found')
  except e:
    raise FindCollectionError(e)
  finally:
    instance.dealloc(name_ptr, name_len)

  return Collection(name)


if __name__ == '__main__':
  create_collection('TestCollection')
  test_coll = collection('TestCollection')
  test_coll.insert({ "name": "Tomek" })