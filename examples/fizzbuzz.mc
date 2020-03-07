proc main
  $i = 0
  jmp LOOP
LOOP:
  call fizzbuzz $count
  $i = add $i 1
  $l = lte $i 100
  jmpt $l LOOP
end

proc fizzbuzz($n)
  $a = mod $n 3
  $b = mod $n 5
  $az = eq $a 0
  $bz = eq $b 0
  $both = and $az $bz
  $none = eq $both 0
  $block = case $both L0; $az L1; $bz L2; $none L3
  exec $block
L0:
  out "FizzBuzz"
L1:
  out "Fizz"
L2:
  out "Buzz"
L3:
  out $n
end
