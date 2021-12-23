using BepInEx.GUI.Models;
using ReactiveUI;
using WebSocketSharp;

namespace BepInEx.GUI.ViewModels
{
    public class ConsoleViewModel : ViewModelBase
    {
        public WebSocket WebSocket { get; }

        public TargetInfo TargetInfo { get; }

        private string _textFilter = "";
        public string TextFilter
        {
            get { return _textFilter; }
            set
            {
                this.RaiseAndSetIfChanged(ref _textFilter, value);
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

        private int _logFilterLevel;
        public int LogFilterLevel
        {
            get { return _logFilterLevel; }
            set
            {
                this.RaiseAndSetIfChanged(ref _logFilterLevel, value);
            }
        }

        public ConsoleViewModel(WebSocket webSocket, TargetInfo targetInfo)
        {
            WebSocket = webSocket;
            WebSocket.OnMessage += AddLogToConsole;

            TargetInfo = targetInfo;
        }

        private void AddLogToConsole(object? sender, MessageEventArgs e)
        {
            // todo : store the entries in list so that we can apply the filters
            var logEntry = LogEntry.Deserialize(e.RawData);
            if (logEntry != null)
            {
                ConsoleText += logEntry.Data + "\n";
            }
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
