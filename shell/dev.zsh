#!/bin/bash
# Build mcfly and run a dev environment zsh for local mcfly testing

this_dir=$(cd `dirname "$0"`; pwd)

# Setup for local testing.
touch $this_dir/.zsh_history

# Needed so that the test instance of zsh sources the local mcfly.zsh file on startup.
echo 'rehash' > $this_dir/.zshrc
echo 'eval "$(mcfly init zsh)"' >> $this_dir/.zshrc
echo 'eval "$(mcfly-fzf init zsh)"' >> $this_dir/.zshrc
echo "SAVEHIST=1000" >> $this_dir/.zshrc
echo "HISTSIZE=1000" >> $this_dir/.zshrc

rm -f ../target/debug/mcfly-fzf
rm -f ../target/debug/mcfly

cargo build -p mcfly
cargo build

# For some reason, to get line numbers in backtraces, we have to run the binary directly.
HISTFILE=$this_dir/.zsh_history \
  MCFLY_PATH=$this_dir/../target/debug/mcfly \
  RUST_BACKTRACE=full \
  MCFLY_DEBUG=1 \
  ZDOTDIR="$this_dir" \
  SHELL="/bin/zsh" \
  PATH=$this_dir/../target/debug/:$PATH \
  /bin/zsh -i
