; Highlight HTTP methods
(method) @keyword

; Highlight comments
(metadata) @comment
(comment) @comment
(request_separator) @comment

; Highlight URLs and paths
(target_url) @string.url
(path) @string.url
(http_version) @tag

; Highlight HTTP headers
(header name: (header_entity) @function.method)
; (header value: (value) @string)

; Highlight HTTP status codes and status texts
(status_code) @constant.numeric
(status_text) @constant.language

; Highlight variables and script variables
(variable_declaration name: (identifier) @constant)
(variable) @variable
(variable name: (identifier) @constant)

; Highlight different types of request bodies
(json_body) @string.special
(xml_body) @string.special
(graphql_body) @string.special
(external_body) @string.special

; Highlight query parameters
(query_param) @string
