define main
  $n = call fib $20
  out $n
end

define fib($n)
  $a = lte $n 1
  jmpt $a L0
  $1 = n - 1
  $2 = n - 2
  $1 = call fib $1
  $2 = call fib $2
  $3 = add $1 $2
  $r = call fib $3
  ret $r
L0:
  ret 1
end