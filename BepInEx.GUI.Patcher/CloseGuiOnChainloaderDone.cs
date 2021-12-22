using BepInEx.Logging;
using System;

namespace BepInEx.GUI.Patcher
{
    public class CloseGuiOnChainloaderDone : ILogListener
    {
        public void Dispose() { }

        public void LogEvent(object sender, LogEventArgs eventArgs)
        {
            if (eventArgs.Data.ToString() == "Chainloader startup complete" && eventArgs.Level.Equals(LogLevel.Message))
            {
                Patcher.LogSource.LogMessage("Closing BepInEx.GUI");

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
                Logger.Listeners.Remove(this);
                Patcher.LogSource.Dispose();
            }
        }
    }
}
