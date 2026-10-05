; Sibling of queries/highlights.scm in https://github.com/msbolton/tree-sitter-schemata; carry a fix to both.
(comment) @comment
(doc_comment) @comment.doc

[
  "namespace"
  "import"
  "as"
  "record"
  "enum"
  "union"
  "alias"
  "reserved"
  "service"
  "operation"
  "stream"
] @keyword

(boolean) @boolean
(integer) @number
(float) @number
(string) @string
(escape_sequence) @string.escape
(ordinal) @constant

(namespace_declaration name: (qualified_name (identifier) @namespace))
(import_declaration namespace: (qualified_name (identifier) @namespace))
(import_declaration alias: (identifier) @namespace)

(record_declaration name: (identifier) @type)
(enum_declaration name: (identifier) @type)
(union_declaration name: (identifier) @type)
(alias_declaration name: (identifier) @type)
(field name: (identifier) @property)
(enum_value name: (identifier) @variant)
(service_declaration name: (identifier) @type)
(operation name: (identifier) @function)

(http_binding verb: (identifier) @keyword)

(type name: (qualified_name (identifier) @type))
((type name: (qualified_name . (identifier) @type.builtin .))
  (#match? @type.builtin "^(bool|int32|int64|float32|float64|decimal|string|bytes|uuid|date|time|instant|duration|list|map)$"))

(field default: (identifier) @constant)
(refinement key: (identifier) @property)

(annotation "@" @attribute)
(annotation name: (identifier) @attribute)
(annotation_argument key: (identifier) @property)
(name_tuple (identifier) @property)

["{" "}" "(" ")" "<" ">"] @punctuation.bracket
["," "." ":"] @punctuation.delimiter
["=" "|" "?" ".."] @operator
