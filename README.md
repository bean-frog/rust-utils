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
	- `notify <add|create|a|c> "notification text" <low|med|high>` adds a notification
	- `notify <view|show|v|s>` displays all notifications
	- `notify <delete|remove|d|r> <n>` deletes the nth notification with confirmation (starting from 1)
	- `notify <notify|n>` fires all notifications with dunst. I have this tied to the unlock_cmd in my hypridle config.
## astrocalc
- some astronomical calculations
- Usage:
	- `astrocalc distance isdeg showwork alpha1 delta1 alpha2 delta2` - calculates approximate angular separation (deg and rad) and distance (arcseconds) between 2 Ra/Dec points.
		- isdeg: boolean, true = alpha/delta values are in degrees, false = radians
		- showwork: boolean, true = display the steps to calculate, false = shut up and just give me the answer
		- alpha 1/2: float, the Right Ascension (Ra) of each coordinate
		- delta 1/2: float, the Declination (Dec) of each coordinate

	- `astrocalc convert <rad-deg|deg-rad> input` - converts some values
		- input can be a float, int, or string. The substring 'pi' will be evaluated as pi. for example, `180, 2pi, 180.0 are all acceptable inputs.

## chatgpt
- access the chatgpt API from the command line
- Usage:
	- `chatgpt setkey <apikey>` - WARNING: this puts your api key under ~/.config/rust-utils/key 
	- `chatgpt <model> input`
	- if no model provided, default is gpt-3.5
