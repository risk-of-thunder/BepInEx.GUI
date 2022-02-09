using BepInEx.Configuration;

namespace BepInEx.GUI.Config
{
#pragma warning disable CS8618 // Non-nullable field must contain a non-null value when exiting constructor. Consider declaring as nullable.

    public static class MainConfig
    {
        public const string FileName = "BepInEx.GUI.cfg";

        public static ConfigFile File { get; private set; }

        public const string EnableDeveloperToolsText = "Enable Developer Tools";
        public static ConfigEntry<bool> EnableDeveloperToolsConfig { get; private set; }

        public const string ShowOneTimeOnlyDisclaimerText = "Show One Time Only Disclaimer";
        public static ConfigEntry<bool> ShowOneTimeOnlyDisclaimerConfig { get; private set; }

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

            ShowOneTimeOnlyDisclaimerConfig = File.Bind("Settings", ShowOneTimeOnlyDisclaimerText, true, ShowOneTimeOnlyDisclaimerText);

            CloseWindowWhenGameLoadedConfig = File.Bind("Settings", CloseWindowWhenGameLoadedConfigKey, false, CloseWindowWhenGameLoadedConfigDescription);

            CloseWindowWhenGameClosesConfig = File.Bind("Settings", CloseWindowWhenGameClosesConfigKey, true, CloseWindowWhenGameClosesConfigDescription);
        }
    }

#pragma warning restore CS8618 // Non-nullable field must contain a non-null value when exiting constructor. Consider declaring as nullable.

}
