# Recursive Fibonacci in Ruby

def fib(n)
  return n if n < 2
  fib(n - 1) + fib(n - 2)
end

puts fib(30)