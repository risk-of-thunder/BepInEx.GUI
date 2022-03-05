# BepInEx.GUI

BepInEx.GUI is a graphical user interface (GUI) meant to replace the regular console host that is used by BepInEx when the config setting is enabled.

### Features

- One time only disclaimer that give a quick guide on how to report mod issues properly for end users
- Show loaded mods when game is starting
- Buttons for fast access to the game modding discord, BepInEx folder, log folder
- Button for pausing game (Windows only)
- Console log entries with colors, live log level filtering, live text filtering

### Installation for developers

- Go to the [GitHub release](https://github.com/risk-of-thunder/BepInEx.GUI/releases)

- Download the .zip release corresponding to your OS

- Extract the .zip into the `BepInEx\patchers` folder

### Create a GitHub release through GitHub Action

```shell
git tag v1.0.0
git push --tags
```

This will make a new GitHub release with the given version number