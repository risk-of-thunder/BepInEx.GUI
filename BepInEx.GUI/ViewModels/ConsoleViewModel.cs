using BepInEx.GUI.Models;
using BepInEx.GUI.Views;
using ReactiveUI;
using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;

namespace BepInEx.GUI.ViewModels
{
    public class ConsoleViewModel : ViewModelBase
    {
        public class ColoredEntry
        {
            public string Text { get; set; }
            public string BackgroundColor { get; set; }
            public string ForegroundColor { get; set; }

            public ColoredEntry(string text, string backgroundColor, string foregroundColor)
            {
                Text = text;
                BackgroundColor = backgroundColor;
                ForegroundColor = foregroundColor;
            }
        }

        public LogSocketClient LogSocketClient { get; }

        public TargetInfo TargetInfo { get; }

        public PlatformInfo PlatformInfo { get; }

        public List<LogEntry> LogEntries { get; }

        private string _pauseButtonText = "Pause Game";
        public string PauseButtonText
        {
            get { return _pauseButtonText; }
            set
            {
                this.RaiseAndSetIfChanged(ref _pauseButtonText, value);
            }
        }

        private bool _consoleAutoScroll = true;
        public bool ConsoleAutoScroll
        {
            get { return _consoleAutoScroll; }
            set
            {
                ConsoleView.ConsoleAutoScroll = value;
                this.RaiseAndSetIfChanged(ref _consoleAutoScroll, value);

                if (ConsoleView.ConsoleAutoScroll)
                    ConsoleView.ScrollToEnd();
            }
        }

        private bool _justChangedSelectedMod;

        private string _textFilter = "";
        public string TextFilter
        {
            get { return _textFilter; }
            set
            {
                this.RaiseAndSetIfChanged(ref _textFilter, value);
                UpdateConsoleBox();

                if (_justChangedSelectedMod)
                {
                    _justChangedSelectedMod = false;
                }
                else
                {
                    SelectedModFilter = null;
                }
            }
        }

        private Mod? _selectedModFilter;
        public Mod? SelectedModFilter
        {
            get { return _selectedModFilter; }
            set
            {
                this.RaiseAndSetIfChanged(ref _selectedModFilter, value);

                if (value != null)
                {
                    _justChangedSelectedMod = true;
                    TextFilter = _selectedModFilter!.Name;
                }
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
                LogFilterLevelText = "Log Filter Level : " + _allowedLogLevel.ToString();
                UpdateConsoleBox();
            }
        }

        private string _logFilterLevelText = "Log Filter Level : " + Logging.LogLevel.All;
        public string LogFilterLevelText
        {
            get { return _logFilterLevelText; }
            set
            {
                this.RaiseAndSetIfChanged(ref _logFilterLevelText, value);
            }
        }

        public ConsoleViewModel(LogSocketClient socketClient, TargetInfo targetInfo, PlatformInfo platformInfo)
        {
            LogEntries = new();

            LogSocketClient = socketClient;

            LogEntries.AddRange(LogSocketClient.PastLogs);

            LogSocketClient.OnLogEntry += AddLogToConsole;

            TargetInfo = targetInfo;

            PlatformInfo = platformInfo;
        }

        private void AddLogToConsole(LogEntry logEntry)
        {
            lock (LogEntries)
            {
                LogEntries.Add(logEntry);
            }

            UpdateConsoleBox();
        }

        private void UpdateConsoleBox()
        {
            var consoleText = new ObservableCollection<ColoredEntry>();

            lock (LogEntries)
            {
                foreach (var logEntry in LogEntries)
                {
                    if (logEntry.LevelCode <= _allowedLogLevel)
                    {
                        var logEntryString = logEntry.ToString();

                        var (backgroundColor, foregroundColor) = logEntry.LevelCode switch
                        {
                            Logging.LogLevel.Fatal => ("Transparent", "Red"),
                            Logging.LogLevel.Error => ("Transparent", "Red"),
                            Logging.LogLevel.Warning => ("Transparent", "Yellow"),
                            _ => ("Transparent", "White"),
                        };

                        if (TextFilter.Length > 0)
                        {
                            if (logEntryString.ToLowerInvariant().Contains(TextFilter.ToLowerInvariant()))
                            {
                                consoleText.Add(new ColoredEntry(logEntryString, backgroundColor, foregroundColor));
                            }
                        }
                        else
                        {
                            consoleText.Add(new ColoredEntry(logEntryString, backgroundColor, foregroundColor));
                        }
                    }
                }
            }

            lock (ConsoleText)
            {
                ConsoleText = consoleText;
            }
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
                    PauseButtonText = "Pause Game";
                }
                else
                {
                    TargetInfo.Process.Suspend();
                    PauseButtonText = "Resume Game";
                }

                _isTargetPaused = !_isTargetPaused;
            }
        }

        public void OnClickClearTextFilter()
        {
            TextFilter = "";
        }
    }
}
