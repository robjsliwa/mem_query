package memquery

import (
	"encoding/json"
	"fmt"
)

var (
	ErrCollectionNotFound = fmt.Errorf("Collection not found")
)

// Collection is a go level abstraction for exposing
// Insert and Find functions on a collection
type Collection struct {
	name string
}

// CreateCollection is a factory for creating named collections
func CreateCollection(name string) (*Collection, error) {
	namePtr, nameLen, err := WriteString(name)
	if err != nil {
		return nil, err
	}

	createCollection, err := instance.Exports.GetFunction("create_collection")
	if err != nil {
		return nil, err
	}
	_, err = createCollection(namePtr, nameLen)
	if err != nil {
		return nil, err
	}

	return &Collection{name}, nil
}

// GetCollection will find a previously created collection by name
func GetCollection(name string) (*Collection, error) {
	namePtr, nameLen, err := WriteString(name)
	if err != nil {
		return nil, err
	}

	collection, err := instance.Exports.GetFunction("collection")
	if err != nil {
		return nil, err
	}

	res, err := collection(namePtr, nameLen)
	if err != nil {
		return nil, err
	}

	if res == int32(0) {
		return nil, ErrCollectionNotFound
	}

	return &Collection{name}, err
}

// Insert allows consumers to add JSON documents to a collection
func (c *Collection) Insert(doc map[string]interface{}) error {
	namePtr, nameLen, err := WriteString(c.name)
	if err != nil {
		return err
	}

	bb, err := json.Marshal(doc)
	if err != nil {
		return err
	}

	docPtr, docLen, err := WriteString(string(bb))
	if err != nil {
		return err
	}
	insert, err := instance.Exports.GetFunction("insert")
	if err != nil {
		return err
	}
	resultPtr, _ := insert(namePtr, nameLen, docPtr, docLen)
	strAddr, err := LinearMemoryAddr()
	if err != nil {
		return err
	}
	result, resultLen, err := ResultPtrToValue(strAddr, resultPtr.(int32))
	if err != nil {
		return fmt.Errorf("failed to insert document: %w", err)
	}

	if result != nil {
		dealloc, err := instance.Exports.GetFunction("dealloc")
		if err != nil {
			return fmt.Errorf("%w", err)
		}
		dealloc(resultPtr, resultLen)
	}

	return nil
}

// Find allows a consumer to execute a query to filter documents in a collection
func (c *Collection) Find(query map[string]interface{}) (*Result, error) {
	namePtr, nameLen, err := WriteString(c.name)
	if err != nil {
		return nil, err
	}
	queryBytes, err := json.Marshal(query)
	if err != nil {
		return nil, err
	}

	queryPtr, queryLen, err := WriteString(string(queryBytes))
	if err != nil {
		return nil, err
	}

	find, err := instance.Exports.GetFunction("find")
	if err != nil {
		return nil, err
	}
	resultPtr, err := find(namePtr, nameLen, queryPtr, queryLen)
	if err != nil {
		return nil, err
	}
	strAddr, err := LinearMemoryAddr()
	if err != nil {
		return nil, err
	}
	result, _, err := ResultPtrToValue(strAddr, resultPtr.(int32))
	if err != nil {
		return nil, err
	}
	return result, nil
}
