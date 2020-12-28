using Mono.Cecil;
using System.Reflection;
using System.IO;
using System.Linq;
using System.Collections.Generic;
using BepInEx.Logging;

namespace BepInEx.GUI
{
    public static class GuiPatcher
    {
        public static IEnumerable<string> TargetDLLs => Enumerable.Empty<string>();
        public static void Patch(AssemblyDefinition _) { }

        internal static System.Diagnostics.Process process;

        public static void Initialize()
        {
            var executable = Path.Combine(Path.GetDirectoryName(Assembly.GetExecutingAssembly().Location), "BepInEx.GUI.SplashGUI.exe");
            process = System.Diagnostics.Process.Start(executable);

            Logger.Listeners.Add(new LogListener());
        }

        private class LogListener : ILogListener
        {
            public void Dispose() { }

            public void LogEvent(object sender, LogEventArgs eventArgs)
            {
                if(eventArgs.Data.ToString().Equals("Chainloader started") && eventArgs.Level.Equals(LogLevel.Message))
                {
                    Logger.Listeners.Remove(this);
                    process.Kill();
                }
            }
        }
    }
}
