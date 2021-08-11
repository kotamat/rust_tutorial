use std::slice;

fn main() {
    raw_pointer();
    safe_over_unsafe();
    external_code();
}

fn raw_pointer() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2)
    }
}
fn safe_over_unsafe() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    // let (a, b) = r.split_at_mut(3);
    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // 下記コードは実行時にSIGSEGVエラーになる
    // let address = 0x01234usize;
    // let r = address as *mut i32;
    // let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    // println!("{:?}", slice)
}

// fn split_at_mut_with_error(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();
//
//     assert!(mid <= len);
//
//     (&mut slice[..mid], &mut slice[mid..])
// }
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}
fn external_code() {
    unsafe { println!("Absolute value of -3 according to C: {}", abs(3)) }
}
