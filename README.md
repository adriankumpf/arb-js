# arb-js

A native Node.js module for controlling the ABACOM CH341A relay board.

## Getting started

### Requirements

In order to compile a recent version of `rust` must be installed. Also, the native [libusb](https://github.com/libusb/libusb) library is required. On Debian-based distributions install `libusb-1.0-0-dev`.

### Installation

Add `arb` to your list of dependencies:

```plain
npm i adriankumpf/arb-js#v0.5.0
```

## Usage

```js
> arb.activate([1, 4, 7])
undefined

> arb.activate([1, 4, 7], { port: 2, verify: false })
undefined
```

## See also

- [abacom-relay-board](https://github.com/adriankumpf/abacom-relay-board)
- [arb-ex](https://github.com/adriankumpf/arb-ex)
