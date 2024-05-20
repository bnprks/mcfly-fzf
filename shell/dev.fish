#!/bin/bash
# Build mcfly and run a dev environment fish for local mcfly testing

this_dir=$(cd `dirname "$0"`; pwd)

# Setup for local testing.
mkdir -p $this_dir/.fish/.config/fish


rm -f ../target/debug/mcfly-fzf
rm -f ../target/debug/mcfly

echo 'mcfly init fish | source' > $this_dir/.fish/.config/fish/config.fish
echo 'mcfly-fzf init fish | source' >> $this_dir/.fish/.config/fish/config.fish

cargo build -p mcfly
cargo build

# For some reason, to get line numbers in backtraces, we have to run the binary directly.
XDG_DATA_HOME=$this_dir/.fish \
  XDG_CONFIG_HOME=$this_dir/.fish/.config \
  MCFLY_PATH=$this_dir/../target/debug/mcfly \
  RUST_BACKTRACE=full \
  MCFLY_DEBUG=1 \
  PATH=$this_dir/../target/debug/:$PATH \
  exec /usr/bin/env fish -i
