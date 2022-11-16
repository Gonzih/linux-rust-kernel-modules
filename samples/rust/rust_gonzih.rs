//! Gonzih module

use kernel::prelude::*;
use kernel::{
    file,
    io_buffer::{IoBufferReader, IoBufferWriter},
    miscdev, mutex_init,
    sync::{Arc, ArcBorrow, Mutex, UniqueArc},
};

module! {
    type: Gonzih,
    name: "gonzih",
    author: "Max Gonzih",
    description: "gonzih",
    license: "GPL",
}

struct Device {
    number: usize,
    contents: Mutex<Vec<u8>>,
}

struct Gonzih {
    _dev: Pin<Box<miscdev::Registration<Gonzih>>>,
}

impl Drop for Gonzih {
    fn drop(&mut self) {
        pr_info!("Rust gonzih example (exit)\n");
    }
}

#[vtable]
impl file::Operations for Gonzih {
    type Data = Arc<Device>;
    type OpenData = Arc<Device>;

    fn open(context: &Self::OpenData, _file: &file::File) -> Result<Self::Data> {
        pr_info!("File for device {} was opened\n", context.number);
        Ok(context.clone())
    }

    fn read(
        this: ArcBorrow<'_, Device>,
        _file: &file::File,
        data: &mut impl IoBufferWriter,
        _offset: u64,
    ) -> Result<usize> {
        let buf = this.contents.lock();
        let n = buf.len();
        pr_info!("About to write {} bytes\n", n);
        data.write_slice("DATA:".as_bytes())?;
        data.write_slice(&buf[..])?;
        pr_info!("Read {} {} bytes\n", this.number, n);
        Ok(n + 5)
    }

    fn write(
        this: ArcBorrow<'_, Device>,
        _file: &file::File,
        data: &mut impl IoBufferReader,
        _offset: u64,
    ) -> Result<usize> {
        let n = data.len();
        pr_info!("About to write {} bytes\n", n);
        let buf = &mut this.contents.lock();
        data.read_slice(&mut buf[..])?;
        pr_info!("Received {}\n", buf.len());
        pr_info!("Written {} {} bytes\n", this.number, n);
        Ok(n)
    }
}

impl kernel::Module for Gonzih {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("====================================");
        pr_info!("Rust gonzih sample (init)\n");
        pr_info!("Am I built-in? {}\n", !cfg!(MODULE));
        pr_info!("====================================");

        let mut dev = Pin::from(UniqueArc::try_new(Device {
            number: 0,
            contents: unsafe { Mutex::new(Vec::new()) },
        })?);

        let pinned = unsafe { dev.as_mut().map_unchecked_mut(|s| &mut s.contents) };
        mutex_init!(pinned, "Device::contents");

        let reg = miscdev::Registration::<Gonzih>::new_pinned(fmt!("gonzih"), dev.into())?;

        Ok(Gonzih { _dev: reg })
    }
}
