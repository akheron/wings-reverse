# Reversing Wings 1.40

This is an early attempt to reverse engineer some aspects of the classic Finnish
game [Wings](http://mvirpioj.mbnet.fi/wings/index.html) from 1990s.

Currently, the focus is to convert data files to more accessible formats.

Build:

```
$ cargo build
```

Run:

```
$ ./target/debug/wings-reverse <wings-dir> <output-dir>
```


## Notes on DosBox debugger

DosBox must be built with `./configure --enable-debug` or `--enable-debug=heavy`.
The "heavy" debugger has more memory debugging features.
Add the following to DosBox config to make it possible to step the debugger one
instruction at a time:

```
[cpu]
core=normal
```

By default, DosBox uses the dynamic core if possible.
It doesn't allow for instruction stepping.

Run `DEBUG.COM WINGS.EXE` to break at program startup.
Type `HELP` and press return to show the built-in help.

Each time I run the progam, CS is `00A7` and DS is `00AF`.
Not initially, but after talking to the DOS Extender and other initialization
stuff is done.
I'm not sure if this is universal among all programs, but this is what I get
consistently when running `WINGS.EXE`.
Knowing what `CS` will contain helps setting breakpoints at program startup.


## File formats

### `VGAFONT.PIC`

The font used throughout the game.

```
256 times =>
    width (2)
    height (2)
    pixel_data (width * height)
```

### Other `*.PIC` files

320x200 256-color PCX files with file extension changed to `.PIC`.

### `SHIPS/*.SHP`

Ship data.

```
WSHP (4)
ignored (4)
name_length (4)
name (name_length)
7 dwords (28) [probably ship properties/parameters]
72 times =>
    image_width (2)
    image_height (2)
    data_len (4)
    rle_encoded_data (data_len)
```

### `LEV/*.LEV`

Level files.

``` sh
palette (768)
level_image =>
    image_width (2)
    image_height (2)
    data_len (4)
    rle_encoded_data (data_len)
is_parallax (1)
if is_parallax =>
    parallax_image =>
        image_width (2)
        image_height (2)
        data_len (4)
        rle_encoded_data (data_len)
show_stars (1)
use_defaults (1)
if use_defaults != 2 =>
    rain_probability (2)
    snow_probability (2)
    bombing_probability (2)
    num_civilians (2)
    armed_civilians_percentage (2)
```

`palette` consists of 256 RBG values, i.e. 768 bytes in total. Each value is
in the range 0-63, and has to be multiplied by 4 to convert to 0-255.

`parallax_image` (level background) is present only if the `is_parallax` byte equals 1.

The last 5 parameter (10 bytes) are present only if the `use_defaults` byte does not equal 2.
