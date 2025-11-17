#!/usr/bin/env bash

cd "$(dirname "$0")"
cd firmware

arduino-cli monitor -p /dev/serial/by-id/usb-Arduino_Nano_Matter_CMSIS-DAP_81643ADB-if02
