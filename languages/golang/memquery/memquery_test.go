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

	err = DeleteCollection("Test")
	assert.Nil(t, err, "cleanup failed")
}

func TestDeleteCollection(t *testing.T) {
	_, err := CreateCollection("Test")
	assert.Nil(t, err, "create collection failed")

	err = DeleteCollection("Test")
	assert.Nil(t, err, "cleanup failed")

}
func TestGetNonExistentCollection(t *testing.T) {
	_, err := GetCollection("TestNonExistent")
	assert.Equal(t, ErrCollectionNotFound, err, "get collection failed")
}

func TestInsertCollection(t *testing.T) {
	c, err := CreateCollection("Test")
	assert.Nil(t, err, "create collection failed")
	assert.NotNil(t, c)

	doc := M{"name": "Ram", "age": float64(20)}

	err = c.Insert(doc)
	assert.Nil(t, err, "insert doc failed")

	err = DeleteCollection("Test")
	assert.Nil(t, err, "cleanup failed")

}

func TestSimpleQuery(t *testing.T) {
	docs := A{
		M{"name": "Ram", "age": float64(30)},
		M{"name": "Shyam", "age": float64(35)},
		M{"name": "Ghanshyam", "age": float64(40)},
	}
	c, err := CreateCollection("Test")
	assert.Nil(t, err, "create collection failed")
	assert.NotNil(t, c)

	for _, d := range docs {
		err = c.Insert(d)
		assert.Nil(t, err, "insert doc failed")
	}

	res, err := c.Find(M{"name": "Ram"})
	assert.Nil(t, err, "failed to query collection")
	assert.Empty(t, res.Error, "find failed")

	if v, ok := res.Value.([]interface{}); ok {
		assert.Equal(t, len(v), 1, "matched more than one document")
		if vv, ok := v[0].(map[string]interface{}); ok {
			assert.Equal(t, vv["name"], "Ram")
		} else {
			t.Errorf("retrived document inconsistent with query")
		}

	} else {
		t.Errorf("Retrieved document of not []interface{}")
	}

}

func TestSimpleQueryMultipleConditions(t *testing.T) {
	docs := A{
		M{"name": "Ram", "age": float64(30)},
		M{"name": "Shyam", "age": float64(35)},
		M{"name": "Ghanshyam", "age": float64(40)},
	}
	c, err := CreateCollection("Test")
	assert.Nil(t, err, "create collection failed")
	assert.NotNil(t, c)

	for _, d := range docs {
		err = c.Insert(d)
		assert.Nil(t, err, "insert doc failed")
	}

	res, err := c.Find(map[string]interface{}{"name": "Ram", "age": float64(30)})
	assert.Nil(t, err, "failed to query collection")
	assert.Empty(t, res.Error, "find failed")

	if v, ok := res.Value.([]interface{}); ok {
		assert.Equal(t, len(v), 1, "matched more than one document")
		if vv, ok := v[0].(map[string]interface{}); ok {
			assert.Equal(t, vv["name"], "Ram")
		} else {
			t.Errorf("retrieved document inconsistent with query")
		}

	} else {
		t.Errorf("Retrieved document of not []interface{}")
	}

}

func TestMatchWithAndOperator(t *testing.T) {
	docs := A{
		M{"name": "Ram", "age": float64(30)},
		M{"name": "Shyam", "age": float64(35)},
		M{"name": "Ghanshyam", "age": float64(40)},
	}
	c, err := CreateCollection("Test")
	assert.Nil(t, err, "create collection failed")
	assert.NotNil(t, c)

	for _, d := range docs {
		err = c.Insert(d)
		assert.Nil(t, err, "insert doc failed")
	}

	res, err := c.Find(M{"$and": A{M{"name": "Ram"}, M{"age": float64(30)}}})
	assert.Nil(t, err, "failed to query collection")
	assert.Empty(t, res.Error, "find failed")

	if v, ok := res.Value.([]interface{}); ok {
		assert.Equal(t, len(v), 1, "matched more than one document")
		if len(v) == 1 {
			if vv, ok := v[0].(map[string]interface{}); ok {
				assert.Equal(t, vv["name"], "Ram")
			} else {
				t.Errorf("retrieved document inconsistent with query")
			}
		}
	} else {
		t.Errorf("Retrieved document of not []interface{}")
	}

}

func TestMatchWithOrOperator(t *testing.T) {
	docs := A{
		M{"name": "Ram", "age": float64(30)},
		M{"name": "Shyam", "age": float64(35)},
		M{"name": "Ghanshyam", "age": float64(40)},
	}
	c, err := CreateCollection("Test")
	assert.Nil(t, err, "create collection failed")
	assert.NotNil(t, c)

	for _, d := range docs {
		err = c.Insert(d)
		assert.Nil(t, err, "insert doc failed")
	}

	res, err := c.Find(M{"$or": A{M{"name": "Ram"}, M{"age": float64(40)}}})
	assert.Nil(t, err, "failed to query collection")
	assert.Empty(t, res.Error, "find failed")

	if v, ok := res.Value.([]interface{}); ok {
		assert.Equal(t, len(v), 2, "matched more than one document")
	} else {
		t.Errorf("Retrieved document of not []interface{}")
	}

}

func TestMatchWithEqualOperator(t *testing.T) {
	docs := A{
		M{"name": "Ram", "age": float64(30)},
		M{"name": "Shyam", "age": float64(35)},
		M{"name": "Ghanshyam", "age": float64(40)},
	}
	c, err := CreateCollection("Test")
	assert.Nil(t, err, "create collection failed")
	assert.NotNil(t, c)

	for _, d := range docs {
		err = c.Insert(d)
		assert.Nil(t, err, "insert doc failed")
	}

	res, err := c.Find(M{"age": M{"$eq": float64(40)}})
	assert.Nil(t, err, "failed to query collection")
	assert.Empty(t, res.Error, "find failed")

	if v, ok := res.Value.([]interface{}); ok {
		assert.Equal(t, len(v), 1, "matched more than one document")
		if len(v) == 1 {
			if vv, ok := v[0].(map[string]interface{}); ok {
				assert.Equal(t, vv["name"], "Ghanshyam")
			} else {
				t.Errorf("retrieved document inconsistent with query")
			}
		}
	} else {
		t.Errorf("Retrieved document of not []interface{}")
	}

}
