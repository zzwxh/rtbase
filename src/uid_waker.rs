pub struct UidWaker<T: UidWake>(std::marker::PhantomData<T>);

pub trait UidWake {
    fn wake(id: usize);
}

impl<T: UidWake> UidWaker<T> {
    pub fn create(id: usize) -> std::task::Waker {
        unsafe { std::task::Waker::from_raw(std::task::RawWaker::new(id as _, &Self::VT)) }
    }

    const VT: std::task::RawWakerVTable = std::task::RawWakerVTable::new(
        |id| std::task::RawWaker::new(id, &Self::VT),
        |id| T::wake(id as _),
        |id| T::wake(id as _),
        |_| {},
    );
}
