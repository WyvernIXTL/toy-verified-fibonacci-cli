#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]
#![cfg_attr(any(), rustfmt::skip)]

pub mod _module {
    pub use ::dafny_runtime::_System::nat;
    pub use ::dafny_runtime::int;
    pub use ::dafny_runtime::integer_range;
    pub use ::dafny_runtime::Sequence;
    pub use ::dafny_runtime::DafnyChar;
    pub use ::std::rc::Rc;
    pub use ::dafny_runtime::DafnyInt;
    pub use crate::_module::Option::None;
    pub use ::std::primitive::char;
    pub use ::dafny_runtime::DafnyPrintWrapper;
    pub use ::dafny_runtime::string_of;
    pub use ::dafny_runtime::DafnyType;
    pub use ::std::fmt::Debug;
    pub use ::std::fmt::Formatter;
    pub use ::std::fmt::Result;
    pub use ::dafny_runtime::DafnyPrint;
    pub use ::std::cmp::Eq;
    pub use ::std::hash::Hash;
    pub use ::std::cmp::PartialEq;
    pub use ::std::hash::Hasher;
    pub use ::std::convert::AsRef;

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
        /// src/main.dfy(27,1)
        pub fn NatFromString(s: &Sequence<DafnyChar>) -> Rc<Option<nat>> {
            if s.cardinality() == int!(0) {
                Rc::new(Option::Some::<DafnyInt> {
                        a: int!(0)
                    })
            } else {
                let mut tail: Rc<Option<nat>> = _default::NatFromString(&s.take(&(s.cardinality() - int!(1))));
                let mut _source0: Rc<Option<nat>> = tail.clone();
                if matches!((&_source0).as_ref(), None{ .. }) {
                    Rc::new(Option::None::<DafnyInt> {})
                } else {
                    let mut ___mcc_h0: nat = _source0.a().clone();
                    let mut _n_k: nat = ___mcc_h0.clone();
                    if s.get(&(s.cardinality() - int!(1))) == DafnyChar(char::from_u32(48).unwrap()) {
                        Rc::new(Option::Some::<DafnyInt> {
                                a: _n_k.clone() * int!(10) + int!(0)
                            })
                    } else {
                        if s.get(&(s.cardinality() - int!(1))) == DafnyChar(char::from_u32(49).unwrap()) {
                            Rc::new(Option::Some::<DafnyInt> {
                                    a: _n_k.clone() * int!(10) + int!(1)
                                })
                        } else {
                            if s.get(&(s.cardinality() - int!(1))) == DafnyChar(char::from_u32(50).unwrap()) {
                                Rc::new(Option::Some::<DafnyInt> {
                                        a: _n_k.clone() * int!(10) + int!(2)
                                    })
                            } else {
                                if s.get(&(s.cardinality() - int!(1))) == DafnyChar(char::from_u32(51).unwrap()) {
                                    Rc::new(Option::Some::<DafnyInt> {
                                            a: _n_k.clone() * int!(10) + int!(3)
                                        })
                                } else {
                                    if s.get(&(s.cardinality() - int!(1))) == DafnyChar(char::from_u32(52).unwrap()) {
                                        Rc::new(Option::Some::<DafnyInt> {
                                                a: _n_k.clone() * int!(10) + int!(4)
                                            })
                                    } else {
                                        if s.get(&(s.cardinality() - int!(1))) == DafnyChar(char::from_u32(53).unwrap()) {
                                            Rc::new(Option::Some::<DafnyInt> {
                                                    a: _n_k.clone() * int!(10) + int!(5)
                                                })
                                        } else {
                                            if s.get(&(s.cardinality() - int!(1))) == DafnyChar(char::from_u32(54).unwrap()) {
                                                Rc::new(Option::Some::<DafnyInt> {
                                                        a: _n_k.clone() * int!(10) + int!(6)
                                                    })
                                            } else {
                                                if s.get(&(s.cardinality() - int!(1))) == DafnyChar(char::from_u32(55).unwrap()) {
                                                    Rc::new(Option::Some::<DafnyInt> {
                                                            a: _n_k.clone() * int!(10) + int!(7)
                                                        })
                                                } else {
                                                    if s.get(&(s.cardinality() - int!(1))) == DafnyChar(char::from_u32(56).unwrap()) {
                                                        Rc::new(Option::Some::<DafnyInt> {
                                                                a: _n_k.clone() * int!(10) + int!(8)
                                                            })
                                                    } else {
                                                        if s.get(&(s.cardinality() - int!(1))) == DafnyChar(char::from_u32(57).unwrap()) {
                                                            Rc::new(Option::Some::<DafnyInt> {
                                                                    a: _n_k.clone() * int!(10) + int!(9)
                                                                })
                                                        } else {
                                                            Rc::new(Option::None::<DafnyInt> {})
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        /// src/main.dfy(51,1)
        pub fn Main(args: &Sequence<Sequence<DafnyChar>>) -> () {
            if args.cardinality() != int!(2) {
                print!("{}", DafnyPrintWrapper(&string_of("Verified Fibonacci Solver by Adam McKellar\n")));
                print!("{}", DafnyPrintWrapper(&string_of("USAGE: verified-fibonacci <n>\n")));
                return ();
            };
            let mut n_opt: Rc<Option<nat>> = _default::NatFromString(&args.get(&int!(1)));
            let mut _source0: Rc<Option<nat>> = n_opt.clone();
            if matches!((&_source0).as_ref(), None{ .. }) {
                print!("{}", DafnyPrintWrapper(&string_of("Invalid Input\n")))
            } else {
                let mut ___mcc_h0: nat = _source0.a().clone();
                let mut n: nat = ___mcc_h0.clone();
                let mut r: nat;
                let mut _out0: nat = _default::FibLin(&n);
                r = _out0.clone();
                print!("{}", DafnyPrintWrapper(&r));
                print!("{}", DafnyPrintWrapper(&string_of("\n")))
            };
            return ();
        }
    }

    /// src/main.dfy(25,1)
    #[derive(Clone)]
    pub enum Option<A: DafnyType> {
        None {},
        Some {
            a: A
        }
    }

    impl<A: DafnyType> Option<A> {
        /// Gets the field a for all enum members which have it
        pub fn a(&self) -> &A {
            match self {
                Option::None{} => panic!("field does not exist on this variant"),
                Option::Some{a, } => a,
            }
        }
    }

    impl<A: DafnyType> Debug
        for Option<A> {
        fn fmt(&self, f: &mut Formatter) -> Result {
            DafnyPrint::fmt_print(self, f, true)
        }
    }

    impl<A: DafnyType> DafnyPrint
        for Option<A> {
        fn fmt_print(&self, _formatter: &mut Formatter, _in_seq: bool) -> std::fmt::Result {
            match self {
                Option::None{} => {
                    write!(_formatter, "Option.None")?;
                    Ok(())
                },
                Option::Some{a, } => {
                    write!(_formatter, "Option.Some(")?;
                    DafnyPrint::fmt_print(a, _formatter, false)?;
                    write!(_formatter, ")")?;
                    Ok(())
                },
            }
        }
    }

    impl<A: DafnyType + Eq + Hash> PartialEq
        for Option<A> {
        fn eq(&self, other: &Self) -> bool {
            match (
                    self,
                    other
                ) {
                (Option::None{}, Option::None{}) => {
                    true
                },
                (Option::Some{a, }, Option::Some{a: _2_a, }) => {
                    a == _2_a
                },
                _ => {
                    false
                },
            }
        }
    }

    impl<A: DafnyType + Eq + Hash> Eq
        for Option<A> {}

    impl<A: DafnyType + Hash> Hash
        for Option<A> {
        fn hash<_H: Hasher>(&self, _state: &mut _H) {
            match self {
                Option::None{} => {
                    
                },
                Option::Some{a, } => {
                    Hash::hash(a, _state)
                },
            }
        }
    }

    impl<A: DafnyType> AsRef<Option<A>>
        for Option<A> {
        fn as_ref(&self) -> &Self {
            self
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