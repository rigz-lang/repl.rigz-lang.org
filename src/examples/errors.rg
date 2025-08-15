fn foo = raise "foo failed"

bar = foo catch = "hello"

baz = foo catch
    1 + 2
end

bar + baz