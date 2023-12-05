pub struct Task(std::pin::Pin<Box<dyn std::future::Future<Output = ()>>>);

impl Task {
    pub fn new(future: impl std::future::Future<Output = ()> + 'static) -> Self {
        Self(Box::pin(future))
    }

    pub fn poll(&mut self, waker: &std::task::Waker) -> std::task::Poll<()> {
        self.0
            .as_mut()
            .poll(&mut std::task::Context::from_waker(waker))
    }
}
