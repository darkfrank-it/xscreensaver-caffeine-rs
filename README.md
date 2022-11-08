# xscreensaver-caffeine-rs

Tray icon to give caffeine to xscreensaver, written in Rust.
A tray icon desktop indicator ‘xscreensaver-caffeine-rs’ supplies a manual toggle to prevent xscreensaver to start.

## Installation

### From package
~~~bash
sudo dpkg -i xscreensaver-caffeine-rs_1.0.0-2_all.deb
~~~

### From git
~~~bash
# install dependency
sudo apt update && sudo apt install libdbus-1-dev

git clone https://github.com/darkfrank-it/xscreensaver-caffeine-rs.git
cd xscreensaver-caffeine-rs

cargo build --release
~~~

## Manual run:

~~~bash
xscreensaver-caffeine-rs
~~~

## To run automaticcally at boot
Add the desktop file from `/usr/share/applications/xscreensaver-caffeine-rs.desktop` to the settings of your DE or add a link in autostart folder es:
 
~~~bash
ln -s /usr/share/applications/xscreensaver-caffeine-rs.desktop /home/$USER/.config/autostart/xscreensaver-caffeine-rs.desktop
~~~

## Uninstall
~~~bash
sudo apt remove xscreensaver-caffeine-rs
~~~
