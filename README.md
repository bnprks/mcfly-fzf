# McFly fzf integration
Integrate [McFly](https://github.com/cantino/mcfly) with [fzf](https://github.com/junegunn/fzf) to combine a solid command history database with a widely-loved fuzzy search UI

Features:

- Advanced [history tracking](https://github.com/cantino/mcfly#features) and [prioritization](https://github.com/cantino/mcfly#prioritization) courtesy of McFly
- Rebinds `ctrl-r` to use an fzf-based interface
    - Supports [fzf search syntax](https://github.com/junegunn/fzf#search-syntax)
    - Inline layout [options](https://www.mankier.com/1/fzf#Options-Layout)
- Toggle options in fzf interface:
    - F1: Sort by time / neural network score
    - F2: Limit to commands run from current directory / any directory
    - F3: Filter by exit status: [Okay (zero) / Fail (non-zero) / Any]

![Screenshot](screenshot.png)

## Installation
1. Install mcfly: Included with the [mcfly-fzf binaries](https://github.com/bnprks/mcfly-fzf/releases). Alternatively [download mcfly release binary](https://github.com/cantino/mcfly/releases), or view [other installation options](https://github.com/cantino/mcfly#installation)
2. Install fzf: [Download binary](https://github.com/junegunn/fzf/releases), or view [other installation options](https://github.com/junegunn/fzf#installation)
3. Install mcfly-fzf: [Download binary](https://github.com/bnprks/mcfly-fzf/releases)  
    Alternatively, install from source:
    ```
    cargo install --git https://github.com/bnprks/mcfly-fzf
    ```
4. Make sure that `mcfly`, `fzf`, and `mcfly-fzf` are all added to your `$PATH`. Following [instructions from McFly](https://github.com/cantino/mcfly#installing-manually-from-github): 
    > For example, you could create a directory at `~/bin`, copy `mcfly` to this location, and add `export PATH="$PATH:$HOME/bin"` to your `.bashrc` / `.zshrc`, or run `set -Ua fish_user_paths "$HOME/bin"` for fish.

5. Add the following to the end of your `~/.bashrc`, `~/.zshrc`, or `~/.config/fish/config.fish` file:

    Bash:
    ```bash
    eval "$(mcfly init bash)"
    eval "$(mcfly-fzf init bash)"
    ```

    Zsh:
    ```bash
    eval "$(mcfly init zsh)"
    eval "$(mcfly-fzf init zsh)"
    ```

    Fish:
    ```bash
    mcfly init fish | source
    mcfly-fzf init fish | source
    ```

## Usage
- Press `ctrl-r` to open search:
    - Type to search
    - Arrow keys to navigate up/down
    - Enter to select
    - F1/F2/F3 to adjust view settings
- Respects the following environment variables
    - [`MCFLY_HISTORY_LIMIT`](https://github.com/cantino/mcfly#slow-startup)
    - [`MCFLY_RESULTS_SORT`](https://github.com/cantino/mcfly#results-sorting)
    - `FZF_DEFAULT_OPTS`, `FZF_CTRL_R_OPTS` (can include multiple options from [`man fzf`](https://www.mankier.com/1/fzf))
    - `MCFLY_FZF_NO_STRICT_ORDERING`: if set, this variable causes mcfly-fzf to default to strict ordering off rather than on for new terminal sessions.
- To delete history entries, run `mcfly search` manually

## How it works
Overall:
- `mcfly-fzf` imports the `mcfly` crate to access the database.
- A per-session temporary json file stores the search setting toggles within and between searches.

What happens during `ctrl-r`:
1. `mcfly-fzf dump` reads the mcfly sqlite database and prints the entry to stdout, where it is piped to `fzf`
2. Toggling F1/F2/F3 in `fzf` runs `mcfly-fzf toggle`, which updates a temporary json file stored at `$MCFLY_FZF_OPTS`. Then `mcfly-fzf dump` runs again to re-populate the filtered results for `fzf`.
3. After selecting a history item, `mcfly-fzf select` marks the item in the `selected_commands` table so that the McFly neural network can prioritize frequently-selected items.

