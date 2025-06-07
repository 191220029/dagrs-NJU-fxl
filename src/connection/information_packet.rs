use std::{any::Any, sync::Arc};

/// Container type to store task output.
#[derive(Debug, Clone)]
pub struct Content {
    pub inner: Arc<dyn Any + Send + Sync>,
}

impl Content {
    /// Construct a new [`Content`].
    pub fn new<H: Send + Sync + 'static>(val: H) -> Self {
        Self {
            inner: Arc::new(val),
        }
    }

    pub fn from_arc<H: Send + Sync + 'static>(val: Arc<H>) -> Self {
        Self { inner: val }
    }

    pub fn get<H: 'static>(&self) -> Option<&H> {
        self.inner.downcast_ref::<H>()
    }

    pub fn into_inner<H: Send + Sync + 'static>(self) -> Option<Arc<H>> {
        self.inner.downcast::<H>().ok()
    }
}

/// Container type to store task output with specific type.
#[derive(Debug)]
pub struct TypedContent<T: Send + Sync + 'static> {
    pub inner: Arc<T>,
}

impl<T: Send + Sync + 'static> Clone for TypedContent<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: Send + Sync + 'static> TypedContent<T> {
    /// Construct a new [`TypedContent`] with specific type.
    pub fn new(val: T) -> Self {
        Self {
            inner: Arc::new(val),
        }
    }

    /// Create a new [`TypedContent`] from an existing Arc.
    pub fn from_arc(val: Arc<T>) -> Self {
        Self { inner: val }
    }

    /// Get a reference to the inner value.
    pub fn get(&self) -> &T {
        &self.inner
    }

    /// Convert into the inner Arc.
    pub fn into_inner(&self) -> Arc<T> {
        self.inner.clone()
    }
}
