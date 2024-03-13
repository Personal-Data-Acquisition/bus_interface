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


```


## Using for Bus Controller

```toml

[dependencies]


```
