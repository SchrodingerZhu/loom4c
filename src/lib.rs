use core::ffi::c_void;
use loom::{
    sync::{atomic::Ordering, Mutex},
    thread::Thread,
};
use std::{collections::VecDeque, ops::Deref};

#[repr(C)]
pub enum SizeType {
    AtomicU8 = 8,
    AtomicU16 = 16,
    AtomicU32 = 32,
    AtomicU64 = 64,
}

#[repr(C)]
pub enum MemoryOrder {
    Relaxed = 0,
    Acquire = 1,
    Release = 2,
    AcqRel = 3,
    SeqCst = 4,
}

struct Atomic<T> {
    inner: T,
    queue: Mutex<VecDeque<Thread>>,
}

impl<T> Deref for Atomic<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.inner
    }
}

type AtomicU8 = Atomic<loom::sync::atomic::AtomicU8>;
type AtomicU16 = Atomic<loom::sync::atomic::AtomicU16>;
type AtomicU32 = Atomic<loom::sync::atomic::AtomicU32>;
type AtomicU64 = Atomic<loom::sync::atomic::AtomicU64>;

macro_rules! impl_atomic {
    ($ty:ty, $atm_ty:ident) => {
        impl $atm_ty {
            fn new(val: $ty) -> Self {
                Atomic {
                    inner: loom::sync::atomic::$atm_ty::new(val),
                    queue: Mutex::new(VecDeque::new()),
                }
            }
            fn wait(&self, val: $ty) {
                let mut queue = self.queue.lock().unwrap();
                if self.load(Ordering::SeqCst) != val {
                    return;
                }
                queue.push_back(loom::thread::current());
                drop(queue);
                loom::thread::park();
            }
            fn notify_one(&self) -> bool {
                let Some(notify) = self.queue.lock().unwrap().pop_front() else {
                    return false;
                };
                notify.unpark();
                true
            }
            fn notify_all(&self) -> usize {
                let mut queue = self.queue.lock().unwrap();
                let length = queue.len();
                while let Some(notify) = queue.pop_front() {
                    notify.unpark();
                }
                length
            }
        }
    };
}

impl_atomic!(u8, AtomicU8);
impl_atomic!(u16, AtomicU16);
impl_atomic!(u32, AtomicU32);
impl_atomic!(u64, AtomicU64);

