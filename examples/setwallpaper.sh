#!/usr/bin/env bash

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
  case "$DESKTOP_SESSION" in
  plasma)
    dbus-send --session --dest=org.kde.plasmashell --type=method_call /PlasmaShell org.kde.PlasmaShell.evaluateScript string:"
      var allDesktops = desktops();
      for (i = 0; i < allDesktops.length; i++) {
          d = allDesktops[i];
          d.wallpaperPlugin = 'org.kde.image';
          d.currentConfigGroup = Array('Wallpaper', 'org.kde.image', 'General');
          d.writeConfig('Image', 'file://$IMG');
      }
      "
    ;;
  ubuntu | gnome)
    gsettings set org.gnome.desktop.background picture-uri file://"$IMG"
    gsettings set org.gnome.desktop.background picture-uri-dark file://"$IMG"
    ;;
  hyprland)
    swww img "$IMG"
    ;;
  i3)
    feh --bg-fill "$IMG"
    ;;
  *)
    echo "Unknown window manager"
    ;;
  esac

elif [[ "$OSTYPE" == "darwin"* ]]; then
  osascript -e "tell application \"Finder\" to set desktop picture to POSIX file \"$IMG\""
fi
