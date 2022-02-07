using BepInEx.GUI.Models;
using ReactiveUI;
using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using WebSocketSharp;

namespace BepInEx.GUI.ViewModels
{
    public class ConsoleViewModel : ViewModelBase
    {
        public class ColoredEntry
        {
            public string Text { get; set; }
            public string Color { get; set; }

            public ColoredEntry(string text, string color)
            {
                Text = text;
                Color = color;
            }
        }

        public WebSocket WebSocket { get; }

        public TargetInfo TargetInfo { get; }

        public PlatformInfo PlatformInfo { get; }

        public List<LogEntry> LogEntries { get; }

        private string _textFilter = "";
        public string TextFilter
        {
            get { return _textFilter; }
            set
            {
                this.RaiseAndSetIfChanged(ref _textFilter, value);
                UpdateConsoleBox();
            }
        }

        private ObservableCollection<ColoredEntry> _consoleText = new();
        public ObservableCollection<ColoredEntry> ConsoleText
        {
            get { return _consoleText; }
            set
            {
                this.RaiseAndSetIfChanged(ref _consoleText, value);
            }
        }

        private Logging.LogLevel[] _logLevels = (Logging.LogLevel[])Enum.GetValues(typeof(Logging.LogLevel));
        private Logging.LogLevel _allowedLogLevel = Logging.LogLevel.All;
        private int _logFilterLevel = 7;
        public int LogFilterLevel
        {
            get { return _logFilterLevel; }
            set
            {
                this.RaiseAndSetIfChanged(ref _logFilterLevel, value);
                _allowedLogLevel = _logLevels[_logFilterLevel];
                UpdateConsoleBox();
            }
        }

        public ConsoleViewModel(WebSocket webSocket, TargetInfo targetInfo, PlatformInfo platformInfo)
        {
            WebSocket = webSocket;
            WebSocket.OnMessage += AddLogToConsole;

            TargetInfo = targetInfo;

            PlatformInfo = platformInfo;

            LogEntries = new();
        }

        private void AddLogToConsole(object? sender, MessageEventArgs e)
        {
            var logEntry = LogEntry.Deserialize(e.RawData);
            if (logEntry != null)
            {
                LogEntries.Add(logEntry);
                UpdateConsoleBox();
            }
        }

        private void UpdateConsoleBox()
        {
            var consoleText = new ObservableCollection<ColoredEntry>();

            foreach (var logEntry in LogEntries)
            {
                if (logEntry.LevelCode <= _allowedLogLevel)
                {
                    var logEntryString = logEntry.ToString();
                    string color = logEntry.LevelCode switch
                    {
                        Logging.LogLevel.Fatal => "Red",
                        Logging.LogLevel.Error => "Red",
                        Logging.LogLevel.Warning => "YellowGreen",
                        _ => "Transparent",
                    };
                    if (TextFilter.Length > 0)
                    {
                        if (logEntryString.ToLowerInvariant().Contains(TextFilter.ToLowerInvariant()))
                        {
                            consoleText.Add(new ColoredEntry(logEntryString, color));
                        }
                    }
                    else
                    {
                        consoleText.Add(new ColoredEntry(logEntryString, color));
                    }
                }
            }

            ConsoleText = consoleText;
        }

        private bool _isTargetPaused;
        public void OnClickPauseGame()
        {
            if (!PlatformInfo.IsWindows)
            {
                Debug.Message("Windows only feature.");
                return;
            }

            if (TargetInfo.Process != null && TargetInfo.Id != 0 && !TargetInfo.Process.HasExited)
            {
                if (_isTargetPaused)
                {
                    TargetInfo.Process.Resume();
                }
                else
                {
                    TargetInfo.Process.Suspend();
                }

                _isTargetPaused = !_isTargetPaused;
            }
        }
    }
}
