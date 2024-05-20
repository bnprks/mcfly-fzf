## 0.1.3 (5/19/2024)
- Bumped dependency versions
- Make strict ordering toggle (ctrl-r) persist in the settings
- This also changes the default to use strict ordering rather than letting fzf adjust based on quality of fuzzy match
- If environment variable `MCFLY_FZF_NO_STRICT_ORDERING` is set to non-empty, don't default to strict ordering