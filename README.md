# quick_edit
*Simple helper to quickly edit frequently used text files*

## Motivation

During the daily usage of my system, I find myself editing the same text files over and over again.
Initially I used a tiny shell script, which worked fine.
Lately I wanted to learn Rust, therefore I decided to rewrite this script in Rust.

## How to Use

Place the binary somewhere on the system. Additionally, I like to bind it to a specific key in my window manager (XMonad):

```haskell
((modMask .|. shiftMask, xK_e), spawn $ XMonad.terminal conf ++ " -e ~/quick_edit")
```

Configure all files to be suggested by `quick_edit` in the file `.quick_edit.toml` (placed in the working directory, usually home):

```toml 
[choices]
paths = [ "~/notes.txt", "~/.xmonad/xmonad.hs", "/etc/hosts", "/etc/shorewall/rules" ]
```

Above configuration will produce following output:

```shell
0): ~/notes.txt
1): ~/.xmonad/xmonad.hs
2): /etc/hosts
3): /etc/shorewall/rules
Type choice:
```

Upon hitting the respective key, the default editor (assumed to be `$EDITOR`) will be used to open the file.

