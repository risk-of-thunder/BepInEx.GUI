using System;
using System.Runtime.InteropServices;

namespace BepInEx.GUI
{
    internal class Debug
    {
        [DllImport("user32.dll", SetLastError = true, CharSet = CharSet.Auto)]
        private static extern int MessageBox(IntPtr hWnd, string text, string caption, uint type);

        internal static void Message(string text)
        {
            MessageBox(IntPtr.Zero, text, "Debug", 0);
        }
    }
}
