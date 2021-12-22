using Avalonia;
using Avalonia.Controls.ApplicationLifetimes;
using Avalonia.Markup.Xaml;
using BepInEx.GUI.Models;
using BepInEx.GUI.ViewModels;
using BepInEx.GUI.Views;

namespace BepInEx.GUI
{
    public class App : Application
    {
        public override void Initialize()
        {
            AvaloniaXamlLoader.Load(this);
        }

        public override void OnFrameworkInitializationCompleted()
        {
            if (ApplicationLifetime is IClassicDesktopStyleApplicationLifetime desktop)
            {
                desktop.Startup += (sender, eventArgs) =>
                {
                    var args = eventArgs.Args.Length == 0 ? new string[] { "Unknown Version", "Unknown Target" } : eventArgs.Args;

                    var bepInExVersion = args[0];
                    var processName = args[1];

                    var processTargetInfo = new PathsInfo(bepInExVersion, processName);

                    desktop.MainWindow = new MainWindow
                    {
                        DataContext = new MainWindowViewModel(processTargetInfo),
                    };
                };       
            }

            base.OnFrameworkInitializationCompleted();
        }
    }
}
