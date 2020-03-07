# Machina

A "high level" virtual machine


# Fizz Buzz example

´´´
define main
  $i = 0
  jmp LOOP
LOOP:
  call fizzbuzz $count
  $i = add $i 1
  $l = lte $i 100
  jmpt $l LOOP
  ret

define fizzbuzz($n)
  $a = mod $n 3
  $b = mod $n 5
  $az = eq $a 0
  $bz = eq $b 0
  $both = and $az $bz
  $none = eq $both 0
  $block = switch $both L0; $az L1; $bz L2; $none L3
  exec $block
  ret
L0:
  output "FizzBuzz"
L1:
  output "Fizz"
L2:
  output "Buzz"
L3:
  output $n
´´´
