package memquery

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"

	wasmer "github.com/wasmerio/wasmer-go/wasmer"
)

var (
	instance *wasmer.Instance
	store    *wasmer.Store
)

func init() {
	var err error
	instance, err = Instance("./wsmemquery.wasm")
	if err != nil {
		log.Fatalf("failed to load wasm: %v", err)
	}
}

// type A represents an array of arbitrary types
type A []interface{}

// type M represents a JSON
type M map[string]interface{}

// instantiates a wasmer.Instance or returns an error
func Instance(path string) (*wasmer.Instance, error) {
	wasmBytes, err := ioutil.ReadFile(path)
	if err != nil {
		return nil, err
	}
	store = wasmer.NewStore(wasmer.NewEngine())

	// Compiles the module
	module, err := wasmer.NewModule(store, wasmBytes)
	if err != nil {
		return nil, err
	}

	// Instantiates the module
	return wasmer.NewInstance(module, wasmer.NewImportObject())
}

func WriteToMemory(text string, memoryAddr []byte, offset int32) []byte {
	bytestr := []byte(text)
	for i := range bytestr {
		memoryAddr[int32(i)+offset] = bytestr[i]
	}
	return memoryAddr
}

func PointerToStringWithLength(memoryAddr []byte, ptr int32, length int) string {
	var text bytes.Buffer
	bb := memoryAddr[ptr : ptr+int32(length)]
	for _, c := range bb {
		text.WriteByte(c)
	}
	return text.String()
}

func PointerToString(memoryAddr []byte, ptr int32) (string, int) {
	var text bytes.Buffer
	i := 0
	c := memoryAddr[ptr+int32(i)]

	for c != 0 {
		text.WriteByte(c)
		i += 1
		c = memoryAddr[ptr+int32(i)]
	}
	retVal := text.String()
	return retVal, len(retVal)
}

func WriteString(name string) (int32, int, error) {
	nameLen := len(name)
	alloc, err := instance.Exports.GetFunction("alloc")
	if err != nil {
		return 0, 0, err
	}
	namePtr, err := alloc(nameLen)
	if err != nil {
		return 0, 0, err
	}
	if namePtr, ok := namePtr.(int32); ok {
		memoryAddr, err := LinearMemoryAddr()
		if err != nil {
			return 0, 0, err
		}

		WriteToMemory(name, memoryAddr, namePtr)
		return namePtr, nameLen, nil
	}

	return 0, 0, fmt.Errorf("failed to convert namePtr to int32")

}

type Result struct {
	Value interface{} `json:"value,omitempty"`
	Error string      `json:"error,omitempty"`
}

func ResultPtrToValue(memoryAddr []byte, resultPtr int32) (*Result, int, error) {
	result, resultLen := PointerToString(memoryAddr, resultPtr)
	var resObject Result
	err := json.Unmarshal([]byte(result), &resObject)
	return &resObject, resultLen, err
}

func LinearMemoryAddr() ([]byte, error) {
	memory, err := instance.Exports.GetMemory("memory")
	if err != nil {
		return nil, err
	}
	return memory.Data(), nil

}