impl From<MemoryOrder> for Ordering {
    fn from(ord: MemoryOrder) -> Self {
        match ord {
            MemoryOrder::Relaxed => Ordering::Relaxed,
            MemoryOrder::Acquire => Ordering::Acquire,
            MemoryOrder::Release => Ordering::Release,
            MemoryOrder::AcqRel => Ordering::AcqRel,
            MemoryOrder::SeqCst => Ordering::SeqCst,
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn loom_atomic_load(
    src: *const c_void,
    dst: *mut c_void,
    ty: SizeType,
    ord: MemoryOrder,
) {
    macro_rules! load {
        ($src: expr, $dst: expr, $atomty: ty, $ty: ty) => {
            let src = $src as *const $atomty;
            let dst = $dst as *mut $ty;
            *dst = (*src).load(ord.into());
        };
    }
    match ty {
        SizeType::AtomicU8 => {
            load!(src, dst, AtomicU8, u8);
        }
        SizeType::AtomicU16 => {
            load!(src, dst, AtomicU16, u16);
        }
        SizeType::AtomicU32 => {
            load!(src, dst, AtomicU32, u32);
        }
        SizeType::AtomicU64 => {
            load!(src, dst, AtomicU64, u64);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn loom_atomic_store(
    dst: *const c_void,
    val: *const c_void,
    ty: SizeType,
    ord: MemoryOrder,
) {
    macro_rules! store {
        ($src: expr, $val: expr, $atomty: ty, $ty: ty) => {
            let dst = $src as *const $atomty;
            let val = $val as *const $ty;
            (*dst).store(*val, ord.into());
        };
    }
    match ty {
        SizeType::AtomicU8 => {
            store!(dst, val, AtomicU8, u8);
        }
        SizeType::AtomicU16 => {
            store!(dst, val, AtomicU16, u16);
        }
        SizeType::AtomicU32 => {
            store!(dst, val, AtomicU32, u32);
        }
        SizeType::AtomicU64 => {
            store!(dst, val, AtomicU64, u64);
        }
    }
}

macro_rules! atomic_op {
    ($name:ident, $op:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(
            atomic: *const c_void,
            val: *const c_void,
            recv: *mut c_void,
            ty: SizeType,
            ord: MemoryOrder,
        ) {
            match ty {
                SizeType::AtomicU8 => {
                    let atomic = atomic as *const AtomicU8;
                    let val = val as *const u8;
                    let recv = recv as *mut u8;
                    *recv = (*atomic).$op(*val, ord.into());
                }
                SizeType::AtomicU16 => {
                    let atomic = atomic as *const AtomicU16;
                    let val = val as *const u16;
                    let recv = recv as *mut u16;
                    *recv = (*atomic).$op(*val, ord.into());
                }
                SizeType::AtomicU32 => {
                    let atomic = atomic as *const AtomicU32;
                    let val = val as *const u32;
                    let recv = recv as *mut u32;
                    *recv = (*atomic).$op(*val, ord.into());
                }
                SizeType::AtomicU64 => {
                    let atomic = atomic as *const AtomicU64;
                    let val = val as *const u64;
                    let recv = recv as *mut u64;
                    *recv = (*atomic).$op(*val, ord.into());
                }
            }
        }
    };
}

atomic_op!(loom_atomic_fetch_add, fetch_add);
atomic_op!(loom_atomic_fetch_sub, fetch_sub);
atomic_op!(loom_atomic_fetch_and, fetch_and);
atomic_op!(loom_atomic_fetch_nand, fetch_nand);
atomic_op!(loom_atomic_fetch_or, fetch_or);
atomic_op!(loom_atomic_fetch_xor, fetch_xor);
atomic_op!(loom_atomic_fetch_max, fetch_max);
atomic_op!(loom_atomic_fetch_min, fetch_min);

#[no_mangle]
pub unsafe extern "C" fn loom_yield_now() {
    loom::thread::yield_now();
}

#[no_mangle]
pub unsafe extern "C" fn loom_fence(ord: MemoryOrder) {
    loom::sync::atomic::fence(ord.into());
}

#[no_mangle]
pub unsafe extern "C" fn loom_atomic_init(val: *const c_void, ty: SizeType) -> *mut c_void {
    macro_rules! init {
        ($atomic:expr, $val:expr, $ty:ty, $atomic_ty:ty) => {{
            let val = $val as *const $ty;
            let atomic = Box::new(<$atomic_ty>::new(*val));
            Box::into_raw(atomic) as *mut c_void
        }};
    }
    match ty {
        SizeType::AtomicU8 => {
            init!(atomic, val, u8, AtomicU8)
        }
        SizeType::AtomicU16 => {
            init!(atomic, val, u16, AtomicU16)
        }
        SizeType::AtomicU32 => {
            init!(atomic, val, u32, AtomicU32)
        }
        SizeType::AtomicU64 => {
            init!(atomic, val, u64, AtomicU64)
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn loom_atomic_destroy(atomic: *mut c_void, ty: SizeType) {
    macro_rules! destroy {
        ($atomic:expr, $atomic_ty:ty) => {
            let atomic = $atomic as *mut $atomic_ty;
            drop(Box::from_raw(atomic));
        };
    }
    match ty {
        SizeType::AtomicU8 => {
            destroy!(atomic, AtomicU8);
        }
        SizeType::AtomicU16 => {
            destroy!(atomic, AtomicU16);
        }
        SizeType::AtomicU32 => {
            destroy!(atomic, AtomicU32);
        }
        SizeType::AtomicU64 => {
            destroy!(atomic, AtomicU64);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn loom_atomic_swap(
    atomic: *const c_void,
    val: *mut c_void,
    ty: SizeType,
    ord: MemoryOrder,
) {
    macro_rules! swap {
        ($atomic:expr, $val:expr, $ord:expr, $ty:ty, $atomic_ty:ty) => {
            let atomic = $atomic as *const $atomic_ty;
            let val = $val as *mut $ty;
            *val = (*atomic).swap(*val, $ord.into());
        };
    }
    match ty {
        SizeType::AtomicU8 => {
            swap!(atomic, val, ord, u8, AtomicU8);
        }
        SizeType::AtomicU16 => {
            swap!(atomic, val, ord, u16, AtomicU16);
        }
        SizeType::AtomicU32 => {
            swap!(atomic, val, ord, u32, AtomicU32);
        }
        SizeType::AtomicU64 => {
            swap!(atomic, val, ord, u64, AtomicU64);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn loom_atomic_compare_and_exchange(
    atomic: *const c_void,
    expected: *mut c_void,
    desired: *const c_void,
    ty: SizeType,
    success: MemoryOrder,
    failure: MemoryOrder,
) -> bool {
    macro_rules! compare_and_exchange {
        ($atomic:expr, $expected:expr, $desired:expr, $success:expr, $failure:expr, $ty:ty, $atomic_ty:ty) => {{
            let atomic = $atomic as *const $atomic_ty;
            let expected = $expected as *mut $ty;
            let desired = $desired as *const $ty;
            match (*atomic).compare_exchange(*expected, *desired, $success.into(), $failure.into())
            {
                Ok(_) => return true,
                Err(x) => {
                    *expected = x;
                    return false;
                }
            }
        }};
    }
    match ty {
        SizeType::AtomicU8 => {
            compare_and_exchange!(atomic, expected, desired, success, failure, u8, AtomicU8)
        }
        SizeType::AtomicU16 => {
            compare_and_exchange!(atomic, expected, desired, success, failure, u16, AtomicU16)
        }
        SizeType::AtomicU32 => {
            compare_and_exchange!(atomic, expected, desired, success, failure, u32, AtomicU32)
        }
        SizeType::AtomicU64 => {
            compare_and_exchange!(atomic, expected, desired, success, failure, u64, AtomicU64)
        }
    }
}

#[no_mangle]
pub extern "C" fn loom_create_thread(
    func: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
    arg: *mut c_void,
) -> *mut c_void {
    if let Some(func) = func {
        Box::into_raw(Box::new(loom::thread::spawn(move || {
            func(arg);
        }))) as *mut c_void
    } else {
        core::ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn loom_join_thread(thread: *mut c_void) -> *mut c_void {
    let thread = thread as *mut loom::thread::JoinHandle<*mut c_void>;
    let boxed = Box::from_raw(thread);
    boxed.join().unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn loom_atomic_wait(atomic: *const c_void, val: *const c_void, ty: SizeType) {
    macro_rules! wait {
        ($atomic:expr, $ty:ty, $atomic_ty:ty) => {{
            let atomic = $atomic as *const $atomic_ty;
            let expected = val as *const $ty;
            (*atomic).wait(*expected);
        }};
    }
    match ty {
        SizeType::AtomicU8 => {
            wait!(atomic, u8, AtomicU8);
        }
        SizeType::AtomicU16 => {
            wait!(atomic, u16, AtomicU16);
        }
        SizeType::AtomicU32 => {
            wait!(atomic, u32, AtomicU32);
        }
        SizeType::AtomicU64 => {
            wait!(atomic, u64, AtomicU64);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn loom_atomic_notify_one(atomic: *const c_void, ty: SizeType) -> bool {
    macro_rules! notify_one {
        ($atomic:expr, $atomic_ty:ty) => {{
            let atomic = $atomic as *const $atomic_ty;
            (*atomic).notify_one()
        }};
    }
    match ty {
        SizeType::AtomicU8 => notify_one!(atomic, AtomicU8),
        SizeType::AtomicU16 => notify_one!(atomic, AtomicU16),
        SizeType::AtomicU32 => notify_one!(atomic, AtomicU32),
        SizeType::AtomicU64 => notify_one!(atomic, AtomicU64),
    }
}

#[no_mangle]
pub unsafe extern "C" fn loom_atomic_notify_all(atomic: *const c_void, ty: SizeType) -> usize {
    macro_rules! notify_all {
        ($atomic:expr, $atomic_ty:ty) => {{
            let atomic = $atomic as *const $atomic_ty;
            (*atomic).notify_all()
        }};
    }
    match ty {
        SizeType::AtomicU8 => notify_all!(atomic, AtomicU8),
        SizeType::AtomicU16 => notify_all!(atomic, AtomicU16),
        SizeType::AtomicU32 => notify_all!(atomic, AtomicU32),
        SizeType::AtomicU64 => notify_all!(atomic, AtomicU64),
    }
}

#[no_mangle]
pub unsafe extern "C" fn loom_start(func: Option<unsafe extern "C" fn()>) {
    if let Some(func) = func {
        loom::model(move || unsafe {
            func();
        });
    }
}

#[no_mangle]
pub unsafe extern "C" fn loom_spin_loop_hint() {
    loom::sync::atomic::spin_loop_hint();
}
