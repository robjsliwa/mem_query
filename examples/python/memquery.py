from wasmer import engine, Store, Module, Instance
from wasmer_compiler_cranelift import Compiler

# Let's define the store, that holds the engine, that holds the compiler.
store = Store(engine.JIT(Compiler))

# Let's compile the module to be able to execute it!
module = Module(store, open('../../wasm/pkg/memquery_wasm_bg.wasm', 'rb').read())
# module = Module(store, open('/home/robsliwa/Downloads/memquery_wasm.wasm', 'rb').read())

# Now the module is compiled, we can instantiate it.
instance = Instance(module)

# Call the exported `sum` function.
# result = instance.exports.sum(5, 37)
# print(result)

print('Hello')
print(instance.exports)
