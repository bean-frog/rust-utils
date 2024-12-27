# rust-utils
small utilities written in Rust
- build instructions (applies to all, unless stated)
	- clone this repo
	- cd to whichever directory
	- `cargo build`
	- optionally `sudo mv ./target/debug/<filename> /bin/<filename>` to make it globally accessible

## cfgedit
- create some aliases for config files that can be opened with short commands
- stored in ~/.config/rust-utils/configs.json
- Usage:
	- `cfgedit add <alias> /path/to/config.file` adds an alias to a config file. path should be absolute, don't use ~ etc
	- `cfgedit view` displays all aliases
	- `cfgedit delete <n>` deletes the nth alias with confirmation (starting from 1)
	- `cfgedit <alias>` opens the file in micro (this can be changed in the code if you prefer something else)

## notify
- create reminders and display them as dunst notifications
- stored in ~/.config/rust-utils/reminders.json
- Usage:
	- `notify add "notification text" <low|med|high>` adds a notification
	- `notify view` displays all notifications
	- `notify delete <n>` deletes the nth notification with confirmation (starting from 1)
	- `notify notify` fires all notifications with dunst. I have this tied to the unlock_cmd in my hypridle config.
