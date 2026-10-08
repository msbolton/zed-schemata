(schema_declaration
  "schema" @context
  name: (qualified_name) @name) @item

(model_declaration
  "model" @context
  name: (identifier) @name) @item

(enum_declaration
  "enum" @context
  name: (identifier) @name) @item

(union_declaration
  "union" @context
  name: (identifier) @name) @item

(alias_declaration
  "alias" @context
  name: (identifier) @name) @item

(service_declaration
  "service" @context
  name: (identifier) @name) @item

(field
  name: (identifier) @name) @item

(enum_value
  name: (identifier) @name) @item

(operation
  name: (identifier) @name) @item
