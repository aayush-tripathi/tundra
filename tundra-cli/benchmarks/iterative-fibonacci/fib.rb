n = 80
a, b = 0, 1
n.times do
  a, b = b, a + b
end
puts a