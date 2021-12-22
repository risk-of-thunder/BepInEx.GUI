using BepInEx.GUI.Models;

namespace BepInEx.GUI.ViewModels
{
    public class MainWindowViewModel : ViewModelBase
    {
        public PathsInfo PathsInfo { get; }

        public WindowInfo WindowInfo { get; }

        public GeneralViewModel GeneralViewModel { get; }

        public SettingsViewModel SettingsViewModel { get; }

        public MainWindowViewModel(PathsInfo pathsInfo, PlatformInfo platformInfo, TargetInfo targetInfo)
        {
            PathsInfo = pathsInfo;

            WindowInfo = new WindowInfo(PathsInfo);

            GeneralViewModel = new GeneralViewModel(PathsInfo, platformInfo);

            SettingsViewModel = new SettingsViewModel(PathsInfo, targetInfo);
        }
    }
}
