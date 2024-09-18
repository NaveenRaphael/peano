#![recursion_limit = "256"]
struct Zero;
trait W {
    fn get() -> i32;
}
impl W for Zero {
    fn get() -> i32 {
        0
    }
}
struct S<A>(A);

impl<A: W> W for S<A> {
    fn get() -> i32 {
        A::get() + 1
    }
}

type One = S<Zero>;
type Two = S<One>;
type Three = S<Two>;
type Four = S<Three>;

trait BPlus: W {
    type Plus<T: BPlus>: BPlus;
}

impl BPlus for Zero {
    type Plus<T: BPlus> = T;
}

impl<A> BPlus for S<A>
where
    A: BPlus,
{
    type Plus<T: BPlus> = S<A::Plus<T>>;
}

trait OP {
    type Result: W;
}

struct Add<A, B>(A, B);
impl<A, B> OP for Add<A, B>
where
    A: BPlus,
    B: BPlus,
{
    type Result = <A as BPlus>::Plus<B>;
}

trait BMul: BPlus {
    type Mul<T: BPlus>: BPlus;
}

impl BMul for Zero {
    type Mul<T: BPlus> = Zero;
}

impl<A> BMul for S<A>
where
    A: BMul,
{
    type Mul<T: BPlus> = <Add<A::Mul<T>, T> as OP>::Result;
}

struct Multiply<A, B>(A, B);
impl<A, B> OP for Multiply<A, B>
where
    A: BMul,
    B: BMul,
{
    type Result = <A as BMul>::Mul<B>;
}

fn main() {
    type Sixteen = <Multiply<Four, Four> as OP>::Result;
    let a: <Multiply<Four, Four> as OP>::Result = 1;
}
