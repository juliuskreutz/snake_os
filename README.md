# snake_os

Just a fun little project to play a game of snake inside your bootloader.

# Building

You will need some kind of rust compiler (`rustup`) and the `dosfstools` for your system. After that `make` should build you an iso. To transfer that iso to an usb drive I usually just do `cat snake_os.iso > /dev/sdX`
