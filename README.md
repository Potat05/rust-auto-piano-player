
# rust-auto-piano-player

This is my first actual rust project.

So it's probably very ugly and bad.

I hope if theres something so stupid someone makes a pr to make it less stupid and I will learn.



* TODO
    * Better midi support
        * Extract song name from midi if there is any.
        * Extract description from midi if there is any.
        * Extract URL from midi if there is any.
    * Figure out github actions to auto build the project
    * Add icon to exe
        * (I have fought hard and long, gave up and failed. I do not want to install 5gb of extra stuff just to get icons to work.)
        * (Someone please find an easy, not dumb way to do this.)



# Usage

Build the project with `cargo build`

Inside `target/debug/auto_piano_player.exe` drag a song file onto the executable.

The song schema is inside `schema/piano_song.json`

Some sample songs are inside `songs/`


