# File: matmul/matmul.rb
# Triple‐loop matrix multiplication, n = 200

def bench_matmul(n)
  # 1) Build A and B as nested arrays
  a = Array.new(n) { Array.new(n) }
  (0...n).each do |i|
    (0...n).each do |j|
      a[i][j] = i * n + j
    end
  end

  b = Array.new(n) { Array.new(n) }
  (0...n).each do |i|
    (0...n).each do |j|
      b[i][j] = j * n + i
    end
  end

  # 2) Allocate C = zero matrix
  c = Array.new(n) { Array.new(n, 0) }

  # 3) Compute C = A × B
  (0...n).each do |i|
    (0...n).each do |j|
      sum = 0
      (0...n).each do |k|
        sum += a[i][k] * b[k][j]
      end
      c[i][j] = sum
    end
  end

  # 4) Print C[0][0]
  puts c[0][0]
  c[0][0]
end

def main
  n = 200
  bench_matmul(n)
end

main
