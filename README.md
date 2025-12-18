# Terminal notes

The purpose of this tool is to be a simple and easy to use terminal notes taking tool, which supports copying lines to clipboard which comes in useful for storing commands. Here are some usage examples:

```bash
nts               # List the existing notes
nts <note_id>     # Print out the note
nts <note_id> e   # Edit the note in selected editor
nts <note_id> d   # Delete a note
nts a <note_name> # Add a note
```

## Installation
To install the ternotes, just call:
```bash
./install.sh
```
You can also remove everything related to ternotes with the uninstall command:
```bash
./uninstall.sh
```
## Config file
The config file is located at ```/etc/ternotes/config.txt```. The options are:
| Key        | Description                                          |
| --------   | -------                                              |
| editor_cmd | Editor that you are using                            |
| notes_dir  | The directory where the notes are stored             |
| clipboard  | The clipboard manager you are using (wlcopy \| xclip) |

Example:

```bash
# Terminal notes config file
editor_cmd: vim
notes_dir: /home/someuser/.local/share/ternotes
clipboard: xclip
```
