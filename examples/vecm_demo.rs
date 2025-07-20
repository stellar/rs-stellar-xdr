#[cfg(all(feature = "curr", feature = "std"))]
fn main() {
    use stellar_xdr::curr::VecM;
    
    println!("Testing VecM IntoIterator implementation");
    
    // Create a VecM with some data
    let my_vecm: VecM<i32, 10> = vec![1, 2, 3, 4, 5].try_into().unwrap();
    
    println!("Original issue was that this wouldn't compile:");
    println!("for item in my_vecm {{ ... }}");
    println!();
    
    println!("Testing iteration by value (consuming the VecM):");
    let my_vecm_copy = my_vecm.clone();
    print!("Values: ");
    for item in my_vecm_copy {
        print!("{} ", item);
    }
    println!();
    
    println!("Testing iteration by reference (non-consuming):");
    print!("Values: ");
    for item in &my_vecm {
        print!("{} ", item);
    }
    println!();
    
    // VecM should still be usable after reference iteration
    println!("VecM is still usable: len = {}", my_vecm.len());
    
    println!("Testing iteration by mutable reference:");
    let mut my_vecm_mut = my_vecm.clone();
    for item in &mut my_vecm_mut {
        *item += 10;
    }
    print!("Modified values: ");
    for item in &my_vecm_mut {
        print!("{} ", item);
    }
    println!();
    
    println!("All tests passed! The issue has been resolved.");
}

#[cfg(not(all(feature = "curr", feature = "std")))]
fn main() {
    println!("This demo requires features: curr, std");
}