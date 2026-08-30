(comment) @comment

(string) @string

[
  (integer)
  (float)
] @number

[
  (true)
  (false)
  (null)
] @constant.builtin

(section
  .
  (identifier) @keyword)

(attribute
  .
  (identifier) @property)

(property
  .
  (path) @property)

(constructor
  .
  (identifier) @type)

(identifier) @variable

[
  "="
  ":"
] @operator

[
  "["
  "]"
  "("
  ")"
  "{"
  "}"
] @punctuation.bracket

"," @punctuation.delimiter
