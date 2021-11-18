use array_macro::array;

pub trait SafeZip<T: Copy, const N: usize> {
    fn safe_zip<U: Copy> (self, rhs: [U;N]) -> [(T,U); N];
}

impl<T: Copy, const N: usize> SafeZip<T,N> for [T; N] {
    fn safe_zip<U: Copy> (self, rhs: [U;N]) -> [(T,U); N] {
        array![i => (self[i], rhs[i]); N]
    }
}