using System.Diagnostics;

namespace BepInEx.GUI.Models
{
    public class TargetInfo
    {
        public int Id { get; }

        public Process Process { get; }

        public TargetInfo(string[] args)
        {
            if (int.TryParse(args[6], out var id));
                Id = id;

            Process = Process.GetProcessById(Id);
        }
    }
}
