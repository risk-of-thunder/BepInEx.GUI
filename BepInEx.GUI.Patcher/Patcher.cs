using BepInEx.Configuration;
using BepInEx.Logging;
using Mono.Cecil;
using MonoMod.Utils;
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
                var fileName = Path.GetFileName(filePath);

                const string GuiFileName = "BepInEx.GUI";

                const Platform windowsPlatform = Platform.Windows;
                const Platform windowsX64Platform = Platform.Windows | Platform.Bits64;

                const Platform linuxX64Platform = Platform.Linux | Platform.Bits64;

                const Platform macOsX64Platform = Platform.MacOS | Platform.Bits64;

                var platform = PlatformHelper.Current;

                var isWindows = (platform & windowsPlatform) == platform;
                var isWindows64 = (platform & windowsX64Platform) == platform;

                // linux x86 https://github.com/dotnet/runtime/issues/31180
                var isLinux64 = (platform & linuxX64Platform) == platform;

                var isMacOs64 = (platform & macOsX64Platform) == platform;

                var filePathLower = filePath.ToLowerInvariant();

                // Not the best but should work...
                if (
                    (isWindows && fileName == $"{GuiFileName}.exe" && filePathLower.Contains("86")) ||
                    (isWindows64 && fileName == $"{GuiFileName}.exe" && filePathLower.Contains("64")) ||

                    (isLinux64 && fileName == GuiFileName && filePathLower.Contains("linux64")) ||

                    (isMacOs64 && fileName == GuiFileName && filePathLower.Contains("macos64"))
                    )
                {
                    return filePath;
                }
            }

            return null;
        }

        private static void LaunchGui(string executablePath)
        {
            var processStartInfo = new ProcessStartInfo();
            processStartInfo.FileName = executablePath;
            processStartInfo.WorkingDirectory = Path.GetDirectoryName(executablePath);

            processStartInfo.Arguments =
                $"\"{PlatformHelper.Current}\" " +
                $"\"{typeof(Paths).Assembly.GetName().Version}\" " +
                $"\"{Paths.ProcessName}\" " +
                $"\"{Paths.BepInExRootPath}\" " +
                $"\"{Paths.ConfigPath}\" " +
                $"\"{Paths.GameRootPath}\" " +
                $"\"{Process.GetCurrentProcess().Id}\"";

            GuiProcess = Process.Start(processStartInfo);
            Logger.Listeners.Add(new CloseGuiOnChainloaderDone());
        }
    }
}
