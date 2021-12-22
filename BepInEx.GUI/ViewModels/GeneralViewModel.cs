using BepInEx.GUI.Models;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Diagnostics;
using System.IO;

namespace BepInEx.GUI.ViewModels
{
    public class GeneralViewModel : ViewModelBase
    {
        public PathsInfo PathsInfo { get; }

        public string TargetIsLoadingCanCloseWindow { get; }

        public string LoadedModCountText { get; }
        public ObservableCollection<Mod> Mods { get; }

        public PlatformInfo PlatformInfo { get; }

        public GeneralViewModel(PathsInfo pathsInfo, PlatformInfo platformInfo)
        {
            PathsInfo = pathsInfo;

            TargetIsLoadingCanCloseWindow = $"{pathsInfo.ProcessName} is loading, you can safely close this window.";

            var mods = new List<Mod>();

            mods.Add(new Mod("qsdsqd"));

            LoadedModCountText = $"Loaded Mods: {mods.Count}";
            Mods = new ObservableCollection<Mod>(mods);

            PlatformInfo = platformInfo;
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
