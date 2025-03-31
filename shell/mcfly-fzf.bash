#!/bin/bash

# Ensure stdin is a tty
# Only run on interactive shells
# Avoid loading this file more than once
if [[ -t 0 ]] && [[ $- =~ .*i.* ]] && [[ "$__MCFLY_FZF_LOADED" != "loaded" ]]; then
  __MCFLY_FZF_LOADED="loaded"

  # Ensure mcfly is initialized first
  if  [[ -z "$MCFLY_SESSION_ID" ]]; then
    echo "Mcfly-fzf: Must initialize mcfly before mcfly-fzf"
    return 1
  fi

  # Find the mcfly-fzf binary
  MCFLY_FZF_PATH=${MCFLY_FZF_PATH:-$(command -v mcfly-fzf)}
  if [ -z "$MCFLY_FZF_PATH" ]; then
    echo "Mcfly-fzf: Cannot find the mcfly-fzf binary, please make sure that mcfly-fzf is in your path before initializing"
    return 1
  fi

  # Find the fzf binary
  _FZF_PATH=$(command -v fzf)
  if [[ -z "$_FZF_PATH" ]]; then
    echo "Mcfly-fzf: Cannot find the fzf binary, please make sure that fzf is in your path before initializing"
    return 1
  fi

  # Get temporary file for communicating view options state
  if [[ ! -f "${MCFLY_FZF_OPTS}" ]]; then
    export MCFLY_FZF_OPTS=$(command mktemp ${TMPDIR:-/tmp}/mcfly-fzf.json.XXXXXXXX)
  fi

  # Function to perform ctrl-r binding for fzf. Adapted from junegunn/fzf shell/key-bindings.bash
  __mcfly_fzf_history__() {
    local output opts script header strict_ordering
    strict_ordering="$("$MCFLY_FZF_PATH" toggle "$MCFLY_FZF_OPTS" strict-ordering --view)"
    opts="--height ${FZF_TMUX_HEIGHT:-40%} --bind=ctrl-z:ignore $FZF_DEFAULT_OPTS
        --nth=2.. --delimiter='\t' --no-hscroll --tiebreak=index --read0 --layout reverse
        --bind=ctrl-r:toggle-sort
        --bind='ctrl-r:+reload(\"$MCFLY_FZF_PATH\" toggle \"$MCFLY_FZF_OPTS\" strict-ordering && \"$MCFLY_FZF_PATH\" dump --header -0 --options-json \"$MCFLY_FZF_OPTS\")'
        --bind='f1:reload(\"$MCFLY_FZF_PATH\" toggle \"$MCFLY_FZF_OPTS\" sort-order && \"$MCFLY_FZF_PATH\" dump --header -0 --options-json \"$MCFLY_FZF_OPTS\")'
        --bind='f2:reload(\"$MCFLY_FZF_PATH\" toggle \"$MCFLY_FZF_OPTS\" current-dir-only && \"$MCFLY_FZF_PATH\" dump --header -0 --options-json \"$MCFLY_FZF_OPTS\")'
        --bind='f3:reload(\"$MCFLY_FZF_PATH\" toggle \"$MCFLY_FZF_OPTS\" exit-code && \"$MCFLY_FZF_PATH\" dump --header -0 --options-json \"$MCFLY_FZF_OPTS\")'
        --ansi
        --header-lines 1
        $FZF_CTRL_R_OPTS +m $strict_ordering"
    output=$(
      "$MCFLY_FZF_PATH" dump --header -0 --options-json "$MCFLY_FZF_OPTS" |
        FZF_DEFAULT_OPTS="$opts" fzf --query "$READLINE_LINE"
    ) || return
    READLINE_LINE=${output#*$'\t'}
    "$MCFLY_FZF_PATH" select -- "$READLINE_LINE"
    if [[ -z "$READLINE_POINT" ]]; then
      echo "$READLINE_LINE"
    else
      READLINE_POINT=0x7fffffff
    fi
  }

  bind -x '"\C-r": __mcfly_fzf_history__'

fi
