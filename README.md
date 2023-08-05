# Reproducer for cheap FTDI dongle and serialport-rs

![cheap FTDI dongle](./img/ft232rt.png)

This bug has the dongle not sending the last few bytes on a series of transmissions.

I suspect it's related with the final FTDI URB "Reset" command being sent and reaching the device before the
last bytes transfer is able to finalise.
