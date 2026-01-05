# wallp

A web service to cycle through wallpapers written in Rust.

The idea is simple. You have a directory of wallpapers and want to cycle through them at the click of a button. The services take a wallpaper 
directory and exposes a get endpoint on port `8998`. Hit the endpoint to page through them. You could use this with a Stream Deck by importing the
[profile here](./streamdeck) (don't forget to update the ip address).

Dependencies:
- Linux GUI desktop (LXDE tested on Pi 4)
- `pcmanfm`
- Rust

![how it works](./demo.gif)

## Setup guide

Install the dependencies:

```
sudo apt install pcmanfm
sudo apt install at-spi2-core #fixes potential accessiblity issues
```

Clone the repo:

```
git clone https://github.com/megaconfidence/wallp.git
cd wallp
```

Add your wallpaper directory to the environment:

```
echo "WALLPAPER_DIR=/path/to/your/wallpaper/dir" > .env
```

[Install rust](https://rust-lang.org/learn/get-started/) and then compile project:

```
cargo build --release
```

Make the `run.sh` script executable:

```
chmod +x ./run.sh
```

Create an auto-start desktop entry in `~/.config/autostart/wall.desktop`:

> Update the path to the project directory below
> Also using crontab or systemd doesn't work because of pcmanfm GUI permission issues

```
[Desktop Entry]
Type=Application
Name=Wallp
Exec=sh <path-to-the-project-direcotry>/run.sh
```

Reboot:

```
sudo reboot
```

Test with a curl request:

```
curl localhost:8998
```
