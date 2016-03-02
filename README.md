# The Wind Waker Bingo Edition

The Wind Waker Bingo Edition is a work in progress modification of Wind Waker that gives free exploration without any limitations.

Here is what you can expect when you start a new file:
 - Most cutscenes will be removed (all post dungeon cutscenes, all story cutscenes, etc)
 - start right on Outset
 - You will start with Wind Waker, Earth God's Lyric, and Wind God's Aria
 - Orca sword is not obtainable, so if you decide to get a sword, you need to go to FF1
 - King of Red Lions will be spawned and no limitations to use him
 - Pirate ship permanently at Windfall
 - Tower of the Gods will be raised
 - Mother and Child Isles will give Fire and Ice arrows
 - No song stones blocking Earth/Wind Temple
 - Medli and Makar will both be in their respective temples
 - Beating Forsaken Fortress 1 or 2 will spawn you at Windfall (no cutscenes obviously)
 - Hyrule 3 warp will be accessible
 - Barrier is removed
 
If you have more things you'd like to see added or potential balance problems please let us know and we'll see what we can do.

## How To Compile

To compile the source code you need to get the Rust Nightly compiler toolchain.
You can acquire it either through the [official website](https://www.rust-lang.org/downloads.html) or through [multirust](https://github.com/brson/multirust).
If you use multirust you might need to override the toolchain in the project by using ```multirust override nightly```.
Since we'll compile PowerPC code, you'll need to get a compiled **libcore** for PowerPC.
Luckily, there's a prebuilt version available [here](http://static.rust-lang.org/dist/).
