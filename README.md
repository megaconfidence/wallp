# wallp

A web service to cycle through a wallpaper directory written in Rust.

## Setup guide

Fix potential accessiblity issues by installing:

```
sudo apt-get install at-spi2-core
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
