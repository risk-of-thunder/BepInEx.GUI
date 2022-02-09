using BepInEx.GUI.Models;
using BepInEx.GUI.Models.Thunderstore;
using System;
using System.Diagnostics;
using System.Net;
using System.Net.Http;
using System.Text.Json;

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

            "If you are a mod developer and wish to enable it back, go to Settings, and tick Enable Developer Tools.";

        public PathsInfo PathsInfo { get; }

        public HttpClient HttpClient { get; }

        public DisclaimerWindowViewModel(PathsInfo pathsInfo)
        {
            PathsInfo = pathsInfo;

            var handler = new HttpClientHandler()
            {
                AutomaticDecompression = DecompressionMethods.GZip | DecompressionMethods.Deflate
            };
            HttpClient = new HttpClient(handler);
        }

        public async void OnClickModdingDiscordLink()
        {
            try
            {
                var communities = JsonSerializer.Deserialize<Communities>(await HttpClient.GetStringAsync("https://thunderstore.io/api/experimental/community/"))!;

                foreach (var res in communities.Results!)
                {
                    var processName = PathsInfo.ProcessName.ToLowerInvariant();
                    var communityName = res.Name!.ToLowerInvariant();
                    if (communityName.Contains(processName) || processName.Contains(communityName))
                    {
                        Debug.Message(res.DiscordUrl!.ToString());

                        var processInfo = new ProcessStartInfo
                        {
                            FileName = res.DiscordUrl!.ToString(),
                            UseShellExecute = true
                        };

                        Process.Start(processInfo);
                    }
                }
            }
            catch (Exception ex)
            {
                Debug.Message(ex.ToString());
            }
        }
    }
}
