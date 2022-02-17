# BepInEx.GUI

BepInEx.GUI is a graphical user interface (GUI) meant to replace the regular console host that is used by BepInEx when the config setting is enabled.

### Features

- One time only disclaimer that give a quick guide on how to report mod issues properly for end users
- Show loaded mods when game is starting
- Buttons for fast access to the game modding discord, BepInEx folder, log folder
- Button for pausing game (Windows only)
- Console log entries with colors, live log level filtering, live text filtering

### Installation for developers

- Get the needed dependencies : [WebSlog](https://thunderstore.io/package/Twiner/WebSlog/) and 
a custom build of [BepInEx.Preloader](https://github.com/xiaoxiao921/BepInEx/commit/23705e678d8a8667eddcc510da4fa79313fbd4dd) 
that allow BepInEx patchers to depends on other patchers

- Put `WebSlog` into the `BepInEx\patchers` folder

- Put `BepInEx.Preloader.dll` into the `BepInEx\core` folder

- Go to the [GitHub release](https://github.com/risk-of-thunder/BepInEx.GUI/releases)

- Download the .zip release corresponding to your OS

- Extract the .zip into the `BepInEx\patchers` folder