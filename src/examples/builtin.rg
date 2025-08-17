a = 42 # by default variables are immutable
let a = "foo" # use let to shadow a variable
mut b = [:baz] # use mut for mutable variables

b += :baz # most types support common operators: +, -, *, /, %, ^, |, ||, &, &&, <, <=, <<, >, >=. >>

puts a, b # puts displays each variable passed in on a newline
println 'println: ', a, b # print concatenates values together, with no commas

# ?: can be used to default to a value if it is none or an error, || checks if it's truthy
fmt = format("{} {}", [] || a, none ?: b)

eprintln 'Error: ', fmt # print, eprint, println, eprintln are available

printf "printf: {} {}", a, b # printf adds a newline after a template

mut b = [] # this will shadow b

/*
* Default log level is info, so the first two logs won't be displayed
* The VM has trace and debug logs, long term you'll be able to disabled them with an option
*/
log :trace, 'This is a trace log'
log :debug, 'This is a debug log'
log :info, 'an info log' # log does not currently support dynamic log levels, but it is case insensitive
log :warn, 'This is a warning log {}', 47 # templating is supported

template = 'This is dynamic log template {}'
b += [1, 2, 3]
log :error, template, b

b