using System.Diagnostics;
using BepInEx.Logging;

namespace BepInEx.GUI.Loader;

public class CloseProcessOnChainloaderDone : ILogListener
{
    private bool Disposed;

    private Process _process;

    public CloseProcessOnChainloaderDone(Process process) => _process = process;

    public void Dispose()
    {
        Disposed = true;
    }

    public void LogEvent(object sender, LogEventArgs eventArgs)
    {
        if (Disposed)
        {
            return;
        }

        if (eventArgs.Data.ToString() == "Chainloader startup complete" && eventArgs.Level.Equals(LogLevel.Message))
        {
            if (Config.CloseWindowWhenGameLoadedConfig.Value)
            {
                Log.Message("Closing BepInEx.GUI");
                KillBepInExGUIProcess();
            }
        }
    }

    private void KillBepInExGUIProcess()
    {
        try
        {
            _process.Kill();
        }
        catch (Exception e)
        {
            Log.Error($"Error while trying to kill BepInEx GUI Process: {e}");
            Log.Error(e);
        }
        finally
        {
            SendLogToClientSocket.Stop = true;
            Dispose();
        }
    }
}
