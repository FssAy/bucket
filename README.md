# Bucket
Library made for fun that offers a very simple container for any value without mutation.

*Beware, it bites.*


# Example
See `src/tests.rs` for more examples.

```rust
use bucket::Bucket;

fn main() {
    let my_val: Vec<u32> = vec![1, 2, 3];

    // Creates new Bucket and takes the ownership of `my_val`.
    let bucket = Bucket::new(my_val);

    // Increases every number from the vector by 1.
    // Mutation of the Bucket isn't needed.
    for number in bucket.peek_mut().unwrap() {
        *number += 1;
    }

    // Returns content of the Bucket.
    let _ = bucket.vacate().unwrap();

    // Now the Bucket can be filled with other u32 vector.
    bucket.fill(vec![4, 4, 4]);
}
```
