#!/bin/zsh

# Ensure mcfly is initialized first
if [[ ! -n "$MCFLY_SESSION_ID" ]]; then
  echo "Mcfly-fzf: Must initialize mcfly before mcfly-fzf"
  return 1
fi

# Make sure history format is defined
if [[ ! -n "$MCFLY_HISTORY_FORMAT" ]]; then
  echo "Mcfly-fzf: MCFLY_HISTORY_FORMAT must be set by mcfly init zsh";
  return 1
fi

# Avoid loading this file more than once
if [[ -o interactive ]] && [[ "$__MCFLY_FZF_LOADED" != "loaded" ]]; then
  __MCFLY_FZF_LOADED="loaded"

  # Find the mcfly-fzf binary
  MCFLY_FZF_PATH=${MCFLY_FZF_PATH:-$(command -v mcfly-fzf)}
  if [[ -z "$MCFLY_FZF_PATH" || "$MCFLY_FZF_PATH" == "mcfly-fzf not found" ]]; then
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

  mcfly-fzf-exit-logger() {
    [ -n "$MCFLY_DEBUG" ] && echo "mcfly-fzf.zsh: Exiting and removing $MCFLY_FZF_OPTS"
    command rm -f $MCFLY_FZF_OPTS
  }
  zshexit_functions+=(mcfly-fzf-exit-logger)

  # If this is an interactive shell, take ownership of ctrl-r.
  if [[ $- =~ .*i.* ]]; then
    # Adapted from junegunn/fzf shell/key-bindings.zsh
    mcfly-fzf-history-widget() {
      local selected num
      setopt localoptions noglobsubst noposixbuiltins pipefail no_aliases 2> /dev/null

      opts="--height ${FZF_TMUX_HEIGHT:-40%} --bind=ctrl-z:ignore $FZF_DEFAULT_OPTS 
          --nth=2.. --delimiter='\t' --no-hscroll --tiebreak=index --read0 --layout reverse 
          --bind=ctrl-r:toggle-sort 
          --bind='ctrl-d:reload(\"$MCFLY_FZF_PATH\" delete {2} && \"$MCFLY_FZF_PATH\" dump --header -0 --options-json \"$MCFLY_FZF_OPTS\")'
          --bind='f1:reload(\"$MCFLY_FZF_PATH\" toggle \"$MCFLY_FZF_OPTS\" sort-order && \"$MCFLY_FZF_PATH\" dump --header -0 --options-json \"$MCFLY_FZF_OPTS\")' 
          --bind='f2:reload(\"$MCFLY_FZF_PATH\" toggle \"$MCFLY_FZF_OPTS\" current-dir-only && \"$MCFLY_FZF_PATH\" dump --header -0 --options-json \"$MCFLY_FZF_OPTS\")' 
          --bind='f3:reload(\"$MCFLY_FZF_PATH\" toggle \"$MCFLY_FZF_OPTS\" exit-code && \"$MCFLY_FZF_PATH\" dump --header -0 --options-json \"$MCFLY_FZF_OPTS\")' 
          --ansi
          --header-lines 1
          $FZF_CTRL_R_OPTS +m"

      selected=$(
          $MCFLY_FZF_PATH --history-format $MCFLY_HISTORY_FORMAT dump --header -0 --options-json $MCFLY_FZF_OPTS | FZF_DEFAULT_OPTS="$opts" fzf --query="$LBUFFER"
      )

      local ret=$?
      if [ -n "$selected" ]; then
          RBUFFER=""
          LBUFFER="${selected#*$'\t'}"
          if [ $ret -eq 0 ]; then
            $MCFLY_FZF_PATH select -- "$LBUFFER"
          fi
      fi
      zle reset-prompt
      return $ret
    }
    zle -N mcfly-fzf-history-widget
    bindkey '^R' mcfly-fzf-history-widget
  fi
fi