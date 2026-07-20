

function FibSpec(n: nat): nat
  decreases n
{
  if n < 2 then n else FibSpec(n-1) + FibSpec(n-2)
}

method FibLin(n: nat) returns (a: nat)
  ensures a == FibSpec(n)
{
  a := 0;
  var b := 1;

  for i := 0 to n
    invariant a == FibSpec(i)
    invariant i == 0 ==> b == 1
    invariant i >= 1 ==> b == FibSpec(i-1)
  {
    a, b := b, a;
    a := a + b;
  }
}

method Main() {
  print "Hello World!";
}
