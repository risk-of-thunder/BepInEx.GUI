using ProtoBuf;
using System;
using System.IO;

namespace BepInEx.GUI.Models
{
#pragma warning disable CS8618 // Non-nullable field must contain a non-null value when exiting constructor. Consider declaring as nullable.

    // https://github.com/PassivePicasso/BepInEx-LogEx/tree/master/WebSlog
    [ProtoContract]
    public class LogEntry
    {
        [ProtoMember(1)]
        public string Source { get; internal set; }

        [ProtoMember(2)]
        public string Level { get; internal set; }

        [ProtoMember(3)]
        public Logging.LogLevel LevelCode { get; internal set; }

        [ProtoMember(4)]
        public string Data { get; internal set; }

        public static LogEntry? Deserialize(byte[] data)
        {
            if (data == null)
            {
                return null;
            }

            try
            {
                using var stream = new MemoryStream(data);
                return Serializer.Deserialize<LogEntry>(stream);
            }
            catch (Exception e)
            {
                // todo
            }

            return null;
        }

        public override string ToString()
        {
            return $"[{Level}: {Source}] {Data}";
        }
    }

#pragma warning restore CS8618 // Non-nullable field must contain a non-null value when exiting constructor. Consider declaring as nullable.
}
