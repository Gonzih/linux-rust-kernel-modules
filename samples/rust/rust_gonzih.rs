//! Gonzih module

use kernel::io_buffer::{IoBufferReader, IoBufferWriter};
use kernel::prelude::*;
use kernel::sync::{Arc, ArcBorrow, RwSemaphore};
use kernel::{file, miscdev};

module! {
    type: Gonzih,
    name: "gonzih",
    author: "Max Gonzih",
    description: "gonzih",
    license: "GPL",
}

struct Device {
    number: usize,
    contents: RwSemaphore<Vec<u8>>,
}

struct Gonzih {
    _dev: Pin<Box<miscdev::Registration<Gonzih>>>,
}

#[vtable]
impl file::Operations for Gonzih {
    type OpenData = Arc<Device>;
    type Data = Arc<Device>;

    fn open(context: &Self::OpenData, _file: &file::File) -> Result<Self::Data> {
        pr_info!("File for device {} was opened\n", context.number);
        Ok(context.clone())
    }

    fn read(
        data: ArcBorrow<'_, Device>,
        _file: &file::File,
        writer: &mut impl IoBufferWriter,
        _offset: u64,
    ) -> Result<usize> {
        let buf = data.contents.read();
        let n = buf.len();
        writer.write_slice(&buf[..])?;
        pr_info!("File was read {} {} bytes\n", data.number, n);
        Ok(n)
    }

    fn write(
        data: ArcBorrow<'_, Device>,
        _file: &file::File,
        reader: &mut impl IoBufferReader,
        _offset: u64,
    ) -> Result<usize> {
        let n = reader.len();
        let mut buf = data.contents.write();
        reader.read_slice(&mut buf[..])?;
        pr_info!("File was written {} {} bytes\n", data.number, n);
        Ok(n)
    }
}

impl kernel::Module for Gonzih {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("====================================");
        pr_info!("Rust gonzih sample (init)\n");
        pr_info!("Am I built-in? {}\n", !cfg!(MODULE));
        pr_info!("====================================");

        let dev = Arc::try_new(Device {
            number: 0,
            contents: unsafe { RwSemaphore::new(Vec::new()) },
        })?;
        let reg = miscdev::Registration::<Gonzih>::new_pinned(fmt!("gonzih"), dev)?;

        Ok(Gonzih { _dev: reg })
    }
}
