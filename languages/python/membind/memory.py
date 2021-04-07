import json


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


def write_str(instance, name):
    name_len = len(name)
    name_ptr = instance.alloc(name_len)
    write_to_memory(name, linear_mem_addr(instance), name_ptr)
    return name_ptr, name_len


def result_ptr_to_value(mem_addr, res_ptr):
    result, res_len = ptr_to_str(mem_addr, res_ptr)
    res_json = json.loads(result)
    if res_json.get('error', None) is not None:
        return None, res_json['error']
    return res_json.get('value', []), None
