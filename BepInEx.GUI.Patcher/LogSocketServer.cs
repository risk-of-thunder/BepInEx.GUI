using ProtoBuf;
using System;
using System.Collections.Generic;
using System.Net;
using System.Net.Sockets;
using System.Threading;

namespace BepInEx.GUI.Patcher
{
    public class LogSocketServer : IDisposable
    {
        private readonly Thread _thread;

        private readonly int _port;
        private TcpListener _listener;

        private TcpClient _client;
        private NetworkStream _stream;

        private bool _disposed;

        public Queue<LogEntry> LogQueue { get; set; } = new();

        public LogSocketServer(int port)
        {
            _port = port;

            _thread = new Thread(new ThreadStart(ServerThread))
            {
                IsBackground = true
            };
            _thread.Start();
        }

        private void ServerThread()
        {
            try
            {
                InitServer();
                GetClientData();
                SendLogsToClient();
            }
            catch (Exception e)
            {
                Patcher.LogSource.LogError(e.ToString());
            }
            finally
            {
                Dispose();
            }
        }

        private void InitServer()
        {
            _listener = new TcpListener(new IPEndPoint(IPAddress.Loopback, _port));
            _listener.Start();
        }

        private void SendLogsToClient()
        {
            while (true)
            {
                if (_disposed)
                {
                    Patcher.LogSource.LogInfo("Closing log server socket.");
                    return;
                }

                try
                {
                    lock (_stream)
                    {
                        lock (_client)
                        {
                            if (_client.Connected)
                            {
                                if (LogQueue != null)
                                {
                                    lock (LogQueue)
                                    {
                                        while (LogQueue.Count > 0)
                                        {
                                            LogEntry log = LogQueue.Dequeue();

                                            Serializer.SerializeWithLengthPrefix(_stream, log, PrefixStyle.Base128);
                                        }
                                    }
                                }
                            }
                            else
                            {
                                Dispose();
                            }
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

        private void GetClientData()
        {
            _client = _listener.AcceptTcpClient();
            _stream = _client.GetStream();
        }

        protected virtual void Dispose(bool disposing)
        {
            if (!_disposed)
            {
                if (disposing)
                {
                    lock (_stream)
                    {
                        lock (_client)
                        {
                            _stream?.Dispose();
                            _client?.Dispose();
                        }
                    }

                    lock (LogQueue)
                    {
                        LogQueue = null;
                    }
                }

                _disposed = true;
            }
        }

        public void Dispose()
        {
            // Do not change this code. Put cleanup code in 'Dispose(bool disposing)' method
            Dispose(disposing: true);
            GC.SuppressFinalize(this);
        }

        internal static int FindFreePort()
        {
            int port = 0;
            Socket socket = new(AddressFamily.InterNetwork, SocketType.Stream, ProtocolType.Tcp);
            try
            {
                IPEndPoint localEP = new(IPAddress.Any, 0);
                socket.Bind(localEP);
                localEP = (IPEndPoint)socket.LocalEndPoint;
                port = localEP.Port;
            }
            finally
            {
                socket.Close();
            }

            return port;
        }
    }
}
