using BepInEx.GUI.Models;
using ReactiveUI;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Diagnostics;
using System.IO;
using System.Windows.Input;

namespace BepInEx.GUI.ViewModels
{
    public class GeneralViewModel : ViewModelBase
    {
        public PathsInfo PathsInfo { get; }

        public string TargetIsLoadingCanCloseWindow { get; }

        public string LoadedModCountText { get; }
        public ObservableCollection<Mod> Mods { get; }

        public PlatformInfo PlatformInfo { get; }

        public ICommand OnClickOpenGameFolderCommand { get; private set; }
        public ICommand OnClickShowLogFolderCommand { get; private set; }
        public ICommand OnClickShowBepInExFolderCommand { get; private set; }

        public GeneralViewModel(PathsInfo pathsInfo, PlatformInfo platformInfo)
        {
            PathsInfo = pathsInfo;

            TargetIsLoadingCanCloseWindow = $"{pathsInfo.ProcessName} is loading, you can safely close this window.";

            var mods = new List<Mod>();

            mods.Add(new Mod("qsdsqd"));

            LoadedModCountText = $"Loaded Mods: {mods.Count}";
            Mods = new ObservableCollection<Mod>(mods);

            PlatformInfo = platformInfo;

            SetButtonCommands();
        }

        private void SetButtonCommands()
        {
            OnClickOpenGameFolderCommand = ReactiveCommand.Create(OnClickOpenGameFolder);
            OnClickShowLogFolderCommand = ReactiveCommand.Create(OnClickShowLogFolder);
            OnClickShowBepInExFolderCommand = ReactiveCommand.Create(OnClickShowBepInExFolder);
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
