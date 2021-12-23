using BepInEx.GUI.Models;
using ReactiveUI;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Diagnostics;
using System.IO;
using WebSocketSharp;

namespace BepInEx.GUI.ViewModels
{
    public class GeneralViewModel : ViewModelBase
    {
        public PathsInfo PathsInfo { get; }

        public string TargetIsLoadingCanCloseWindow { get; }

        private string _loadedModCountText;
        public string LoadedModCountText
        {
            get { return _loadedModCountText; }
            set
            {
                this.RaiseAndSetIfChanged(ref _loadedModCountText, value);
            }
        }

        public ObservableCollection<Mod> Mods { get; }

        public PlatformInfo PlatformInfo { get; }

        public GeneralViewModel(PathsInfo pathsInfo, PlatformInfo platformInfo, WebSocket webSocket)
        {
            PathsInfo = pathsInfo;

            TargetIsLoadingCanCloseWindow = $"{pathsInfo.ProcessName} is loading, you can safely close this window.";

            Mods = new ObservableCollection<Mod>();
            webSocket.OnMessage += AddLoadedModToList;

            PlatformInfo = platformInfo;
        }

        private void AddLoadedModToList(object? sender, MessageEventArgs e)
        {
            const string LoadingModLog = "Loading [";

            var logEntry = LogEntry.Deserialize(e.RawData);
            if (logEntry == null)
            {
                return;
            }

            var logEntryText = logEntry.Data;
            if (logEntryText.Contains(LoadingModLog) && logEntry.Source == "BepInEx")
            {
                var modInfoArray = logEntryText.Split('[')[1].Split(' ');
                var modName = modInfoArray[0];
                var modVersion = modInfoArray[1].Remove(modInfoArray[1].Length - 1, 1);

                Mods.Add(new Mod(modName, modVersion));

                LoadedModCountText = $"Loaded Mods: {Mods.Count}";
            }
        }

        public void OnClickOpenGameFolder()
        {
            OpenFolder(PathsInfo.GameFolderPath);
        }

        public void OnClickShowLogFolder()
        {
            OpenFolder(PathsInfo.BepInExFolderPath);
        }

        public void OnClickShowBepInExFolder()
        {
            OpenFolder(PathsInfo.BepInExFolderPath);
        }

        private void OpenFolder(string folderPath)
        {
            if (Directory.Exists(folderPath))
            {
                var processStartInfo = new ProcessStartInfo();
                processStartInfo.Arguments = folderPath;

                if (PlatformInfo.IsWindows)
                {
                    processStartInfo.FileName = "explorer.exe";
                }

                Process.Start(processStartInfo);
            }
            else
            {
                Debug.Message($"{folderPath} Directory does not exist!");
            }
        }
    }
}
