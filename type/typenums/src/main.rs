use typenum::{Exp, Integer, Sum, N2, P3, P4};

fn main() {
    type X = Sum<P3, P4>;
    assert_eq!(<X as Integer>::to_i32(), 7);

    type Y = Exp<N2, P3>;
    assert_eq!(<Y as Integer>::to_i32(), -8);
}
