#!/bin/sh
socat PTY,link=/tmp/tty_test0,echo=0 PTY,link=/tmp/tty_test1,echo=0

