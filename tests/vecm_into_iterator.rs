//! Test IntoIterator implementations for VecM

#![cfg(all(
    any(feature = "curr", feature = "next"),
    not(all(feature = "curr", feature = "next"))
))]
#![cfg(feature = "std")]

#[cfg(feature = "curr")]
use stellar_xdr::curr as stellar_xdr;
#[cfg(feature = "next")]
use stellar_xdr::next as stellar_xdr;

use stellar_xdr::VecM;

#[test]
fn vecm_into_iterator_by_value() {
    // Test that we can iterate over a VecM by value (consuming it)
    let vecm: VecM<i32, 5> = vec![1, 2, 3, 4, 5].try_into().unwrap();
    let mut result = Vec::new();
    
    // This should work without explicitly calling .iter() or .into_iter()
    for item in vecm {
        result.push(item);
    }
    
    assert_eq!(result, vec![1, 2, 3, 4, 5]);
}

#[test]
fn vecm_into_iterator_by_reference() {
    // Test that we can iterate over a VecM by reference
    let vecm: VecM<i32, 5> = vec![1, 2, 3, 4, 5].try_into().unwrap();
    let mut result = Vec::new();
    
    // This should work without explicitly calling .iter()
    for item in &vecm {
        result.push(*item);
    }
    
    assert_eq!(result, vec![1, 2, 3, 4, 5]);
    
    // VecM should still be usable after the reference iteration
    assert_eq!(vecm.len(), 5);
}

#[test]
fn vecm_into_iterator_by_mutable_reference() {
    // Test that we can iterate over a VecM by mutable reference
    let mut vecm: VecM<i32, 5> = vec![1, 2, 3, 4, 5].try_into().unwrap();
    
    // This should work without explicitly calling .iter_mut()
    for item in &mut vecm {
        *item += 10;
    }
    
    assert_eq!(vecm.as_slice(), &[11, 12, 13, 14, 15]);
}

#[test]
fn vecm_into_iterator_empty() {
    // Test iterating over an empty VecM
    let vecm: VecM<i32, 5> = vec![].try_into().unwrap();
    let mut result = Vec::new();
    
    for item in vecm {
        result.push(item);
    }
    
    assert_eq!(result, vec![]);
}

#[test]
fn vecm_into_iterator_with_different_types() {
    // Test with strings
    let vecm: VecM<String, 3> = vec!["hello".to_string(), "world".to_string()].try_into().unwrap();
    let mut result = Vec::new();
    
    for item in &vecm {
        result.push(item.len());
    }
    
    assert_eq!(result, vec![5, 5]); // "hello" and "world" are both 5 chars
}

#[test]
fn vecm_into_iterator_in_for_loop_equivalent_to_explicit_iter() {
    let vecm: VecM<i32, 10> = vec![10, 20, 30].try_into().unwrap();
    
    // Test that the for-in loop gives same result as explicit .iter()
    let mut result_for_in = Vec::new();
    let mut result_explicit = Vec::new();
    
    // Using explicit iter() on a copy  
    let vecm_copy = vecm.clone();
    for item in vecm_copy.iter() {
        result_explicit.push(*item);
    }
    
    // Using for-in loop on reference
    for item in &vecm {
        result_for_in.push(*item);
    }
    
    assert_eq!(result_for_in, result_explicit);
    assert_eq!(result_for_in, vec![10, 20, 30]);
}