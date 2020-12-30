using BepInEx.Configuration;
using BepInEx.Logging;
using Mono.Cecil;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Reflection;

namespace BepInEx.GUI
{
    public static class GuiPatcher
    {
        public static IEnumerable<string> TargetDLLs => Enumerable.Empty<string>();
        public static void Patch(AssemblyDefinition _) { }

        internal static System.Diagnostics.Process process;

        internal static bool quickFail = false;

        public static void Initialize()
        {
            var consoleConfig = (ConfigEntry<bool>)typeof(BepInPlugin).Assembly.GetType("BepInEx.ConsoleManager", true).GetField("ConfigConsoleEnabled", BindingFlags.Static | BindingFlags.Public).GetValue(null);
            if (consoleConfig.Value)
            {
                var logsrc = Logger.CreateLogSource("BepInEx.GUI");
                logsrc.LogMessage("Not showing a splash screen, because you can read this message!");
                logsrc.Dispose();
            }
            else
            {
                var executable = Path.Combine(Path.GetDirectoryName(Assembly.GetExecutingAssembly().Location), "BepInEx.GUI.SplashGUI.exe");
                process = System.Diagnostics.Process.Start(executable);
                Logger.Listeners.Add(new LogListener());
            }

        }

        private class LogListener : ILogListener
        {
            public void Dispose() { }

            public void LogEvent(object sender, LogEventArgs eventArgs)
            {
                if (quickFail)
                {
                    return;
                }
                if (eventArgs.Data.ToString().Equals("Chainloader startup complete") && eventArgs.Level.Equals(LogLevel.Message))
                {
                    var logsrc = Logger.CreateLogSource("BepInEx.GUI");
                    logsrc.LogMessage("Closing GUI");
                    quickFail = true;
                    try
                    {
                        process.Kill();
                    }
                    catch (Exception e)
                    {
                        logsrc.LogError(e.Message);
                        logsrc.LogError(e.StackTrace);
                    }
                    logsrc.Dispose();
                }
            }
        }
    }
}
