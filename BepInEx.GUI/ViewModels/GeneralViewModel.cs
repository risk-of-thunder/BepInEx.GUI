using BepInEx.GUI.Models;
using System.Collections.Generic;
using System.Collections.ObjectModel;

namespace BepInEx.GUI.ViewModels
{
    public class GeneralViewModel : ViewModelBase
    {
        public string TargetIsLoadingCanCloseWindow { get; }

        public string LoadedModCountText { get; }
        public ObservableCollection<Mod> Mods { get; }

        public GeneralViewModel(PathsInfo pathsInfo)
        {
            TargetIsLoadingCanCloseWindow = $"{pathsInfo.ProcessName} is loading, you can safely close this window.";

            var mods = new List<Mod>();

            mods.Add(new Mod("qsdsqd"));

            LoadedModCountText = $"Loaded Mods: {mods.Count}";
            Mods = new ObservableCollection<Mod>(mods);
        }
    }
}
