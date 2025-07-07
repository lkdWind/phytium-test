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

        let mmio = iomap((base as usize).into(), reg.size.unwrap());

        let pl011 = Pl011 { base_addr: mmio };

        println!("PL011 base address: {:p}", pl011.base_addr);

        unsafe { pl011.base_addr.as_ptr().write_volatile(b'A') }; // Write 'A' to the data register}

        unsafe { pl011.base_addr.as_ptr().write_volatile(b'\r') }; // Write 'A' to the data register}
        unsafe { pl011.base_addr.as_ptr().write_volatile(b'\n') }; // Write 'A' to the data register}

        println!("test passed!");
    }
}
