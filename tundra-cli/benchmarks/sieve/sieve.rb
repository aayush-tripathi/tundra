# File: sieve/sieve.rb
# Sieve of Eratosthenes in Ruby

def sieve(n)
  is_composite = Array.new(n + 1, false)
  p = 2
  while p * p <= n
    unless is_composite[p]
      (p * p).step(n, p) { |m| is_composite[m] = true }
    end
    p += 1
  end
  count = 0
  (2..n).each { |i| count += 1 unless is_composite[i] }
  puts count
end

n =200000
sieve(n)
