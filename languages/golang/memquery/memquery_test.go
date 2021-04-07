package memquery

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCreateCollection(t *testing.T) {
	created, err := CreateCollection("Test")
	assert.Nil(t, err, "create collection failed")

	lookedupCollection, err := GetCollection("Test")
	assert.Nil(t, err, "get collection failed")

	assert.NotNil(t, created)
	assert.NotNil(t, lookedupCollection)

	assert.Equal(t, created, lookedupCollection)

}
func TestGetNonExistentCollection(t *testing.T) {
	_, err := GetCollection("TestNonExistent")
	assert.Equal(t, ErrCollectionNotFound, err, "get collection failed")
}

func TestInsertCollection(t *testing.T) {
	c, err := CreateCollection("Test")
	assert.Nil(t, err, "create collection failed")
	assert.NotNil(t, c)

	doc := map[string]interface{}{
		"name": "Ram",
		"age":  float64(20),
	}

	err = c.Insert(doc)
	assert.Nil(t, err, "insert doc failed")

	res, err := c.Find(map[string]interface{}{"name": "Ram"})
	assert.Nil(t, err, "failed to lookup doc")

	assert.Empty(t, res.Error, "find resulted in error")

	for _, v := range res.Value.([]interface{}) {
		if vmap, ok := v.(map[string]interface{}); ok {
			assert.Equal(t, doc, vmap, "inserted object mismatch")
		}
	}
}
