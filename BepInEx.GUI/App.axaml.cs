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
                    var pathsInfo = new PathsInfo(eventArgs.Args);

                    desktop.MainWindow = new MainWindow
                    {
                        DataContext = new MainWindowViewModel(pathsInfo),
                    };
                };       
            }

            base.OnFrameworkInitializationCompleted();
        }
    }
}
