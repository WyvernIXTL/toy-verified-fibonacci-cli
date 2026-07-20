
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

datatype Option<A> = None | Some(a: A)

function NatFromString(s: string): Option<nat> {
  if |s| == 0 then
    Some(0)
  else
    var tail := NatFromString(s[..|s|-1]);
    match tail {
      case None => None
      case Some(n') =>
        match s[ |s| - 1] {
          case '0' => Some(n' * 10 + 0)
          case '1' => Some(n' * 10 + 1)
          case '2' => Some(n' * 10 + 2)
          case '3' => Some(n' * 10 + 3)
          case '4' => Some(n' * 10 + 4)
          case '5' => Some(n' * 10 + 5)
          case '6' => Some(n' * 10 + 6)
          case '7' => Some(n' * 10 + 7)
          case '8' => Some(n' * 10 + 8)
          case '9' => Some(n' * 10 + 9)
          case _ => None
        }
    }
}

method Main(args: seq<string>) {
  if |args| != 2 {
    print "Verified Fibonacci Solver by Adam McKellar\n";
    print "USAGE: verified-fibonacci <n>\n";
    return;
  }

  var n_opt := NatFromString(args[1]);
  match n_opt {
    case None => print "Invalid Input\n";
    case Some(n) =>
      var r := FibLin(n);
      print r;
      print "\n";
  }
}
