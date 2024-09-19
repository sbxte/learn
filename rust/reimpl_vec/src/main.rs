use std::alloc::{self, Layout};
use std::ptr::NonNull;

struct MyVec<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
    growth_multiplier: usize,
}

impl<T> MyVec<T> {
    fn new_empty() -> Self {
        MyVec {
            ptr: NonNull::dangling(),
            cap: 0,
            len: 0,
            growth_multiplier: 2,
        }
    }

    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            // This can't overflow since self.cap <= isize::MAX.
            let new_cap = self.growth_multiplier * self.cap;

            // `Layout::array` checks that the number of bytes is <= usize::MAX,
            // but this is redundant since old_layout.size() <= isize::MAX,
            // so the `unwrap` should never fail.
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        // Ensure that the new allocation doesn't exceed `isize::MAX` bytes.
        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Allocation too large"
        );

        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        // If allocation fails, `new_ptr` will be null, in which case we abort.
        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }

    fn push(&mut self, element: T) {
        // Grow if we don't have enough space
        if self.len >= self.cap {
            self.grow();
        }

        unsafe { self.ptr.offset(self.len as isize).write(element) }
        self.len += 1;
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        if self.cap == 0 {
            return;
        }

        unsafe {
            alloc::dealloc(
                self.ptr.as_ptr() as *mut u8,
                Layout::array::<T>(self.cap).unwrap(),
            )
        }
    }
}

impl<T> std::fmt::Debug for MyVec<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut vec_string = "".to_string();

        if self.len > 0 {
            vec_string.push_str(format!("[{:?}", unsafe { self.ptr.read() }).as_str());
            for i in 1..(self.len as isize) {
                vec_string
                    .push_str(format!(", {:?}", unsafe { self.ptr.offset(i).read() }).as_str());
            }
            vec_string.push(']');
        }
        f.debug_struct("MyVec")
            .field("vec", &vec_string)
            .field("ptr", &self.ptr)
            .field("cap", &self.cap)
            .field("len", &self.len)
            .field("growth_multiplier", &self.growth_multiplier)
            .finish()
    }
}

unsafe impl<T> Send for MyVec<T> {}
unsafe impl<T> Sync for MyVec<T> {}

macro_rules! myvec {
    () => {
        MyVec::new_empty()
    };
}

fn main() {
    let mut x = myvec![];
    dbg!(&x);

    // Test for pushes
    for i in 0..10 {
        x.push(i);
    }
    dbg!(&x);
}
