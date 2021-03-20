import wasmtime.loader
import wsmemquery

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
  return text
  
def write_str(name, mem_addr):
  name_len = len(name)
  name_ptr = wsmemquery.alloc(name_len)
  write_to_memory(name, linear_mem_addr(wsmemquery), name_ptr)
  return name_ptr, name_len

# memquery API

def create_collection(name):
  name_ptr, name_len = write_str(name, linear_mem_addr(wsmemquery))

  try:
    res_ptr = wsmemquery.create_collection(name_ptr, name_len)
    resstr = ptr_to_str(linear_mem_addr(wsmemquery), res_ptr)
    print(f"Result: {resstr}")
  except e:
    print(e)
    print('Failed to create_collection.')
  finally:
    wsmemquery.dealloc(name_ptr, name_len)

if __name__ == '__main__':
  create_collection('TestCollection')
  print(wsmemquery.mytest(2))