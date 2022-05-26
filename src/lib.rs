#[macro_export]
//Declarative macro
//don't use the vec! macro
macro_rules! avec {
                    //0 or 1 of comma at the end --> trailing comma
    ($($e:expr),*)=>({  //it allows comma between brackets: let z: Vec<u32> = avec![,];

        // const count:usize = $crate::avec!(@COUNT; $($e),*);
        const COUNT:usize = $crate::count_expr!($($e),*);
        #[allow(unused_mut)] //for test empty_vec
        let mut vs=Vec::with_capacity(COUNT);
        $(
            vs.push($e);
        )*
        vs
    });
    ($($e:expr,)* )=>({
        $crate::avec![$($e),*]
    });


    ($e:expr;$n:expr)=>({
        let count=$n;
        // let mut vs=Vec::with_capacity(count);
        let mut vs=Vec::new(); //for 3rd option only
        // 1st option:
        // let pass_val = $e;
         // for _ in 0..count{
        //     vs.push(pass_val); //push is O(1) is a pointer increment
        // }

        // 2nd option:
                  //root level path
        // vs.extend(::std::iter::repeat($e).take(count)); //clone elements and only take count times

        // 3rd option:
        vs.resize(count,$e); //in charge of clone elements and allocate memory
        vs
    });
    ($($e:expr),*;$n:expr)=>({
        let x = [$($e),*]; //only valuate the expression once
        let size=x.len();
        let count=$n;
        let mut vs=Vec::with_capacity(size*count);
          //1st option
        // for _ in 0..count{
        //     for i in x.iter(){
                // vs.push(*i);
            // }
        // }
        for _ in 0..count{
            vs.extend(x.iter().clone());
        }

        vs
    });



}
#[macro_export]
#[doc(hidden)]
macro_rules! count_expr {
     ($($e:expr),*)=>{
              //slice method len()
        <[()]>::len(&[$($crate::count_expr![@SUBS $e]),*])
          //()-> "unit" ->zero size type, no memory
    };
    (@SUBS $e:expr)=>{
        () //the expression is not consumed, "unit" is the substitution
    };
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
    let mut x: Option<u32> = Some(42);
    let y: Vec<u32> = avec![x.take().unwrap();3];
    assert_eq!(y.len(), 3);
}

#[test]
fn group_of_three() {
    let mut y: Option<u32> = Some(42);

    let x: Vec<u32> = avec![y.take().unwrap(),43,44;3];
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
