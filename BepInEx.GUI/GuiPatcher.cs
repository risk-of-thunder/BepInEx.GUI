using Mono.Cecil;
using System.Collections.Generic;
using System.Reflection;
using System.IO;
using System.Linq;
using BepInEx;

namespace BepInEx.GUI
{
    public static class GuiPatcher
    {
        public static IEnumerable<string> TargetDLLs => Enumerable.Empty<string>();
        public static void Patch(AssemblyDefinition _) { }

        public static void Initialize()
        {
            var executable = Path.Combine(Path.GetDirectoryName(Assembly.GetExecutingAssembly().Location), "GraphicalUserInterface.exe");
            System.Diagnostics.Process.Start(executable, "\"BepInEx - Risk of Rain 2\"");
        }
    }
}
