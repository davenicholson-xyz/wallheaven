#!/usr/bin/env bash

# Sample script to set wallpaper on linux or mac

if [ -n "$1" ]; then
  IMG=$1
else
  read IMG
fi

EXT="${IMG##*.}"
if ! [[ $EXT =~ ^(jpg|jpeg|png)$ ]]; then
  echo "Incorrect file format"
  exit 1
fi

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
  # the following are some examples of setting wallpaper on various WMs

  # Wayland
  swww img $IMG

  ## X11
  #feh --bg-fill "$IMG"

  ## Gnome / Ubuntu
  #gsettings set org.gnome.desktop.background picture-uri file://"$IMG"
  #gsettings set org.gnome.desktop.background picture-uri-dark file://"$IMG"

  ## Plasma
  # dbus-send --session --dest=org.kde.plasmashell --type=method_call /PlasmaShell org.kde.PlasmaShell.evaluateScript string:"
  # var allDesktops = desktops();
  # for (i = 0; i < allDesktops.length; i++) {
  #     d = allDesktops[i];
  #     d.wallpaperPlugin = 'org.kde.image';
  #     d.currentConfigGroup = Array('Wallpaper', 'org.kde.image', 'General');
  #     d.writeConfig('Image', 'file://$IMG');
  # }
  # "

elif [[ "$OSTYPE" == "darwin"* ]]; then
  # Change wallpaper in OSX.
  osascript -e "tell application \"Finder\" to set desktop picture to POSIX file \"$IMG\""
fi
