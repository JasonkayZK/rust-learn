use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Another implementation of Arc
/// According to https://nomicon.purewhite.io/arc-mutex/arc.html
pub struct MyArc<T> {
    // For variant case: can't use MyArc<&'a str> as MyArc<&'static str>
    ptr: NonNull<MyArcInner<T>>,
    // Declare ownership for sharing collection
    phantom: PhantomData<MyArcInner<T>>,
}

pub struct MyArcInner<T> {
    // Sharing counter
    rc: AtomicUsize,
    data: T,
}

impl<T> MyArc<T> {
    pub fn new(data: T) -> Self {
        let boxed = Box::new(MyArcInner {
            rc: AtomicUsize::new(1), // Add Sharing Counter to 1
            data,
        });
        MyArc {
            ptr: NonNull::new(Box::into_raw(boxed)).unwrap(), // Take raw pointer from box
            phantom: PhantomData,
        }
    }
}

// Limit "T: Sync + Send" is needed, or "Arc<Rc<u32>>" can still clone Rc to share in multiple threads!
unsafe impl<T: Sync + Send> Send for MyArc<T> {}
unsafe impl<T: Sync + Send> Sync for MyArc<T> {}

impl<T> Deref for MyArc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let inner = unsafe { self.ptr.as_ref() };
        &inner.data
    }
}

impl<T> Clone for MyArc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.ptr.as_ref() };

        // Using a relaxed ordering is alright here as we don't need any atomic
        // synchronization here as we're not modifying or accessing the inner data.
        let old_rc = inner.rc.fetch_add(1, Ordering::Relaxed);

        if old_rc >= isize::MAX as usize {
            std::process::abort();
        }

        Self {
            ptr: self.ptr,
            phantom: PhantomData,
        }
    }
}

impl<T> Drop for MyArc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.ptr.as_ref() };

        // Not the last reference
        if inner.rc.fetch_sub(1, Ordering::Release) != 1 {
            return;
        }

        // This fence is needed to prevent reordering of the use and deletion of the data
        atomic::fence(Ordering::Acquire);

        // This is safe as we know we have the last pointer to the `ArcInner`
        // and that its pointer is valid.
        unsafe {
            // Ignore return value to drop!
            Box::from_raw(self.ptr.as_ptr());
        }
    }
}
