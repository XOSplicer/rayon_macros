extern crate rayon;

/// Run several closures in parallel using rayon.
/// Join on all of them.
/// Rayon may decide not to do things in parallel, beware!
///
/// #Examples
/// ```
/// extern crate rayon;
/// #[macro_use]
/// extern crate rayon_macros;
/// fn main() {
///     let mut a = false;
///     let mut b = false;
///     let mut c = false;
///     rayon_par!(|| a = true, || b = true, || c = true);
///     assert!(a && b && c);
/// }
/// ```
#[macro_export]
macro_rules! rayon_par {
    ($a:expr) => (rayon::join($a, ||()));
    ($a:expr, $($r:expr),+) => (rayon::join($a, || rayon_par!($($r),*)));
}

#[cfg(test)]
mod tests {
    use rayon;

    #[test]
    fn rayon_single() {
        let mut a = false;
        rayon::join(|| a = true, ||());
        assert!(a);
    }

    #[test]
    fn rayon_simple() {
        let mut a = false;
        let mut b = false;
        rayon::join(|| a = true, || b = true);
        assert!(a && b);
    }

    #[test]
    fn rayon_nested() {
        let mut a = false;
        let mut b = false;
        let mut c = false;
        rayon::join(|| a = true,
                    || rayon::join(|| b = true,
                                   || c = true));
        assert!(a && b && c);
    }

    #[test]
    fn macro_single() {
        let mut a = false;
        rayon_par!(|| a = true);
        assert!(a);
    }

    #[test]
    fn macro_simple() {
        let mut a = false;
        let mut b = false;
        rayon_par!(|| a = true, || b = true);
        assert!(a && b);
    }

    #[test]
    fn macro_nested() {
        let mut a = false;
        let mut b = false;
        let mut c = false;
        let mut d = false;
        rayon_par!(|| a = true, || b = true, || c = true, || d = true);
        assert!(a && b && c && d);
    }

}
