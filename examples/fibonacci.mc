define main:
  $fib = call fib 35
  out $fib
end

define fib($n):
  $a = lt $n 2
  jmpt $a L0
  $1 = sub $n 1
  $1 = call fib $1
  $2 = sub $n 2
  $2 = call fib $2
  $3 = add $1 $2
  $r = call fib $3
  ret $r
L0:
  ret $n
end
