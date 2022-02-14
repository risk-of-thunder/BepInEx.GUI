using Avalonia;
using Avalonia.Controls;
using System;
using System.Collections;

namespace BepInEx.GUI.Views
{
    public partial class ConsoleView : UserControl
    {
        public static bool ConsoleAutoScroll = true;

        private static bool _logEntryCountJustChanged;

        private static ConsoleView? _instance;

        public ConsoleView()
        {
            InitializeComponent();

            ConsoleItemsRepeater.GetObservable(ItemsControl.ItemsProperty).Subscribe(LogEntryCountChanged);

            ConsoleScrollViewer.ScrollChanged += AutoScroll;

            _instance = this;
        }

        public static void ScrollToEnd()
        {
            _instance?.ConsoleScrollViewer?.ScrollToEnd();
        }

        private void LogEntryCountChanged(IEnumerable obj)
        {
            _logEntryCountJustChanged = true;
        }

        private void AutoScroll(object? sender, ScrollChangedEventArgs e)
        {
            if (ConsoleAutoScroll && e.ExtentDelta.Y != 0 && _logEntryCountJustChanged)
            {
                ConsoleScrollViewer.ScrollToEnd();

                _logEntryCountJustChanged = false;
            }
        }
    }
}
