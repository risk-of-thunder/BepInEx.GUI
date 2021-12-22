using MonoMod.Utils;
using System;

namespace MonoMod.Utils
{
    //
    // Summary:
    //     Generic platform enum.
    [Flags]
    public enum _Platform
    {
        //
        // Summary:
        //     Bit applied to all OSes (Unknown, Windows, MacOS, ...).
        OS = 0x1,
        //
        // Summary:
        //     On demand 64-bit platform bit.
        Bits64 = 0x2,
        //
        // Summary:
        //     Applied to all NT and NT-oid platforms (Windows).
        NT = 0x4,
        //
        // Summary:
        //     Applied to all Unix and Unix-oid platforms (macOS, Linux, ...).
        Unix = 0x8,
        //
        // Summary:
        //     On demand ARM platform bit.
        ARM = 0x10000,
        //
        // Summary:
        //     Unknown OS.
        Unknown = 0x11,
        //
        // Summary:
        //     Windows, using the NT kernel.
        Windows = 0x25,
        //
        // Summary:
        //     macOS, using the Darwin kernel.
        MacOS = 0x49,
        //
        // Summary:
        //     Linux.
        Linux = 0x89,
        //
        // Summary:
        //     Android, using the Linux kernel.
        Android = 0x189,
        //
        // Summary:
        //     iOS, sharing components with macOS.
        iOS = 0x249
    }
}

namespace BepInEx.GUI.Models
{
    public class PlatformInfo
    {
        public _Platform Current { get; }

        public bool IsWindows { get; }
        public bool IsLinux { get; }
        public bool IsMacOs { get; }
        
        public PlatformInfo(string[] args)
        {
            const _Platform windowsX64Platform = _Platform.Windows | _Platform.Bits64;
            const _Platform linuxX64Platform = _Platform.Linux | _Platform.Bits64;
            const _Platform macOsX64Platform = _Platform.MacOS | _Platform.Bits64;

            int.TryParse(args[0], out var current);
            Current = (_Platform)current;

            IsWindows = (Current & windowsX64Platform) == Current;
            IsLinux = (Current & linuxX64Platform) == Current;
            IsMacOs = (Current & macOsX64Platform) == Current;
        }
    }
}
