#!/bin/bash

sudo mkdir -p /etc/ternotes


mkdir -p "/home/${USER}/.local/share/ternotes"

echo "# Terminal notes config file" > __config.txt
echo "editor_cmd: vim" >> __config.txt
echo "notes_dir: /home/${USER}/.local/share/ternotes" >> __config.txt
echo "clipboard: xclip" >> __config.txt
sudo mv __config.txt /etc/ternotes/config.txt

cargo build --release
sudo cp target/release/nts /usr/bin

echo ""
echo "----------------------------"
echo ""
echo "The ternotes aplication is now installed. You"
echo "can use it by calling 'nts'."
echo ""
echo "You can change the /etc/ternotes/config.txt"
echo "to set the editor command, notes directory"
echo "and other settings."
echo ""
echo "Try calling 'nts h' to view possible commands."
echo ""
