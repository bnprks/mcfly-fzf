#!/bin/bash
# Build mcfly and run a dev environment bash for local mcfly testing

if ! this_dir=$(cd "$(dirname "$0")" && pwd); then
    exit $?
fi

rm -f ../target/debug/mcfly-fzf
rm -f ../target/debug/mcfly

echo 'eval "$(mcfly init bash)"' > $this_dir/.bash_dev_setup
echo 'eval "$(mcfly-fzf init bash)"' >> $this_dir/.bash_dev_setup

cargo build -p mcfly
cargo build

# For some reason, to get line numbers in backtraces, we have to run the binary directly.
HISTFILE=$HOME/.bash_history \
  MCFLY_PATH=$this_dir/../target/debug/mcfly \
  RUST_BACKTRACE=full \
  MCFLY_DEBUG=1 \
  PATH=$this_dir/../target/debug/:$PATH \
  SHELL="/bin/bash" \
  exec /bin/bash --init-file "$this_dir/.bash_dev_setup" -i
