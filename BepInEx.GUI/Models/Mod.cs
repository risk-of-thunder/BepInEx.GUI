namespace BepInEx.GUI.Models
{
    public class Mod
    {
        public string Name { get; }
        public string Version { get; }

        public Mod(string name, string version)
        {
            Name = name;
            Version = version;
        }
    }
}
