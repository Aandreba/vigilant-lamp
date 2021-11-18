pub trait ArrayBuilder<T, const N: usize> {
    fn build<F> (f: F) -> [T;N] where F: Fn(usize) -> T;
    fn build2<F, const R: usize> (f: F) -> [[T;N];R] where F: Fn(usize) -> [T;N];
}

impl<T: Copy, const N: usize> ArrayBuilder<T,N> for [T;N] {
    fn build<F>(f: F) -> [T;N] where F: Fn(usize) -> T {
        let first_val = f(0);
        let mut array : [T;N] = [first_val; N];

        for i in 1..N {
            array[i] = f(i);
        }

        return array;
    }

    fn build2<F, const R: usize> (f: F) -> [[T;N];R] where F: Fn(usize) -> [T;N] {
        let first_val = f(0);
        let mut array : [[T;N];R] = [first_val; R];

        for i in 1..R {
            array[i] = f(i);
        }

        return array;
    }
}