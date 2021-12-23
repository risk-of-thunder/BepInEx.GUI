using BepInEx.GUI.Models;
using ReactiveUI;
using System;
using System.Collections.Generic;
using System.Text;
using WebSocketSharp;

namespace BepInEx.GUI.ViewModels
{
    public class ConsoleViewModel : ViewModelBase
    {
        public WebSocket WebSocket { get; }

        public TargetInfo TargetInfo { get; }

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

        private string _consoleText = "";
        public string ConsoleText
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

        public ConsoleViewModel(WebSocket webSocket, TargetInfo targetInfo)
        {
            WebSocket = webSocket;
            WebSocket.OnMessage += AddLogToConsole;

            TargetInfo = targetInfo;

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
            var stringBuilder = new StringBuilder();

            foreach (var logEntry in LogEntries)
            {
                if (logEntry.LevelCode <= _allowedLogLevel)
                {
                    if (TextFilter.Length > 0)
                    {
                        var logEntryString = logEntry.ToString();
                        if (logEntryString.ToLowerInvariant().Contains(TextFilter.ToLowerInvariant()))
                        {
                            stringBuilder.AppendLine(logEntryString);
                        }
                    }
                    else
                    {
                        stringBuilder.AppendLine(logEntry.ToString());
                    }
                }
            }

            ConsoleText = stringBuilder.ToString();
        }

        private bool _isTargetPaused;
        public void OnClickPauseGame()
        {
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
