pub trait Fn<Args, Ret> {
    fn call(&self, Args) -> Ret;
}

pub trait FnMut<Args, Ret> {
    fn call(&mut self, Args) -> Ret;
}

/*impl<'a, Ret> FnMut<(), Ret> for 'a || -> Ret {
    fn call(&mut self) {
        self();
    }
}*/
