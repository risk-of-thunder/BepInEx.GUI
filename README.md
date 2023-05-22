# BepInEx.GUI

BepInEx.GUI is a graphical user interface (GUI) meant to replace the regular console host that is used by BepInEx when the config setting is enabled.

## Features

-   One time only disclaimer that give a quick guide on how to report mod issues properly for end users.

-   Show loaded mods when game is starting.

-   Buttons for fast access to the game modding discord, BepInEx folder, log folder.

-   Button for pausing the game process.

-   Console log entries with colors, live log level filtering, live text filtering.

-   A real console host that saves its position and size on closing and reopening.

-   Quickly close the game process and the GUI by pressing CTRL + F5

## How it works internally

### BepInEx.GUI.Loader

#### Purpose

-   Launch the actual gui called `bepinex_gui` made with Rust with the correct launch arguments.

-   Send the log entries through a localhost tcp socket.

### bepinex_gui

#### Purpose

-   Receive the log entries from `BepInEx.GUI.Loader`

-   Show them in a GUI
