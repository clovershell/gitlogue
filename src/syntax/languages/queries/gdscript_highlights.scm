; Comments and literals
(comment) @comment
(string) @string
(string_name) @string
(node_path) @string
(escape_sequence) @string.escape
(integer) @number
(float) @number
[
  (true)
  (false)
  (null)
] @constant

; Types and declarations
(type) @type
(class_name_statement (name) @type)
(class_definition (name) @type)
(enum_definition (name) @type)
(enumerator (identifier) @constant)
(variable_statement (identifier) @variable)
(const_statement (name) @constant)
(signal_statement (name) @label)

; Functions, methods, and members
(function_definition
  name: (name) @function)
(constructor_definition "_init" @function)
(lambda (name) @function)
(parameters
  [
    (identifier) @parameter
    (typed_parameter (identifier) @parameter)
    (default_parameter (identifier) @parameter)
    (typed_default_parameter (identifier) @parameter)
    (variadic_parameter (identifier) @parameter)
    (variadic_parameter (typed_parameter (identifier) @parameter))
    (variadic_parameter (default_parameter (identifier) @parameter))
    (variadic_parameter (typed_default_parameter (identifier) @parameter))
  ])
(call (identifier) @function)
(attribute_call (identifier) @function)
(base_call (identifier) @function)
(attribute
  (identifier)
  (identifier) @property)

; Annotations and keywords
(annotation (identifier) @keyword)
[
  "if"
  "else"
  "elif"
  "match"
  "when"
  "while"
  "for"
  "return"
  "pass"
  "break"
  "continue"
  "func"
  "export"
  "in"
  "is"
  "as"
  "and"
  "or"
  "not"
  "var"
  "class"
  "class_name"
  "enum"
  "const"
  "signal"
  "setget"
  "onready"
  "extends"
  "set"
  "get"
  "await"
  (remote_keyword)
  (static_keyword)
] @keyword

; Operators and punctuation
[
  "+"
  "-"
  "*"
  "/"
  "%"
  "=="
  "!="
  ">"
  "<"
  ">="
  "<="
  "="
  "+="
  "-="
  "*="
  "/="
  "%="
  "&"
  "|"
  "^"
  "~"
  "<<"
  ">>"
  ":="
  ":"
] @operator
[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
  ","
  "."
] @punctuation
