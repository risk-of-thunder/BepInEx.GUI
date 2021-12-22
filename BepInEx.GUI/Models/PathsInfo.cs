namespace BepInEx.GUI.Models
{
    public class PathsInfo
    {
        public string? BepInExVersion { get; }

        public string ProcessName { get; }

        public string ConfigFolderPath { get; }

        public PathsInfo(string[] args)
        {
            args = DefaultIfNoneProvided(args);

            BepInExVersion = args[0];
            ProcessName = args[1];
            ConfigFolderPath = args[2];
        }

        private static string[] DefaultIfNoneProvided(string[] args)
        {
            if (args.Length == 0)
            {
                args = new string[]
                {
                    "Unknown Version",
                    "Unknown Target",
                    string.Empty
                };
            }

            return args;
        }
    }
}
