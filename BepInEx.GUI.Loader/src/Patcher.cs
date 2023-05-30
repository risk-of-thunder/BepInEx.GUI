global using System;
global using System.Collections.Generic;
global using System.Text;
using System.Diagnostics;
using System.IO;
using System.Net;
using System.Net.Sockets;
using BepInEx.Configuration;
using System.Reflection;
using BepInEx.Logging;
using Mono.Cecil;

namespace BepInEx.GUI.Loader;

internal static class Patcher
{
    public static IEnumerable<string> TargetDLLs { get; } = Array.Empty<string>();

    public static void Patch(AssemblyDefinition _) { }

    public static void Initialize()
    {
        Log.Init();

        try
        {
            InitializeInternal();
        }
        catch (Exception e)
        {
            Log.Error($"Failed to initialize : ({e.GetType()}) {e.Message}{Environment.NewLine}{e}");
        }
    }

    private static void InitializeInternal()
    {
        Config.Init(Paths.ConfigPath);

        var consoleConfig = (ConfigEntry<bool>)typeof(BepInPlugin).Assembly.
            GetType("BepInEx.ConsoleManager", true).
            GetField("ConfigConsoleEnabled",
            BindingFlags.Static | BindingFlags.Public).GetValue(null);

        if (consoleConfig.Value)
        {
            Log.Info("BepInEx regular console is enabled, aborting launch.");
        }
        else if (Config.EnableBepInExGUIConfig.Value)
        {
            FindAndLaunchGUI();
        }
        else
        {
            Log.Info("Custom BepInEx.GUI is disabled in the config, aborting launch.");
        }
    }

    private static string FindGUIExecutable()
    {
        foreach (var filePath in Directory.GetFiles(Paths.PatcherPluginPath, "*", SearchOption.AllDirectories))
        {
            var fileName = Path.GetFileName(filePath);

            const string GuiFileName = "bepinex_gui";

            // No platform check because proton is used for RoR2 and it handles it perfectly anyway:
            // It makes the Process.Start still goes through proton and makes the bep gui
            // that was compiled for Windows works fine even in linux operating systems.

            if (fileName == $"{GuiFileName}.exe")
            {
                Log.Info($"Found bepinex_gui executable in {filePath}");
                return filePath;
            }
        }

        return null;
    }

    private static void FindAndLaunchGUI()
    {
        Log.Info("Finding and launching GUI");

        var executablePath = FindGUIExecutable();
        if (executablePath != null)
        {
            var freePort = FindFreePort();
            var process = LaunchGUI(executablePath, freePort);
            if (process != null)
            {
                Logger.Listeners.Add(new SendLogToClientSocket(freePort));
                Logger.Listeners.Add(new CloseProcessOnChainloaderDone(process));
            }
            else
            {
                Log.Info("LaunchGUI failed");
            }
        }
        else
        {
            Log.Info("bepinex_gui executable not found.");
        }
    }

    private static int FindFreePort()
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

    private static Process LaunchGUI(string executablePath, int socketPort)
    {
        var processStartInfo = new ProcessStartInfo();
        processStartInfo.FileName = executablePath;
        processStartInfo.WorkingDirectory = Path.GetDirectoryName(executablePath);

        processStartInfo.Arguments =
            $"\"{typeof(Paths).Assembly.GetName().Version}\" " +
            $"\"{Paths.ProcessName}\" " +
            $"\"{Paths.GameRootPath}\" " +
            $"\"{GetLogOutputFilePath()}\" " +
            $"\"{Config.ConfigFilePath}\" " +
            $"\"{Process.GetCurrentProcess().Id}\" " +
            $"\"{socketPort}\"";

        return Process.Start(processStartInfo);
    }

    // Bad and hacky way to retrieve the correct log file path
    private static string GetLogOutputFilePath()
    {
        foreach (var logListener in Logger.Listeners)
        {
            if (logListener is DiskLogListener diskLogListener)
            {
                return diskLogListener.FileFullPath;
            }
        }

        return "";
    }
}
