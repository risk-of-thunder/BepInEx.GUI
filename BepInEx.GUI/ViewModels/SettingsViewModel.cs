using Avalonia;
using Avalonia.Controls.ApplicationLifetimes;
using BepInEx.GUI.Config;
using BepInEx.GUI.Models;
using ReactiveUI;
using System.Threading;
using System.Threading.Tasks;

namespace BepInEx.GUI.ViewModels
{
    public class SettingsViewModel : ViewModelBase
    {
        public TargetInfo TargetInfo { get; }

        private bool _enableDeveloperTools;
        public bool EnableDeveloperTools
        {
            get { return _enableDeveloperTools; }
            set
            {
                MainConfig.EnableDeveloperToolsConfig.Value = this.RaiseAndSetIfChanged(ref _enableDeveloperTools, value);
                MainConfig.File.Save();
            }
        }
        
        private bool _closeWindowWhenGameLoaded;
        public bool CloseWindowWhenGameLoaded
        {
            get { return _closeWindowWhenGameLoaded; }
            set
            {
                MainConfig.CloseWindowWhenGameLoadedConfig.Value = this.RaiseAndSetIfChanged(ref _closeWindowWhenGameLoaded, value);
                MainConfig.File.Save();
            }
        }

        private bool _closeWindowWhenGameCloses;
        public bool CloseWindowWhenGameCloses
        {
            get { return _closeWindowWhenGameCloses; }
            set
            {
                MainConfig.CloseWindowWhenGameClosesConfig.Value = this.RaiseAndSetIfChanged(ref _closeWindowWhenGameCloses, value);
                MainConfig.File.Save();
            }
        }

        private bool _enableBepInExGUI;
        public bool EnableBepInExGUI
        {
            get { return _enableBepInExGUI; }
            set
            {
                MainConfig.EnableBepInExGUIConfig.Value = this.RaiseAndSetIfChanged(ref _enableBepInExGUI, value);
                MainConfig.File.Save();
            }
        }

        public CancellationTokenSource CancellationTokenSource { get; private set; }

#pragma warning disable CS8618 // Non-nullable field must contain a non-null value when exiting constructor. Consider declaring as nullable.
        // dumb af compiler
        public SettingsViewModel(TargetInfo targetInfo)
#pragma warning restore CS8618 // Non-nullable field must contain a non-null value when exiting constructor. Consider declaring as nullable.
        {
            TargetInfo = targetInfo;

            SetConfigBindings();

            InitBackgroundTask();
        }

        private void SetConfigBindings()
        {
            EnableDeveloperTools = MainConfig.EnableDeveloperToolsConfig.Value;

            CloseWindowWhenGameLoaded = MainConfig.CloseWindowWhenGameLoadedConfig.Value;

            CloseWindowWhenGameCloses = MainConfig.CloseWindowWhenGameClosesConfig.Value;

            EnableBepInExGUI = MainConfig.EnableBepInExGUIConfig.Value;
        }

        private void InitBackgroundTask()
        {
            if (TargetInfo.Process == null)
            {
                return;
            }

            CancellationTokenSource = new CancellationTokenSource();

            _ = CheckGameIsClosed(CancellationTokenSource.Token);
        }

        private async Task CheckGameIsClosed(CancellationToken cancel)
        {
            while (true)
            {
                if (cancel.IsCancellationRequested)
                {
                    return;
                }

                if (MainConfig.CloseWindowWhenGameClosesConfig.Value && TargetInfo.Process.HasExited)
                {
                    ((IClassicDesktopStyleApplicationLifetime)Application.Current.ApplicationLifetime).Shutdown();
                }

                await Task.Delay(500);
            }
        }
    }
}
