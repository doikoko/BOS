use memory::mem64::UnsafeArr;

#[test]
fn unsafe_arr_test(){
    let mut arr: UnsafeArr<u8> = UnsafeArr::new(3);
    unsafe { *(arr.0) = 2; } 
   // println!("{}", arr[0]);
    assert_eq!(arr[0], 2);
}