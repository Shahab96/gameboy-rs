pub trait Storage<T, U> {
    fn read(&mut self, src: T) -> U;
    fn write(&mut self, dest: T, value: U);
}
