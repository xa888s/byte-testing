fn main() {
    let good_pointer = &10 as *const i32;

    // whatever address 10 from above is at
    dbg!(good_pointer);

    // this pointer is safe as we know it hasn't been dropped and it was just
    // created
    unsafe {
        println!("{}", *good_pointer);
    }

    // a i32 is just 4 i8s (kinda) so we can get the i8s out of it by using the
    // pointer to make a slice of i8s
    unsafe {
        // taking 4 i8s
        let slice: &[i8] = std::slice::from_raw_parts(good_pointer as *const i8, 4);
        // since 10 spans only the first i8 boundary, the last 3 will be 0 as the
        // bytes are 0
        assert_eq!(slice, &[10, 0, 0, 0]);

        // bit layout (little endian time 😎)
        // 10000000 00000000 00000000 00000000
        let slice: &[i8] = &[0, 0, 0, i8::MIN];
        let smallest_number: i32 = *(slice.as_ptr() as *const i32);
        assert_eq!(smallest_number, i32::MIN);
        println!("{:b}", i8::MIN);
    }

    let bad_pointer: *const i32 = 0x0 as *const i32;
    dbg!(bad_pointer);

    // dereferencing this pointer would cause a seg fault
    //unsafe {
    //    println!("{}", *bad_pointer);
    //}
}
