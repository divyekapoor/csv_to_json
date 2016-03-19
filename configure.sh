#!/bin/sh

if hash rustc 2>/dev/null; then
  echo
  echo "Rust compiler found. You're all good."
  echo
  echo "Please run:"
  echo "  make && sudo make install"
  echo
  exit 0
else
  echo "Rust compiler not found. Please install from https://www.rust-lang.org/"
  exit 1
fi

