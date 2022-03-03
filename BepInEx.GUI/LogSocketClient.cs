using BepInEx.GUI.Models;
using ProtoBuf;
using System;
using System.Collections.Generic;
using System.Net;
using System.Net.Sockets;
using System.Threading;

namespace BepInEx.GUI
{
    public class LogSocketClient : IDisposable
    {
        private readonly Thread? _thread;

        private readonly int _port;

        public PathsInfo? PathsInfo { get; private set; }

        private TcpClient? _client;
        private NetworkStream? _stream;

        private bool _disposed;

        private readonly Queue<LogEntry> _logs = new();
        public Queue<LogEntry> PastLogs => new(FixAggregatorEntries());

        // The start of the list contains out of order entries for some reason ?
        private List<LogEntry> FixAggregatorEntries()
        {
            var fixedLogEntries = new List<LogEntry>();

            bool gotFirstLog = false;
            foreach (var log in _logs)
            {
                if (!gotFirstLog)
                {
                    gotFirstLog = IsFirstLogEntry(log);
                    if (gotFirstLog)
                        fixedLogEntries.Add(log);
                }
                else
                {
                    fixedLogEntries.Add(log);
                }
            }

            return fixedLogEntries;
        }

        private bool IsFirstLogEntry(LogEntry log)
        {
            bool isFirstLogEntry = false;

            if (log.Source == "BepInEx" && log.ToString().Contains(PathsInfo!.BepInExVersion!))
            {
                isFirstLogEntry = true;
            }

            return isFirstLogEntry;
        }

        public Action<LogEntry>? OnLogEntry { get; set; }

        public LogSocketClient(string[] args, PathsInfo pathsInfo)
        {
            if (int.TryParse(args[7], out var port))
            {
                _port = port;

                PathsInfo = pathsInfo;

                _thread = new Thread(new ThreadStart(ClientThread))
                {
                    IsBackground = true
                };
                _thread.Start();
            }
            else
                Debug.Message("Error parsing args[7] for socket port");
        }

        private void ClientThread()
        {
            try
            {
                InitClient();
                GetLogsFromServer();
            }
            catch (Exception)
            {
                // No console logs for you

                // Todo save logs somewhere : File.Append (bepinexGuiAppDataFolderPath);
            }
            finally
            {
                Dispose();
            }
        }

        private void InitClient()
        {
            _client = new TcpClient();
            _client.Connect(new IPEndPoint(IPAddress.Loopback, _port));
            _stream = _client.GetStream();
        }

        private void GetLogsFromServer()
        {
            while (true)
            {
                if (_disposed)
                {
                    return;
                }

                try
                {
                    lock (_stream!)
                    {
                        if (_stream.DataAvailable)
                        {
                            var logEntry = Serializer.DeserializeWithLengthPrefix<LogEntry>(_stream, PrefixStyle.Base128);

                            OnLogEntry?.Invoke(logEntry);

                            _logs.Enqueue(logEntry);
                        }
                    }
                }
                catch (Exception)
                {
                    // stream / client could be disposed because of multi threading
                    // IO Exception can also happens when user close GUI
                }

                Thread.Yield();
            }
        }

        protected virtual void Dispose(bool disposing)
        {
            if (!_disposed)
            {
                if (disposing)
                {
                    lock (_stream!)
                    {
                        lock (_client!)
                        {
                            _stream?.Dispose();
                            _client?.Dispose();
                        }
                    }
                }

                // TODO: free unmanaged resources (unmanaged objects) and override finalizer
                // TODO: set large fields to null
                _disposed = true;
            }
        }

        public void Dispose()
        {
            // Do not change this code. Put cleanup code in 'Dispose(bool disposing)' method
            Dispose(disposing: true);
            GC.SuppressFinalize(this);
        }
    }
}
