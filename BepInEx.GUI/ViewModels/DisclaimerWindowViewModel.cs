using System;

namespace BepInEx.GUI.ViewModels
{
    public class DisclaimerWindowViewModel : ViewModelBase
    {
        public string DisclaimerText { get; } =
            "The console is now disabled by default." + Environment.NewLine +
            "For correct troubleshooting, please read below." + Environment.NewLine + Environment.NewLine +

            "By not posting a log file and instead screenshotting random pieces of " +
            "text you find in the console, you only make the process of resolving your mod problem longer." + Environment.NewLine + Environment.NewLine +

            "If you notice issues with a mod while playing, " +
            "head to the Modding Discord by clicking on the button below, " +
            "post the log file in the tech-support channel and wait for help." + Environment.NewLine + Environment.NewLine +

            "If you are a mod developer and wish to enable it back, go to Settings, and tick Enable Developer Tools." + Environment.NewLine +
            "If you like the old conhost console style, you can enable it by opening the BepInEx/config/BepInEx.cfg and " +
            "setting to true the \"Enables showing a console for log output.\" config option.";
    }
}
