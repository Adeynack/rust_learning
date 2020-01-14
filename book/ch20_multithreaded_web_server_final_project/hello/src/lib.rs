pub struct ThreadPool {
    size: usize,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        ThreadPool { size }
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static
    {

    }
}
