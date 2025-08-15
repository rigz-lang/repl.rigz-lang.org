mut a = 1
bar = do
    a += 1
    21 * a
end

fn foo = bar

@test
fn test_foo
  mut a = 1 # variables in main scope are not available for tests, this will be fixed in a later version
  bar = do
    a += 1
    21 * a
  end

  # assert_eq returns error if false
  # try is required because there are no panics
  try assert_eq foo, 42
  # scopes are only processed once
  # try not required because last line is return value
  assert_eq foo, 42
end

foo