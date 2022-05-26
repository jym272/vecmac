
#[macro_export]
//Declarative macro
//don't use the vec! macro
macro_rules! avec {
                    //0 or 1 of comma at the end --> trailing comma
    ($($e:expr),* $(,)?)=>({
        #[allow(unused_mut)] //for test empty_vec
        let mut vs=Vec::new();
        $(
            vs.push($e);
        )*
        vs
    });
    ($e:expr;$n:expr)=>({
        let mut vs=Vec::new();
        let pass_val = $e;
        for _ in 0..$n{
            vs.push(pass_val);
        }
        vs
    });
    ($($e:expr),*;$n:expr)=>({
        let mut vs=Vec::new();
        let x = [$($e),*];
        for _ in 0..$n{
            for i in x.iter(){
                vs.push(*i);
            }
        }
        vs
    });
}



#[test]
fn empty_vec() {
    let x: Vec<u32> = avec![];
    assert!(x.is_empty());
}

#[test]
fn single() {
    let x: Vec<u32> = avec![42];
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 42);
}

#[test]
fn double() {
    let x: Vec<u32> = avec![42,43];
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 43);
}
#[test]
fn three_of_the_same() {
    let x: Vec<u32> = avec![42;3];
    assert_eq!(x.len(), 3);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 42);
    assert_eq!(x[2], 42);
}
#[test]
fn three_of_the_same_no_literal() {
    let mut x:Option<u32>= Some(42);
    let y: Vec<u32> = avec![x.take().unwrap();3];
    assert_eq!(y.len(), 3);
}
#[test]
fn group_of_three() {
    let x: Vec<u32> = avec![42,43,44;3];
    assert_eq!(x.len(), 9);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 43);
    assert_eq!(x[2], 44);
    assert_eq!(x[3], 42);
    assert_eq!(x[4], 43);
    assert_eq!(x[5], 44);
    assert_eq!(x[6], 42);
    assert_eq!(x[7], 43);
    assert_eq!(x[8], 44);
}

#[test]
fn trailing() {
    let strs = avec!["foo in the garden kitchen with a lot of dolls to celebrate the init",
                "bar for all is inclusive and ulterior to the life on mars that we can get",
        "baz for all",
    ];
    assert_eq!(strs.len(), 3);
}
// Failure to use the trailing comma is a compile-time error.
/// ```compile_fail
/// let strs_ = vecmac::avec![42;"foo"];
/// ```
#[allow(dead_code)]
struct CompileFail;





//
// trait MaxValue{
//     fn max_value()->Self;
// }
//
// macro_rules! max_impl{
//     ($t:ty) => (
//         impl $crate::MaxValue for $t {
//             fn max_value() -> $t {
//                 <$t>::MAX
//             }
//         }
//     )
// }
// max_impl!(i32);
// max_impl!(i64);
// max_impl!(isize);

