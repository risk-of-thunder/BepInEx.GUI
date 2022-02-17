using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace BepInEx.GUI.Models.Thunderstore
{
    public class Communities
    {
        [JsonPropertyName("pagination")]
        public Pagination? Pagination { get; set; }

        [JsonPropertyName("results")]
        public List<Result>? Results { get; set; }
    }

    public class Pagination
    {
        [JsonPropertyName("next_link")]
        public object? NextLink { get; set; }

        [JsonPropertyName("previous_link")]
        public object? PreviousLink { get; set; }
    }

    public class Result
    {
        [JsonPropertyName("identifier")]
        public string? Identifier { get; set; }

        [JsonPropertyName("name")]
        public string? Name { get; set; }

        [JsonPropertyName("discord_url")]
        public Uri? DiscordUrl { get; set; }

        [JsonPropertyName("wiki_url")]
        public Uri? WikiUrl { get; set; }

        [JsonPropertyName("require_package_listing_approval")]
        public bool RequirePackageListingApproval { get; set; }
    }
}
