use std::ops::Deref;
use crate::Bucket;


#[test]
fn construction() {
    let bucket = Bucket::new(1);

    assert!(!bucket.is_empty());  // Bucket should not be empty
}

#[test]
fn default_construction() {
    let bucket = Bucket::<i32>::default();

    assert!(bucket.is_empty());  // Bucket should be empty
}

#[test]
fn empty_exception() {
    let bucket_zero = Bucket::new(0);
    let bucket_unit = Bucket::new(());
    let bucket_null = Bucket::new(std::ptr::null() as *const u8);

    // these buckets are filled, but should be marked as empty
    assert!(bucket_zero.is_empty());
    assert!(bucket_unit.is_empty());
    assert!(bucket_null.is_empty());
}

#[test]
fn vacating_primitive() {
    let bucket = Bucket::new(5);
    let data = bucket.vacate();

    assert!(data.is_some());  // bucket should spill the value with "vacate"
    assert_eq!(data.unwrap(), 5);  // data should be valid
}

#[test]
fn vacating_non_primitive() {
    let bucket = Bucket::new(Box::new(5));
    let data = bucket.vacate();

    assert!(data.is_some());  // bucket should spill the value with "vacate"
    assert_eq!(data.unwrap().deref(), &5);  // data should be valid
}

#[test]
fn filling() {
    let bucket = Bucket::default();
    bucket.fill(Box::new(5));

    assert!(!bucket.is_empty());  // bucket should not be empty after performing a fill
    assert!(bucket.fill(Box::new(10)).is_some());  // bucket is filled, so a "new fill value" should be returned
    assert!(bucket.vacate().is_some());
    assert!(bucket.fill(Box::new(10)).is_none());  // after "vacate" the bucket should be empty and allow filling
}

#[test]
fn peek_ref() {
    let bucket = Bucket::new(5);

    assert_eq!(bucket.peek_ref().unwrap(), &5);  // bucket's content should be valid
}

#[test]
fn peek_ref_mut() {
    let bucket = Bucket::new(5);
    let peek = bucket.peek_mut().unwrap();

    assert_eq!(peek, &mut 5);  // bucket's content should be valid
    *peek = 6;
    assert_eq!(bucket.peek_mut().unwrap(), &mut 6);  // altered bucket's content should be valid
}
