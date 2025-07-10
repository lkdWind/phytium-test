#![no_std]
#![no_main]
#![feature(used_with_arg)]

extern crate alloc;
extern crate bare_test;

#[bare_test::tests]
mod tests {
    use bare_test::{
        GetIrqConfig,
        driver::IrqConfig,
        globals::{PlatformInfoKind, global_val},
        irq::{IrqHandleResult, IrqParam},
        mem::iomap,
        println,
    };
    use core::{
        cell::UnsafeCell,
        ops::{Deref, DerefMut},
        sync::atomic::{AtomicBool, Ordering},
    };
    use log::info;
    use pl011::Pl011;

    pub struct Mutex<T> {
        inner: AtomicBool,
        data: UnsafeCell<T>,
    }

    unsafe impl<T> Sync for Mutex<T> {}
    unsafe impl<T> Send for Mutex<T> {}

    impl<T> Mutex<T> {
        pub const fn new(data: T) -> Self {
            Self {
                inner: AtomicBool::new(false),
                data: UnsafeCell::new(data),
            }
        }

        pub fn lock(&self) -> MutexGuard<'_, T> {
            loop {
                if self.inner.swap(true, Ordering::Acquire) {
                    continue;
                }
                return MutexGuard { mutex: self };
            }
        }

        pub fn unlock(&self) {
            self.inner.store(false, Ordering::Release);
        }

        #[allow(clippy::mut_from_ref)]
        pub fn force_use(&self) -> &mut T {
            unsafe { &mut *self.data.get() }
        }
    }
    pub struct MutexGuard<'a, T> {
        mutex: &'a Mutex<T>,
    }

    impl<T> Deref for MutexGuard<'_, T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            unsafe { &*self.mutex.data.get() }
        }
    }

    impl<T> DerefMut for MutexGuard<'_, T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { &mut *self.mutex.data.get() }
        }
    }

    impl<T> Drop for MutexGuard<'_, T> {
        fn drop(&mut self) {
            self.mutex.inner.store(false, Ordering::Release);
        }
    }

    static PL011: Mutex<Option<Pl011>> = Mutex::new(None);

    #[test]
    fn it_works() {
        info!("This is a test log message.");

        let PlatformInfoKind::DeviceTree(fdt) = &global_val().platform_info;
        let fdt = fdt.get();

        let node = fdt.find_compatible(&["arm,pl011"]).next().unwrap();

        let reg = node.reg().unwrap().next().unwrap();

        let base = reg.address;

        let mmio_addr = iomap((base as usize).into(), reg.size.unwrap());
        let mut pl011 = Pl011::new(mmio_addr);
        pl011.init();

        {
            let mut uart = PL011.lock();
            *uart = Some(pl011);
        }

        let irq_info = node.irq_info().unwrap();
        println!("IRQ: {:?}", irq_info);
        let irq_cfg = irq_info.cfgs[0].clone();

        IrqParam {
            intc: irq_info.irq_parent,
            cfg: irq_cfg,
        }
        .register_builder({
            |_irq| {
                PL011.force_use().as_mut().unwrap().handle_interrupt();
                IrqHandleResult::Handled
            }
        })
        .register();

        spin_on::spin_on(async {
            let mut uart = PL011.lock();
            let pl011 = uart.as_mut().unwrap();
            println!("PL011 base address: {:p}", mmio_addr);
            pl011.write_bytes(b"Hello, async pl011!\n").await;
            println!("irq count: {}", pl011.irq_count);
        });

        println!("test passed!");
    }
}
