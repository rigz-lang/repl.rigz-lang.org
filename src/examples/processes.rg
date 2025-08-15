a = spawn do
    "first"
end

b = spawn do
    "second"
end

receive [a, b]