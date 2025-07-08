#![no_std]
#![no_main]
#![feature(used_with_arg)]

extern crate alloc;
extern crate bare_test;

#[bare_test::tests]
mod tests {
    use bare_test::{
        globals::{PlatformInfoKind, global_val},
        mem::iomap,
        println,
    };
    use log::info;
    use pl011::Pl011;

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
        println!("PL011 base address: {:p}", mmio_addr);
        unsafe {
            pl011.init();
            for i in 0..10 {
                pl011.send_byte(b'0' + i);
            }
        }
        println!("test passed!");
    }
}
