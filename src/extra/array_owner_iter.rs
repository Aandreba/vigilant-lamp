struct ArrayOwnershipIterator<T, const N: usize> {
    parent: [T;N],
    pos: usize
}

impl<T, const N: usize> ArrayOwnershipIterator<T,N>  {
    // TODO
}