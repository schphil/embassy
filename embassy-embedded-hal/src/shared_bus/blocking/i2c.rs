//! Blocking shared I2C bus
//!
//! # Example (nrf52)
//!
//! ```rust
//! use embassy_embedded_hal::shared_bus::blocking::i2c::I2cBusDevice;
//! use embassy::blocking_mutex::{NoopMutex, raw::NoopRawMutex};
//!
//! static I2C_BUS: Forever<NoopMutex<RefCell<Twim<TWISPI0>>>> = Forever::new();
//! let irq = interrupt::take!(SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0);
//! let i2c = Twim::new(p.TWISPI0, irq, p.P0_03, p.P0_04, Config::default());
//! let i2c_bus = NoopMutex::new(RefCell::new(i2c));
//! let i2c_bus = I2C_BUS.put(i2c_bus);
//!
//! let i2c_dev1 = I2cBusDevice::new(i2c_bus);
//! let mpu = Mpu6050::new(i2c_dev1);
//! ```

use core::cell::RefCell;

use embassy::blocking_mutex::raw::RawMutex;
use embassy::blocking_mutex::Mutex;
use embedded_hal_1::i2c::blocking::{I2c, Operation};
use embedded_hal_1::i2c::ErrorType;

use crate::shared_bus::i2c::I2cBusDeviceError;
use crate::SetConfig;

pub struct I2cBusDevice<'a, M: RawMutex, BUS> {
    bus: &'a Mutex<M, RefCell<BUS>>,
}

impl<'a, M: RawMutex, BUS> I2cBusDevice<'a, M, BUS> {
    pub fn new(bus: &'a Mutex<M, RefCell<BUS>>) -> Self {
        Self { bus }
    }
}

impl<'a, M: RawMutex, BUS> ErrorType for I2cBusDevice<'a, M, BUS>
where
    BUS: ErrorType,
{
    type Error = I2cBusDeviceError<BUS::Error>;
}

impl<M, BUS> I2c for I2cBusDevice<'_, M, BUS>
where
    M: RawMutex,
    BUS: I2c,
{
    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.bus
            .lock(|bus| bus.borrow_mut().read(address, buffer).map_err(I2cBusDeviceError::I2c))
    }

    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.bus
            .lock(|bus| bus.borrow_mut().write(address, bytes).map_err(I2cBusDeviceError::I2c))
    }

    fn write_read(&mut self, address: u8, wr_buffer: &[u8], rd_buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.bus.lock(|bus| {
            bus.borrow_mut()
                .write_read(address, wr_buffer, rd_buffer)
                .map_err(I2cBusDeviceError::I2c)
        })
    }

    fn transaction<'a>(&mut self, address: u8, operations: &mut [Operation<'a>]) -> Result<(), Self::Error> {
        let _ = address;
        let _ = operations;
        todo!()
    }

    fn write_iter<B: IntoIterator<Item = u8>>(&mut self, addr: u8, bytes: B) -> Result<(), Self::Error> {
        let _ = addr;
        let _ = bytes;
        todo!()
    }

    fn write_iter_read<B: IntoIterator<Item = u8>>(
        &mut self,
        addr: u8,
        bytes: B,
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        let _ = addr;
        let _ = bytes;
        let _ = buffer;
        todo!()
    }

    fn transaction_iter<'a, O: IntoIterator<Item = Operation<'a>>>(
        &mut self,
        address: u8,
        operations: O,
    ) -> Result<(), Self::Error> {
        let _ = address;
        let _ = operations;
        todo!()
    }
}

impl<'a, M, BUS, E> embedded_hal_02::blocking::i2c::Write for I2cBusDevice<'_, M, BUS>
where
    M: RawMutex,
    BUS: embedded_hal_02::blocking::i2c::Write<Error = E>,
{
    type Error = I2cBusDeviceError<E>;

    fn write<'w>(&mut self, addr: u8, bytes: &'w [u8]) -> Result<(), Self::Error> {
        self.bus
            .lock(|bus| bus.borrow_mut().write(addr, bytes).map_err(I2cBusDeviceError::I2c))
    }
}

