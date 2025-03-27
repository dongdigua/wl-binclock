# Wayland Binary Clock
This is a toy project for learning Wayland and the first graphical project I've ever written.

The `wlr-layer-shell-unstable-v1` rust skeleton code is from
https://github.com/PurestAsh/wayland_hello_world_rust (https://bbs.deepin.org/post/280508)

It should show a 96x64 widget.

## Demos
![](images/demo_mono.png)<br>
![](images/demo_reversed.png)<br>
![](images/demo_brekkie.png)<br>
![](images/demo_peak.png)<br>
![](images/demo_dinners.png)<br>
![](images/demo_rainforest.png)<br>

## Usage
```
wl_binclock --fg 0x80e8b6 0xa1fff9 0xbd7cf8 0x7288f6  # use a palette (from Chicory: A Colorful Tale)
wl_binclock --anchor 9  # top-right (top=1 bottom=2 left=4 right=8)
```

## Related Projects
I made this project because I can't find a binary clock for wayland.
- [c4llv07e/binok](https://codeberg.org/c4llv07e/binok)
  not true "binary", though
