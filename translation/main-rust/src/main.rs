#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]
#![cfg_attr(any(), rustfmt::skip)]

pub mod _module {
    pub use ::dafny_runtime::_System::nat;
    pub use ::dafny_runtime::int;
    pub use ::dafny_runtime::integer_range;
    pub use ::dafny_runtime::Sequence;
    pub use ::dafny_runtime::DafnyChar;
    pub use ::dafny_runtime::DafnyPrintWrapper;
    pub use ::dafny_runtime::string_of;

    pub struct _default {}

    impl _default {
        /// src/main.dfy(3,1)
        pub fn FibSpec(n: &nat) -> nat {
            if n.clone() < int!(2) {
                n.clone()
            } else {
                _default::FibSpec(&(n.clone() - int!(1))) + _default::FibSpec(&(n.clone() - int!(2)))
            }
        }
        /// src/main.dfy(9,1)
        pub fn FibLin(n: &nat) -> nat {
            let mut a: nat = int!(0);
            let mut b: nat = int!(1);
            let mut _hi0: nat = n.clone();
            for i in integer_range(int!(0), _hi0.clone()) {
                let mut _rhs0: nat = b.clone();
                let mut _rhs1: nat = a.clone();
                a = _rhs0.clone();
                b = _rhs1.clone();
                a = a.clone() + b.clone();
            }
            return a.clone();
        }
        /// src/main.dfy(25,1)
        pub fn Main(_noArgsParameter: &Sequence<Sequence<DafnyChar>>) -> () {
            print!("{}", DafnyPrintWrapper(&string_of("Hello World!")));
            return ();
        }
    }
}
fn main() {
  let args: Vec<String> = ::std::env::args().collect();
  let dafny_args =
    ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(
    &args, |s| 
  ::dafny_runtime::dafny_runtime_conversions::unicode_chars_true::string_to_dafny_string(s));
  crate::_module::_default::Main(&dafny_args);
}