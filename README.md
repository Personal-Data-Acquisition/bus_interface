# Bus_interface

The interface repo/crate for connecting sensor_modules to a bus controller.

Abstracts away the physical hardware and bus being used to allow for platform
agnostic communications.

## Goals 

* Be Bus independant.
* Be hardware/sensor independent.
* Allow Plug and play of new sensor_modules.


## Running tests

```sh

cd ./bus_interface
cargo test
```

## Using for sensor_module(embedded)

```toml

[dependencies]
bus_interface = {
    git = "https://github.com/Personal-Data-Acquisition/bus_interface.git",
    branch = "main",
    }

[dependencies.bus_interface]
version = "0.2.0"
default_features = false
features = ["bus_master"]

```


## Using for Bus Controller

```toml
[dependencies]
bus_interface = {
    git = "https://github.com/Personal-Data-Acquisition/bus_interface.git",
    branch = "main",
    }

[dependencies.bus_interface]
version = "0.2.0"
default_features = false
features = []


```


## Implimenting needed functions

**Controller(CAN master)**

- BUS:: The pub trait for the systems coms.

The controller side of things is pretty much ready to roll. It's assuming
it's run on a system that's powerful enough to make use of vectors and the
standard library.


**Handler(CAN slave)**

- SensorInterface:: this is a public trait.
- BUS:: another public trait.

For example on a stm32 system you would write a wrapper around the
bxCAN functionality that was compatible with the `pub trait Bus`, in order
to use the repo.


