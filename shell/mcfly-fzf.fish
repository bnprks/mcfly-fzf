#!/usr/bin/env fish

# Only run on interactive shells
# Avoid loading this file more than once
if status is-interactive && test "$__MCFLY_FZF_LOADED" != "loaded"
  set -g __MCFLY_FZF_LOADED "loaded"


  # Ensure mcfly is initialized first
  if not set -q MCFLY_SESSION_ID
    echo "Mcfly-fzf: Must initialize mcfly before mcfly-fzf"
    exit 1
  end

  # Find the mcfly-fzf binary
  set -q MCFLY_FZF_PATH; or set -g MCFLY_FZF_PATH (command -v mcfly-fzf)
  if test -z "$MCFLY_FZF_PATH"; or test "$MCFLY_FZF_PATH" = "mcfly-fzf not found"
    echo "Mcfly-fzf: Cannot find the mcfly-fzf binary, please make sure that mcfly-fzf is in your path before initializing"
    exit 1
  end

  # Find the fzf binary
  if not command -vq fzf
    echo "Mcfly-fzf: Cannot find the fzf binary, please make sure that fzf is in your path before initializing"
    exit 1
  end

  set -l tmpdir $TMPDIR
  if test -z "$tmpdir"
    set tmpdir /tmp
  end
  # MCFLY_SESSION_ID is used by McFly internally to keep track of the commands from a particular terminal session.
  set -gx MCFLY_FZF_OPTS (mktemp "$tmpdir/mcfly-fzf.json.XXXXXXXX")

  # Adapted from junegunn/fzf shell/key-bindings.fish
  function __mcfly-fzf-history-widget -d "Search command history with McFly (using fzf)"
    test -n "$FZF_TMUX_HEIGHT"; or set FZF_TMUX_HEIGHT 40%
    begin
      set -lx strict_ordering ("$MCFLY_FZF_PATH" toggle "$MCFLY_FZF_OPTS" strict-ordering --view)
      set -lx FZF_DEFAULT_OPTS "--height $FZF_TMUX_HEIGHT --bind=ctrl-z:ignore $FZF_DEFAULT_OPTS
        --nth=2.. --delimiter='\t' --no-hscroll --tiebreak=index --read0 --layout reverse
        --bind=ctrl-r:toggle-sort
        --bind='ctrl-r:+reload(\"$MCFLY_FZF_PATH\" toggle \"$MCFLY_FZF_OPTS\" strict-ordering && \"$MCFLY_FZF_PATH\" dump --header -0 --options-json \"$MCFLY_FZF_OPTS\")'
        --bind='f1:reload(\"$MCFLY_FZF_PATH\" toggle \"$MCFLY_FZF_OPTS\" sort-order && \"$MCFLY_FZF_PATH\" dump --header -0 --options-json \"$MCFLY_FZF_OPTS\")'
        --bind='f2:reload(\"$MCFLY_FZF_PATH\" toggle \"$MCFLY_FZF_OPTS\" current-dir-only && \"$MCFLY_FZF_PATH\" dump --header -0 --options-json \"$MCFLY_FZF_OPTS\")'
        --bind='f3:reload(\"$MCFLY_FZF_PATH\" toggle \"$MCFLY_FZF_OPTS\" exit-code && \"$MCFLY_FZF_PATH\" dump --header -0 --options-json \"$MCFLY_FZF_OPTS\")'
        --ansi
        --header-lines 1
        $FZF_CTRL_R_OPTS +m $strict_ordering"
      eval $MCFLY_FZF_PATH --history-format fish dump --header -0 --options-json $MCFLY_FZF_OPTS | eval fzf -q '(commandline)' |
      string replace -r "[^\t]*\t" "" | read -l result
      and commandline -- $result
      and eval $MCFLY_FZF_PATH select -- "$result"
    end
    commandline -f repaint
  end

  bind \cr __mcfly-fzf-history-widget
  if bind -M insert >/dev/null 2>&1
    bind -M insert \cr __mcfly-fzf-history-widget
  end
end
