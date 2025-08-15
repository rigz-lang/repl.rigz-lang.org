lists = [for a in [1, 2, 3, 4]: a * a]

maps = {for k, v in {a = 1, b = 2, c = 3, d = 4}: v, k * v}

mut a = 0
loop
    a += 1
    break if a == 10
end

mut res = []

for v in lists
    next unless v % 2 == 0

    res.push v
end

{ a, lists, maps, res }