echo ""
echo "Next directories and files are about to be deleted:"
echo ""
echo "/home/${USER}/.local/share/ternotes"
echo "/etc/ternotes/config.txt"
echo "/bin/usr/nts"
echo ""
read -p "Are you sure (y/N): " selection

if [[ "$selection" != "y" ]];
then
	echo "Action canceled."
	exit 1
fi

rm -rf /home/${USER}/.local/share/ternotes
sudo rm -rf /etc/ternotes/config.txt
sudo rm -rf /usr/bin/nts
