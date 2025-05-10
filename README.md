# Wayland Binary Clock
This is a toy project for learning Wayland and the first graphical project I've ever written.

The `wlr-layer-shell-unstable-v1` rust skeleton code is from
https://github.com/PurestAsh/wayland_hello_world_rust (https://bbs.deepin.org/post/280508)

It should show a 96x64 widget.

## Demos
![](demos/demo_mono.png)
![](demos/demo_reversed.png)
![](demos/demo_brekkie.png)
![](demos/demo_peak.png)
![](demos/demo_dinners.png)
![](demos/demo_rainforest.png)

or with image

![](demos/demo_redstone.png)
![](demos/demo_shroom.png)
![](demos/demo_obsidian.png)


## Usage
```
wl-binclock --fg 0xff80e8b6 0xffa1fff9 0xffbd7cf8 0xff7288f6  # use a palette (from Chicory: A Colorful Tale)
wl-binclock --fg redstone_lamp_on.png --bg redstone_lamp.png # use a image palette
wl-binclock --bg 0  # full transparent
wl-binclock --anchor 9  # top-right (top=1 bottom=2 left=4 right=8)
```

## Usage: Advanced (IPC)
You can pipe 6-digit numbers into stdin when `--pipe` is enabled.
```
command | wl-binclock --pipe
wl-binclock --pipe < fifo
```

## Package
- [AUR](https://aur.archlinux.org/packages/wl-binclock)

## Related Projects
I made this project because I can't find a binary clock for wayland.
- [c4llv07e/binok](https://codeberg.org/c4llv07e/binok)
  not true "binary", though
