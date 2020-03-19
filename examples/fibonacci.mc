main:
  const     35
  call      fib
  output

fib($n):
  load      $n
  const     2
  lt
  jumpt     .L0

  load      $n
  const     1
  sub
  call      fib

  load      $n
  const     2
  sub
  call      fib

  add
  call      fib
  return
.L0:
  load      $n
  return