proc main() =
  const n = 80
  var a = 0; var b = 1
  for _ in 0..<n:
    let tmp = a + b
    a = b
    b = tmp
  echo a

main()