#[macro_export]
macro_rules! avec{
    ()=>{
        Vec::new()
    }
}

#[test]
fn empty_vec(){
    let x:Vec<u32> = avec![];
    assert!(x.is_empty());
}