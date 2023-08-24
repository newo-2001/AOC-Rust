pub trait Pipe {
    fn pipe<O>(self, dest: impl FnOnce(Self) -> O) -> O where Self: Sized;
}

impl<T: Sized> Pipe for T {
    fn pipe<O>(self, dest: impl FnOnce(Self) -> O) -> O {
        dest(self)
    }
}