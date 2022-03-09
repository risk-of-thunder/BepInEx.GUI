using Avalonia;
using Avalonia.Controls;
using Avalonia.Controls.ApplicationLifetimes;
using Avalonia.Markup.Xaml;
using Avalonia.Platform;
using BepInEx.GUI.Config;
using BepInEx.GUI.Models;
using BepInEx.GUI.ViewModels;
using BepInEx.GUI.Views;
using System;
using System.IO;
using System.Text.Json;

namespace BepInEx.GUI
{
    public class App : Application
    {
        private LogSocketClient? _socketClient;
        private readonly WindowPosAndSize _windowPosAndSize = new();

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
                    AppDomain.CurrentDomain.UnhandledException += ShowUnhandledException;

                    var args = DefaultArgsIfNoneProvided(eventArgs.Args);

                    var platformInfo = new PlatformInfo(args);
                    var pathsInfo = new PathsInfo(args);
                    var targetInfo = new TargetInfo(args);

                    _socketClient = new LogSocketClient(args, pathsInfo);

                    MainConfig.Init(pathsInfo.ConfigFilePath, true);

                    desktop.MainWindow = new MainWindow
                    {
                        DataContext = new MainWindowViewModel(_socketClient, pathsInfo, platformInfo, targetInfo),
                    };

                    desktop.MainWindow.PlatformImpl.PositionChanged += OnPositionChanged;
                    desktop.MainWindow.PlatformImpl.Resized += OnSizeChanged;

                    SetWindowPositionAndSizeFromFile(desktop.MainWindow);

                    desktop.Exit += OnExit;
                };       
            }

            base.OnFrameworkInitializationCompleted();
        }

        private void OnExit(object? sender, ControlledApplicationLifetimeExitEventArgs e)
        {
            _socketClient?.Dispose();

            SaveWindowPositionAndSize();
        }

        private void OnSizeChanged(Size arg1, PlatformResizeReason arg2)
        {
            _windowPosAndSize.ClientSizeWidth = arg1.Width;
            _windowPosAndSize.ClientSizeHeight = arg1.Height;
        }

        private void OnPositionChanged(PixelPoint obj)
        {
            if (!IsMinimizedOnWindows(obj.X, obj.Y))
            {
                _windowPosAndSize.PositionX = obj.X;
                _windowPosAndSize.PositionY = obj.Y;
            }
        }

        // https://devblogs.microsoft.com/oldnewthing/20041028-00/?p=37453
        private static bool IsMinimizedOnWindows(int x, int y)
        {
            return x <= -30000 && y <= -30000;
        }

        public class WindowPosAndSize
        {
            public int PositionX { get; set; }
            public int PositionY { get; set; }

            public double ClientSizeWidth { get; set; }
            public double ClientSizeHeight { get; set; }
        }

        private static void SetWindowPositionAndSizeFromFile(Window mainWindow)
        {
            try
            {
                if (File.Exists(MainConfig.BepinexGuiWindowSizePosFilePath))
                {
                    var w = JsonSerializer.Deserialize<WindowPosAndSize>(File.ReadAllText(MainConfig.BepinexGuiWindowSizePosFilePath))!;

                    if (!IsMinimizedOnWindows(w.PositionX, w.PositionY))
                    {
                        mainWindow.PlatformImpl.Move(new(w.PositionX, w.PositionY));
                        mainWindow.PlatformImpl.Resize(new(w.ClientSizeWidth, w.ClientSizeHeight));
                    }
                }
            }
            catch (Exception e)
            {
                // todo save log

                Debug.Message(e.ToString());
            }
        }

        private void SaveWindowPositionAndSize()
        {
            try
            {
                File.WriteAllText(MainConfig.BepinexGuiWindowSizePosFilePath, JsonSerializer.Serialize(_windowPosAndSize));
            }
            catch (Exception e)
            {
                // todo save log

                Debug.Message(e.ToString());
            }
        }

        private void ShowUnhandledException(object sender, UnhandledExceptionEventArgs e)
        {
            _socketClient?.Dispose();

            var ex = (Exception)e.ExceptionObject;
            Debug.Message(ex.ToString());
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
                    string.Empty,
                    "27090"
                };
            }

            return args;
        }
    }
}
