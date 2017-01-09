# rayon_macros
Utility Macros for Rayon Library in Rust

#Examples
```rust
extern crate rayon;
#[macro_use]
extern crate rayon_macros;
fn main() {
  let mut a = false;
  let mut b = false;
  let mut c = false;
  rayon_par!(|| a = true, || b = true, || c = true);
  assert!(a && b && c);
}
```
