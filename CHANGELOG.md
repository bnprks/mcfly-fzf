## 0.1.3 (5/19/2024)
Enhancements:

- Bumped dependency versions
- Make strict ordering toggle (ctrl-r) persist in the settings
- This also changes the default to use strict ordering rather than letting fzf adjust based on quality of fuzzy match
- If environment variable `MCFLY_FZF_NO_STRICT_ORDERING` is set to non-empty, don't default to strict ordering

## 0.1.2 (7/26/2023)
Fixes:
- The main change is a fix for the way the temporary settings json file is calculated, as I was getting name conflicts on MacOS

## 0.1.1 (1/3/2023)
Enhancements:

- mcfly-fzf dump runs about 4x faster when ordering by last run, since now we skip the neural network rank calculations in that case
- Some improved descriptions in the help command

## 0.1.0 (12/30/2022)
First release of mcfly-fzf

Initial Features:

- Advanced [history tracking](https://github.com/cantino/mcfly#features) and [prioritization](https://github.com/cantino/mcfly#prioritization) courtesy of McFly
- Rebinds `ctrl-r` to use an fzf-based interface
    - Supports [fzf search syntax](https://github.com/junegunn/fzf#search-syntax)
    - Inline layout [options](https://www.mankier.com/1/fzf#Options-Layout)
- Toggle options in fzf interface:
    - F1: Sort by time / neural network score
    - F2: Limit to commands run from current directory / any directory
    - F3: Filter by exit status: [Okay (zero) / Fail (non-zero) / Any]