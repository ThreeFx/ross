pub struct Uart {
    base_addr: usize,
}

impl Uart {
    pub fn new(base_addr: usize) -> Uart {
        Uart {
            base_addr: base_addr,
        }
    }

    pub fn put(self: &Self, c: u8) {
        let ptr = self.base_addr as *mut u8;
        unsafe {
            ptr.write_volatile(c);
        }
    }
}

impl core::fmt::Write for Uart {
    fn write_str(self: &mut Self, s: &str) -> Result<(), core::fmt::Error> {
        for c in s.bytes() {
            self.put(c)
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! print
{
	($($args:tt)+) => ({
        use core::fmt::Write;
			let _ = write!(uart::Uart::new(0x1000_0000), $($args)+);
	});
}

#[macro_export]
macro_rules! println
{
	() => ({
		print!("\r\n")
	});
	($fmt:expr) => ({
		print!(concat!($fmt, "\r\n"))
	});
	($fmt:expr, $($args:tt)+) => ({
		print!(concat!($fmt, "\r\n"), $($args)+)
	});
}
