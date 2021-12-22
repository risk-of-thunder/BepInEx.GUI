namespace BepInEx.GUI.Models
{
    public class WindowInfo
    {
        public string Title { get; }

        public WindowInfo(PathsInfo pathsInfo)
        {
            Title = $"BepInEx {pathsInfo.BepInExVersion} - {pathsInfo.ProcessName}";
        }
    }
}
