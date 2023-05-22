using System.Net;
using System.Net.Sockets;
using System.Threading;
using BepInEx.Logging;

namespace BepInEx.GUI.Loader;

internal class SendLogToClientSocket : ILogListener
{
    private int _freePort;

    private readonly Thread _thread;

    private readonly Queue<LogEventArgs> _logQueue = new();
    private readonly ManualResetEvent _signal = new(false);

    internal static bool Stop;

    internal SendLogToClientSocket(int freePort)
    {
        _freePort = freePort;

        _thread = new Thread(() =>
        {
            var ipAddress = IPAddress.Parse("127.0.0.1");

            var listener = new TcpListener(ipAddress, _freePort);

            listener.Start();

            while (true)
            {
                Log.Info($"[SendLogToClient] Accepting Socket.");
                var clientSocket = listener.AcceptSocket();

                if (Stop)
                {
                    break;
                }

                SendPacketsToClientUntilConnectionIsClosed(clientSocket);
            }
        });

        _thread.Start();
    }

    private void SendPacketsToClientUntilConnectionIsClosed(Socket clientSocket)
    {
        while (true)
        {
            if (Stop)
            {
                break;
            }

            _signal.WaitOne();

            while (_logQueue.Count > 0)
            {
                var log = _logQueue.Peek();
                var logPacket = new LogPacket(log);

                try
                {
                    clientSocket.Send(logPacket.Bytes);
                }
                catch (Exception e)
                {
                    Log.Error($"Error while trying to send log to socket: {e}{Environment.NewLine}Disconnecting socket.");
                    return;
                }

                _ = _logQueue.Dequeue();
            }

            _signal.Reset();
        }
    }

    public void Dispose()
    {

    }

    internal void StoreLog(LogEventArgs eventArgs)
    {
        _logQueue.Enqueue(eventArgs);
        _signal.Set();
    }

    private bool _gotFirstLog = false;
    public void LogEvent(object sender, LogEventArgs eventArgs)
    {
        if (Stop)
        {
            return;
        }

        if (eventArgs.Data == null)
        {
            return;
        }

        if (!_gotFirstLog)
        {
            if (eventArgs.Level == LogLevel.Message &&
                eventArgs.Source.SourceName == "BepInEx" &&
                eventArgs.Data.ToString().StartsWith("BepInEx"))
            {
                _gotFirstLog = true;
            }
        }
        else
        {
            StoreLog(eventArgs);
        }
    }
}
