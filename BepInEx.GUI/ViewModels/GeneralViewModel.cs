using BepInEx.GUI.Models;
using BepInEx.GUI.Models.Thunderstore;
using ReactiveUI;
using System;
using System.Collections.ObjectModel;
using System.Diagnostics;
using System.IO;
using System.Net;
using System.Net.Http;
using System.Text.Json;
using WebSocketSharp;

namespace BepInEx.GUI.ViewModels
{
    public class GeneralViewModel : ViewModelBase
    {
        public PathsInfo PathsInfo { get; }

        public HttpClient HttpClient { get; }

        public string TargetIsLoadingCanCloseWindow { get; }

        private string _loadedModCountText;
        public string LoadedModCountText
        {
            get { return _loadedModCountText; }
            set
            {
                this.RaiseAndSetIfChanged(ref _loadedModCountText, value);
            }
        }

        public ObservableCollection<Mod> Mods { get; }

        public PlatformInfo PlatformInfo { get; }

        public GeneralViewModel(PathsInfo pathsInfo, PlatformInfo platformInfo, WebSocket webSocket)
        {
            PathsInfo = pathsInfo;

            TargetIsLoadingCanCloseWindow = $"{pathsInfo.ProcessName} is loading, you can safely close this window.";

            Mods = new ObservableCollection<Mod>();
            webSocket.OnMessage += AddLoadedModToList;

            PlatformInfo = platformInfo;

            var handler = new HttpClientHandler()
            {
                AutomaticDecompression = DecompressionMethods.GZip | DecompressionMethods.Deflate
            };
            HttpClient = new HttpClient(handler);
        }

        private void AddLoadedModToList(object? sender, MessageEventArgs e)
        {
            const string LoadingModLog = "Loading [";

            var logEntry = LogEntry.Deserialize(e.RawData);
            if (logEntry == null)
            {
                return;
            }

            var logEntryText = logEntry.Data;
            if (logEntryText.Contains(LoadingModLog) && logEntry.Source == "BepInEx")
            {
                var modInfoArray = logEntryText.Split('[')[1].Split(' ');
                var modName = modInfoArray[0];
                var modVersion = modInfoArray[1].Remove(modInfoArray[1].Length - 1, 1);

                Mods.Add(new Mod(modName, modVersion));

                LoadedModCountText = $"Loaded Mods: {Mods.Count}";
            }
        }

        public void OnClickOpenGameFolder()
        {
            OpenFolder(PathsInfo.GameFolderPath);
        }

        public void OnClickShowLogFolder()
        {
            OpenFolder(PathsInfo.BepInExFolderPath);
        }

        public void OnClickShowBepInExFolder()
        {
            OpenFolder(PathsInfo.BepInExFolderPath);
        }

        private void OpenFolder(string folderPath)
        {
            if (!folderPath.EndsWith(Path.DirectorySeparatorChar))
            {
                folderPath += Path.DirectorySeparatorChar;
            }

            if (Directory.Exists(folderPath))
            {
                var processStartInfo = new ProcessStartInfo();
                processStartInfo.FileName = folderPath;
                processStartInfo.UseShellExecute = true;
                processStartInfo.Verb = "open";

                Process.Start(processStartInfo);
            }
            else
            {
                Debug.Message($"{folderPath} Directory does not exist!");
            }
        }

        public async void OnClickModdingDiscordLink()
        {
            try
            {
                var targetProcessName = PathsInfo.ProcessName.ToLowerInvariant();

                var communities = JsonSerializer.Deserialize<Communities>(await HttpClient.GetStringAsync("https://thunderstore.io/api/experimental/community/"))!;
                var foundDiscord = false;
                foreach (var community in communities.Results!)
                {
                    var communityName = community.Name;
                    if (string.IsNullOrWhiteSpace(communityName))
                        continue;
                    communityName = communityName.ToLowerInvariant();

                    if (community.DiscordUrl == null)
                        continue;

                    var discordUrl = community.DiscordUrl.ToString();
                    if (string.IsNullOrWhiteSpace(discordUrl))
                        continue;

                    if (communityName.Contains(targetProcessName) || targetProcessName.Contains(communityName))
                    {
                        var processInfo = new ProcessStartInfo
                        {
                            FileName = discordUrl,
                            UseShellExecute = true
                        };

                        Process.Start(processInfo);

                        foundDiscord = true;
                    }
                }

                if (!foundDiscord)
                {
                    Debug.Message("Did not find any discord for the following target process : " + targetProcessName);
                }
            }
            catch (Exception ex)
            {
                Debug.Message(ex.ToString());
            }
        }
    }
}
