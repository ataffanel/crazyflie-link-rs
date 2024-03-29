# Crazyflie link

Radio link implementation for the Crazyflie quadcopter.

This crates implements low-level link communication to a [Crazyflie] using the
[Crazyradio] dongle. It allows to scan for Crazyflies and to open a safe
bidirectional radio connection.


This crate API is async, the [async_executor] crate is used to abstract the async
executor. Examples are using `async-std`.

## Cargo features

By default the `native` feature is used which make use of the [Crazyradio crate]
which in turn uses `libusb` to access the Crazyradio. This will work on Linux,
Mac and Windows natively.

By disabling default features and enabling the feature `webusb`, the
[Crazyradio-webusb crate] will be used which allows to compile the link to wasm
in order to run in a WebUSB-compatible web-browser.

## Limitations

This crate currently only supports 2Mbit/s datarate.

[Crazyflie]: https://www.bitcraze.io/products/crazyflie-2-1/
[Crazyradio]: https://www.bitcraze.io/products/crazyradio-pa/
[async_executor]: https://crates.io/crates/async_executors
[Crazyradio crate]: https://crates.io/crates/crazyradio
[Crazyradio-webusb crate]: https://crates.io/crates/crazyradio-webusb