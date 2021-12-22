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
                    var args = DefaultArgsIfNoneProvided(eventArgs.Args);

                    var platformInfo = new PlatformInfo(args);
                    var pathsInfo = new PathsInfo(args);

                    desktop.MainWindow = new MainWindow
                    {
                        DataContext = new MainWindowViewModel(pathsInfo, platformInfo),
                    };
                };       
            }

            base.OnFrameworkInitializationCompleted();
        }

        private static string[] DefaultArgsIfNoneProvided(string[] args)
        {
            if (args.Length == 0)
            {
                args = new string[]
                {
                    (0x25 | 0x2).ToString(), // win64
                    "Unknown Version",
                    "Unknown Target",

                    string.Empty,
                    string.Empty,
                    string.Empty,
                };
            }

            return args;
        }
    }
}
