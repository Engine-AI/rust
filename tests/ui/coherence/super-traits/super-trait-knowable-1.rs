// Added in #124532. While `(): Super` is knowable, `(): Sub<?t>` is not.
//
// We therefore elaborate super trait bounds in the implicit negative
// overlap check.

//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[next] compile-flags: -Znext-solver
//@[next] check-pass

trait Super {}
trait Sub<T>: Super {}

trait Overlap<T> {}
impl<T, U: Sub<T>> Overlap<T> for U {}
impl<T> Overlap<T> for () {}
//[current]~^ ERROR conflicting implementations

fn main() {}
