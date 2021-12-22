using BepInEx.Configuration;
using BepInEx.Logging;
using Mono.Cecil;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Reflection;

namespace BepInEx.GUI.Patcher
{
    public static class Patcher
    {
        public static IEnumerable<string> TargetDLLs => Enumerable.Empty<string>();
        public static void Patch(AssemblyDefinition _) { }

        internal static ManualLogSource LogSource { get; private set; }

        internal static Process GuiProcess;

        public static void Initialize()
        {
            LogSource = Logger.CreateLogSource("BepInEx.GUI.Patcher");

            var consoleConfig = (ConfigEntry<bool>)typeof(BepInPlugin).Assembly.GetType("BepInEx.ConsoleManager", true).GetField("ConfigConsoleEnabled", BindingFlags.Static | BindingFlags.Public).GetValue(null);
            if (consoleConfig.Value)
            {
                LogSource.LogMessage("Console is enabled, not using BepInEx.GUI");
                LogSource.Dispose();
            }
            else
            {
                string executablePath = FindGuiExecutable();
                if (executablePath != null)
                {
                    LaunchGui(executablePath);
                }
                else
                {
                    LogSource.LogMessage("BepInEx.GUI executable not found.");
                    LogSource.Dispose();
                }
            }
        }

        private static string FindGuiExecutable()
        {
            foreach (var filePath in Directory.GetFiles(Paths.PatcherPluginPath, "*", SearchOption.AllDirectories))
            {
                var fileName = Path.GetFileNameWithoutExtension(filePath);
                if (fileName == "BepInEx.GUI")
                {
                    return filePath;
                }
            }

            return null;
        }

        private static void LaunchGui(string executablePath)
        {
            var processStartInfo = new ProcessStartInfo();
            processStartInfo.WorkingDirectory = Assembly.GetExecutingAssembly().Location;

            processStartInfo.Arguments =
                $"{typeof(Paths).Assembly.GetName().Version} " +
                $"{Paths.ProcessName} " +
                $"{Paths.ConfigPath}";

            GuiProcess = Process.Start(executablePath);
            Logger.Listeners.Add(new CloseGuiOnChainloaderDone());
        }
    }
}
