main:
  const     1
  store     $i
.L0:
  load      $i
  call      fizzbuzz
  output

  load      $i
  const     1
  add
  store     $i

  load      $i
  const     100
  lte
  jumpt     .L0

  return

fizzbuzz($n):
  store     $n

  load      $n
  const     3
  mod
  store     $a

  load      $n
  const     5
  mod
  store     $b

  load      $a
  const     0
  eq
  jumpf     .L0

  load      $b
  const     0
  eq
  jumpf     .L0

  const     "FizzBuzz"
  return

.L0:
  load      $a
  const     0
  eq
  jumpf     .L1

  const     "Fizz"
  return

.L1:
  load      $b
  const     0
  eq
  jumpf     .L2

  const     "Buzz"
  return

.L2:
  load      $n
  return