impl<'a, M, BUS, E> embedded_hal_02::blocking::i2c::Read for I2cBusDevice<'_, M, BUS>
where
    M: RawMutex,
    BUS: embedded_hal_02::blocking::i2c::Read<Error = E>,
{
    type Error = I2cBusDeviceError<E>;

    fn read<'w>(&mut self, addr: u8, bytes: &'w mut [u8]) -> Result<(), Self::Error> {
        self.bus
            .lock(|bus| bus.borrow_mut().read(addr, bytes).map_err(I2cBusDeviceError::I2c))
    }
}

impl<'a, M, BUS, E> embedded_hal_02::blocking::i2c::WriteRead for I2cBusDevice<'_, M, BUS>
where
    M: RawMutex,
    BUS: embedded_hal_02::blocking::i2c::WriteRead<Error = E>,
{
    type Error = I2cBusDeviceError<E>;

    fn write_read<'w>(&mut self, addr: u8, bytes: &'w [u8], buffer: &'w mut [u8]) -> Result<(), Self::Error> {
        self.bus.lock(|bus| {
            bus.borrow_mut()
                .write_read(addr, bytes, buffer)
                .map_err(I2cBusDeviceError::I2c)
        })
    }
}

pub struct I2cBusDeviceWithConfig<'a, M: RawMutex, BUS: SetConfig> {
    bus: &'a Mutex<M, RefCell<BUS>>,
    config: BUS::Config,
}

impl<'a, M: RawMutex, BUS: SetConfig> I2cBusDeviceWithConfig<'a, M, BUS> {
    pub fn new(bus: &'a Mutex<M, RefCell<BUS>>, config: BUS::Config) -> Self {
        Self { bus, config }
    }
}

impl<'a, M, BUS> ErrorType for I2cBusDeviceWithConfig<'a, M, BUS>
where
    M: RawMutex,
    BUS: ErrorType + SetConfig,
{
    type Error = I2cBusDeviceError<BUS::Error>;
}

impl<M, BUS> I2c for I2cBusDeviceWithConfig<'_, M, BUS>
where
    M: RawMutex,
    BUS: I2c + SetConfig,
{
    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.bus.lock(|bus| {
            let mut bus = bus.borrow_mut();
            bus.set_config(&self.config);
            bus.read(address, buffer).map_err(I2cBusDeviceError::I2c)
        })
    }

    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.bus.lock(|bus| {
            let mut bus = bus.borrow_mut();
            bus.set_config(&self.config);
            bus.write(address, bytes).map_err(I2cBusDeviceError::I2c)
        })
    }

    fn write_read(&mut self, address: u8, wr_buffer: &[u8], rd_buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.bus.lock(|bus| {
            let mut bus = bus.borrow_mut();
            bus.set_config(&self.config);
            bus.write_read(address, wr_buffer, rd_buffer)
                .map_err(I2cBusDeviceError::I2c)
        })
    }

    fn transaction<'a>(&mut self, address: u8, operations: &mut [Operation<'a>]) -> Result<(), Self::Error> {
        let _ = address;
        let _ = operations;
        todo!()
    }

    fn write_iter<B: IntoIterator<Item = u8>>(&mut self, addr: u8, bytes: B) -> Result<(), Self::Error> {
        let _ = addr;
        let _ = bytes;
        todo!()
    }

    fn write_iter_read<B: IntoIterator<Item = u8>>(
        &mut self,
        addr: u8,
        bytes: B,
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        let _ = addr;
        let _ = bytes;
        let _ = buffer;
        todo!()
    }

    fn transaction_iter<'a, O: IntoIterator<Item = Operation<'a>>>(
        &mut self,
        address: u8,
        operations: O,
    ) -> Result<(), Self::Error> {
        let _ = address;
        let _ = operations;
        todo!()
    }
}
