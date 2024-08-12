//Based on: https://whenderson.dev/blog/rust-mutexes/

use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering};
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct MutexGuard<'a, T> {
    lock: &'a Mutex<T>
}

impl <T> Deref for MutexGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.lock.value.get() }
    }
}

impl <T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.value.get() }
    } 
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.release();
    } 
}

#[derive(Debug)]
pub struct Mutex<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>
    //TODO: add cpu field 
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

impl <T> Mutex<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value)
        }
    }
    
    pub fn acquire(&self) -> MutexGuard<T> {
        while self
            .locked
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err() {}
        MutexGuard { lock: self }
    }

    pub fn try_acquire(&self) -> Result<MutexGuard<'_, T>, ()> {
        if self
            .locked
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            return Err(());
        }
        return Ok(MutexGuard { lock: self});
    }

    pub fn release(&self) {
        //TODO: Are we the ones holding the lock?
        self.locked.store(false, Ordering::Release);
    }
}
