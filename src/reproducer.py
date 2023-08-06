#!/usr/bin/env python3

from argparse import ArgumentParser, BooleanOptionalAction
from serial import Serial
import time


parser = ArgumentParser()
parser.add_argument('--packet-separation-ms', type=int, default=None)
parser.add_argument('--flush-before-drop', action=BooleanOptionalAction)
parser.add_argument('--baud', type=int, default=9600)
parser.add_argument('port')

args = parser.parse_args()


FOO = bytes.fromhex('ca fe ca fe ca fe')
BAR = bytes.fromhex('ba be ba be ba be')
BAZ = bytes.fromhex('de ad be ef de ad be ef')


ser = Serial(args.port)

ser.write(FOO)
if args.packet_separation_ms:
    ser.flush()
    time.sleep(args.packet_separation_ms / 1000.0)
ser.write(BAR)
if args.packet_separation_ms:
    ser.flush()
    time.sleep(args.packet_separation_ms / 1000.0)
ser.write(BAZ)

if args.flush_before_drop:
    ser.flush()

ser.close()

