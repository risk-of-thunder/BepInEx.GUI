using BepInEx.GUI.Config;
using System.IO;

namespace BepInEx.GUI.Models
{
    public class PathsInfo
    {
        public string? BepInExVersion { get; }

        public string ProcessName { get; }

        public string BepInExFolderPath { get; }

        public string ConfigFolderPath { get; }
        public string ConfigFilePath { get; }

        public string GameFolderPath { get; }

        public PathsInfo(string[] args)
        {
            BepInExVersion = args[1];
            ProcessName = args[2];

            BepInExFolderPath = args[3];
            ConfigFolderPath = args[4];
            GameFolderPath = args[5];

            ConfigFilePath = Path.Combine(ConfigFolderPath, MainConfig.FileName);
        }
    }
}
