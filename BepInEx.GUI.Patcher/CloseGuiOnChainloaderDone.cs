using BepInEx.GUI.Config;
using BepInEx.Logging;
using System;
using System.IO;

namespace BepInEx.GUI.Patcher
{
    public class CloseGuiOnChainloaderDone : ILogListener
    {
        private bool Disposed;

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
                Patcher.LogSource.LogMessage("Closing BepInEx.GUI");

                MainConfig.Init(Path.Combine(Paths.ConfigPath, MainConfig.FileName));
                if (MainConfig.CloseWindowWhenGameLoadedConfig.Value)
                    Exit();
            }
        }

        private void Exit()
        {
            try
            {
                Patcher.GuiProcess.Kill();
            }
            catch (Exception e)
            {
                Patcher.LogSource.LogError(e.Message);
                Patcher.LogSource.LogError(e.StackTrace);
            }
            finally
            {
                Patcher.LogSource.Dispose();
            }
        }
    }
}
