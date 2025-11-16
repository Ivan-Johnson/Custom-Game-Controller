#!/usr/bin/env bash
set -euo pipefail

if [[ -v DEBUG ]]; then
	set -x
fi

###################
# PARSE ARGUMENTS #
###################

basename="$(basename "$0")"

if [ "$#" -ge 1 ] && ( [ "$1" = "-h" ] || [ "$1" = "--help" ] ); then
	echo "Usage:"
	fstring="\t%-40s %s\n"
	printf "$fstring" "$basename" "bar"
	exit 0
fi

if [ "$#" -ne 0 ]; then
	echo "Error: This script takes no args"
	echo
	"$0" --help
	exit 1
fi


#########
# SETUP #
#########
cd "$(dirname "$0")"

##############
# MAIN LOGIC #
##############

PORT="/dev/serial/by-id/usb-Arduino_Nano_Matter_CMSIS-DAP_81643ADB-if02"

# fully-qualified board name
FQBN="SiliconLabs:silabs:nano_matter"

BINARY_DIR="ArduinoIDE/test/build/SiliconLabs.silabs.nano_matter"
FNAME="test.ino.hex"
#test.ino.bin NO
#test.ino.hex YES
#test.ino.map NO
#test.ino.with_bootloader.bin NO
#test.ino.with_bootloader.hex YES

cp "$BINARY_DIR/$FNAME" "/tmp/"
status=0
arduino-cli upload --fqbn "$FQBN" --port "$PORT" --input-file "/tmp/$FNAME" || status="$?"
rm "/tmp/$FNAME"
exit "$status"
