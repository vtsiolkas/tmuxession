# tmuxession

`tmuxession` is a simple rust cli application that allows you to save and
restore tmux sessions on a per-project basis.

## Getting started

![Tutorial][tutorial]

```sh
tmuxession save     # save the current tmux session, must be run from within tmux
tmuxession restore  # restore the tmux session associated with the current directory
tmuxession edit     # edit the tmux session script associated with the current directory
tmuxession list     # list all saved tmux sessions and pick one to restore/switch into
```

## Installation

Download the executable from the [Releases][latest-release] page. Move the
executable to a directory in your PATH (e.g. `~/.local/bin/`).

Optionally, you can set an alias in your shell configuration file (e.g.
`~/.bashrc`, `~/.zshrc`, etc.):

```sh
alias tm="tmuxession"
```

_Note:_ `tm` is just an example. You can use any alias you like.

You can also set a tmux keybinding in your tmux.conf to quickly save the
current session:

```
bind-key W confirm-before -p "Save session?" "run-shell 'tmuxession save'"
```

_Note:_ `tmuxession` is designed to work with tmux 3.1 and above. It may work
with older versions, but it hasn't been tested.

## Usage

The basic workflow of `tmuxession` is as follows:

- You have a tmux session running for a project with some windows and panes.
  You want to "persist" this session so that you can restore it later (even
  after reboots).
- You run `tmuxession save` from within a pane in the tmux session. This will
  extract all the necessary information about the session and create a shell
  script that "recreates" the session from start. By default, the script is
  saved in tmuxession's data directory (usually `~/.local/share/tmuxession/`)
  using the name of the current directory as the script name.
- You can run `tmuxession edit` to edit the saved session script. This is
  recommended in order to review the commands that were captured running inside
  the session's panes and make any necessary changes.
- You can then run `tmuxession restore` from the same directory to restore the
  session. This will run the script created by `tmuxession save`, recreate the
  session and attach to it. `tmuxession restore` should be run outside of tmux.
- Alternatively, you can run `tmuxession list` to see all saved sessions and
  pick one to restore regardless of the cwd. This is useful if you have
  multiple saved sessions and want to switch between them quickly.

When restoring a session, `tmuxession` checks if a session with the same name
is already running. If it is, it will prompt you to either attach to the
existing session, kill it and restore the saved session, or restore the saved
session with a different name.

_Note:_ If you are a Neovim user, `tmuxession` works well with the
[auto-session][auto-session] nvim plugin, which
automatically restores your nvim session based on the cwd.

## Capture

`tmuxession` captures the following information about the tmux session:

- The name of the session.
- The name and layout of each window in the session.
- The name, cwd and current command of each pane in each window.
- The currently active window and pane inside each window.

**_Warning:_** `tmuxession` captures the currently running command in each pane.
This means that if you were running a "harmful" one-time command in a pane (e.g.
`rm some_large_file`) while saving, it will be saved and will be run again when
you restore the session. Always review the script before restoring the session.

## Limitations

`tmuxession` has some limitations, mostly inherent from the way tmux works.

- `tmuxession` captures only the last run command in each pane. This means that
  if you run a command in a pane, then run another command in the same pane,
  only the second command will be captured.
- It also doesn't capture the full path of the command, only the command name.
  So if you ran `~/random/bin/program` in a pane, only `program` will be
  captured. This means that the script may not work if the command is not in
  your PATH.
- `tmuxession` doesn't capture the state of the shell (e.g. environment) in
  each pane. This means that if you had modified the environment before or
  after saving the session, the script may not work as expected.
- `tmuxession` doesn't yet capture some advanced tmux features like
  synchronized panes, session-specific options, etc. Most of these will
  probably be implemented in the future.

[auto-session]: https://github.com/rmagatti/auto-session
[tutorial]: contrib/tutorial.webm
[latest-release]: https://github.com/vtsiolkas/tmuxession/releases
