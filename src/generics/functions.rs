struct A;
struct S(A);
struct SGen<T>(T);

fn reg_fn(_s: S) {}
fn gen_spec_t(_s: SGen<A>) {}
fn gen_spec_i32(_s: SGen<i32>) {}
// `SGen<T>` is preceded by `<T>`, so this function is generic over `T`
fn generic<T>(_s: SGen<T>) {}

pub(crate) fn main() {
    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));
    generic(SGen('c'));
    generic::<char>(SGen('a'));
}
