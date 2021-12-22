using BepInEx.Configuration;
using BepInEx.GUI.Models;
using ReactiveUI;

namespace BepInEx.GUI.ViewModels
{
    public class SettingsViewModel : ViewModelBase
    {
        public ConfigFile ConfigFile { get; }

        public const string EnableDeveloperToolsText = "Enable Developer Tools";
        private ConfigEntry<bool> _enableDeveloperToolsConfig;
        private bool _enableDeveloperTools;
        public bool EnableDeveloperTools
        {
            get { return _enableDeveloperTools; }
            set
            {
                _enableDeveloperToolsConfig.Value = this.RaiseAndSetIfChanged(ref _enableDeveloperTools, value);
                ConfigFile.Save();
            }
        }

        public const string CloseWindowWhenGameLoadedConfigKey = "Close Window When Game Loaded";
        public const string CloseWindowWhenGameLoadedConfigDescription = "Close the graphic user interface window when the game is loaded";
        public ConfigEntry<bool> _closeWindowWhenGameLoadedConfig { get; private set;}
        private bool _closeWindowWhenGameLoaded;
        public bool CloseWindowWhenGameLoaded
        {
            get { return _closeWindowWhenGameLoaded; }
            set
            {
                _closeWindowWhenGameLoadedConfig.Value = this.RaiseAndSetIfChanged(ref _closeWindowWhenGameLoaded, value);
                ConfigFile.Save();
            }
        }

        public const string CloseWindowWhenGameClosesConfigKey = "Close Window When Game Closes";
        public const string CloseWindowWhenGameClosesConfigDescription = "Close the graphic user interface window when the game closes";
        public ConfigEntry<bool> _closeWindowWhenGameClosesConfig { get; private set;}
        private bool _closeWindowWhenGameCloses;
        public bool CloseWindowWhenGameCloses
        {
            get { return _closeWindowWhenGameCloses; }
            set
            {
                _closeWindowWhenGameClosesConfig.Value = this.RaiseAndSetIfChanged(ref _closeWindowWhenGameCloses, value);
                ConfigFile.Save();
            }
        }

#pragma warning disable CS8618 // Non-nullable field must contain a non-null value when exiting constructor. Consider declaring as nullable.
        // dumb af compiler
        public SettingsViewModel(PathsInfo pathsInfo)
#pragma warning restore CS8618 // Non-nullable field must contain a non-null value when exiting constructor. Consider declaring as nullable.
        {
            ConfigFile = new ConfigFile(pathsInfo.ConfigFilePath, true);

            SetConfigBindings();
        }

        private void SetConfigBindings()
        {
            _enableDeveloperToolsConfig = ConfigFile.Bind("Settings", EnableDeveloperToolsText, false, EnableDeveloperToolsText);

            _closeWindowWhenGameLoadedConfig = ConfigFile.Bind("Settings", CloseWindowWhenGameLoadedConfigKey, false, CloseWindowWhenGameLoadedConfigDescription);

            _closeWindowWhenGameClosesConfig = ConfigFile.Bind("Settings", CloseWindowWhenGameClosesConfigKey, false, CloseWindowWhenGameClosesConfigDescription);
        }
    }
}
