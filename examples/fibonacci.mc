main:
  const     35
  call      fib
  output

fib($n):
  store     $n

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
  return
.L0:
  load      $n
  return