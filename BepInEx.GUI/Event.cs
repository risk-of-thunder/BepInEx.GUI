using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace BepInEx.GUI
{
    public class Event
    {
        public enum Category
        {
            Patcher,
            Plugin,
            Game,
        }

        public enum Type
        {
            Begin,
            StartOne,
            FinishOne,
            End,
        }
        
        public Event(Category category, Type type, string args)
        {
            _category = category;
            _type = type;
            _args = args;
        }

        public readonly Category _category;
        public readonly Type _type;
        public readonly string _args;

        public override string ToString()
        {
            var baseString =  _category + ":" + _type;
            if(_args == null)
            {
                return baseString;
            }
            return baseString + ":" + _args.Length + ":" + _args;
            
        }

        public static Event StartEvent(Category category, int count)
        {
            return new Event(category, Type.Begin, count.ToString());
        }

        public static Event EndEvent(Category category)
        {
            return new Event(category, Type.End, null);
        }

        public static Event StartOneEvent(Category category, string name)
        {
            return new Event(category, Type.StartOne, name);
        }

        public static Event FinishOneEvent(Category category, string name)
        {
            return new Event(category, Type.FinishOne, name);
        }
    }
}
