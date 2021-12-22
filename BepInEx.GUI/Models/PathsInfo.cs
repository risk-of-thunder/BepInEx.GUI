namespace BepInEx.GUI.Models
{
    public class PathsInfo
    {
        public object BepInExVersion { get; }

        public string ProcessName { get; }

        public PathsInfo(string bepInExVersion, string processName)
        {
            BepInExVersion = bepInExVersion;
            ProcessName = processName;
        }
    }
}
