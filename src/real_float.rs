pub mod real_float {
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default, Debug)]
    pub struct RealFloat<T> {
        inner: T,
    }

    impl<T> RealFloat<T> {
        pub fn inner(self) -> T {
            self.inner
        }
    }

    // Define the Realable trait with a method that takes self
    pub trait Realable {
        fn to_real(self) -> Option<RealFloat<Self>>
        where
            Self: Sized;
    }

    // Implement the Realable trait for f64
    impl Realable for f64 {
        fn to_real(self) -> Option<RealFloat<f64>> {
            if !self.is_finite() {
                return None;
            }
            Some(RealFloat { inner: self })
        }
    }
}
