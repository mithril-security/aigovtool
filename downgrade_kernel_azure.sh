#!/bin/bash

set -e

# Check if the script is run as root
if [[ $EUID -ne 0 ]]; then
   echo "This script must be run as root" 
   exit 1
fi

KERNEL_VERSION="5.15.0-1043-azure"
KERNEL_PKG="linux-image-$KERNEL_VERSION"

apt-get install -y $KERNEL_PKG

MENU_ENTRY=$(grep -E "menuentry '[^']*$KERNEL_VERSION" /boot/grub/grub.cfg | head -n 1 | cut -d "'" -f 2)

if [ -z "$MENU_ENTRY" ]; then
    echo "Menu entry for $KERNEL_PKG not found."
    exit 1
fi


echo "Setting default menu entry for grub to '$MENU_ENTRY'"
sed -i.bak "s/^GRUB_DEFAULT=.*$/GRUB_DEFAULT=\"$MENU_ENTRY\"/" /etc/default/grub
apt remove linux-image-5.15.0-1049-azure -y
apt remove linux-image-5.15.0-1047-azure -y
apt remove linux-image-5.15.0-1045-azure -y

update-grub
