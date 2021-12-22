using BepInEx.Configuration;

namespace BepInEx.GUI.Config
{
    public static class MainConfig
    {
        public const string FileName = "BepInEx.GUI.cfg";

        public static ConfigFile File { get; private set; }

        public const string EnableDeveloperToolsText = "Enable Developer Tools";
        public static ConfigEntry<bool> EnableDeveloperToolsConfig;

        public const string CloseWindowWhenGameLoadedConfigKey = "Close Window When Game Loaded";
        public const string CloseWindowWhenGameLoadedConfigDescription = "Close the graphic user interface window when the game is loaded";
        public static ConfigEntry<bool> CloseWindowWhenGameLoadedConfig { get; private set; }

        public const string CloseWindowWhenGameClosesConfigKey = "Close Window When Game Closes";
        public const string CloseWindowWhenGameClosesConfigDescription = "Close the graphic user interface window when the game closes";
        public static ConfigEntry<bool> CloseWindowWhenGameClosesConfig { get; private set; }

        public static void Init(string configFilePath)
        {
            File = new ConfigFile(configFilePath, true);

            EnableDeveloperToolsConfig = File.Bind("Settings", EnableDeveloperToolsText, false, EnableDeveloperToolsText);

            CloseWindowWhenGameLoadedConfig = File.Bind("Settings", CloseWindowWhenGameLoadedConfigKey, false, CloseWindowWhenGameLoadedConfigDescription);

            CloseWindowWhenGameClosesConfig = File.Bind("Settings", CloseWindowWhenGameClosesConfigKey, false, CloseWindowWhenGameClosesConfigDescription);
        }
    }
}
