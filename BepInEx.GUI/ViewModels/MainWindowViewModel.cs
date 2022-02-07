using BepInEx.GUI.Models;
using WebSocketSharp;

namespace BepInEx.GUI.ViewModels
{
    public class MainWindowViewModel : ViewModelBase
    {
        public PathsInfo PathsInfo { get; }

        public WindowInfo WindowInfo { get; }

        public GeneralViewModel GeneralViewModel { get; }

        public ConsoleViewModel ConsoleViewModel { get; }

        public SettingsViewModel SettingsViewModel { get; }

        public MainWindowViewModel(WebSocket webSocket, PathsInfo pathsInfo, PlatformInfo platformInfo, TargetInfo targetInfo)
        {
            PathsInfo = pathsInfo;

            WindowInfo = new WindowInfo(PathsInfo);

            GeneralViewModel = new GeneralViewModel(PathsInfo, platformInfo, webSocket);

            ConsoleViewModel = new ConsoleViewModel(webSocket, targetInfo, platformInfo);

            SettingsViewModel = new SettingsViewModel(PathsInfo, targetInfo);
        }
    }
}
