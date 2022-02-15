using Avalonia;
using Avalonia.Controls;
using Avalonia.Controls.ApplicationLifetimes;
using Avalonia.Interactivity;
using BepInEx.GUI.ViewModels;
using System;
using System.Collections;
using System.Threading.Tasks;

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

        private Vector _oldScrollOffset;
        private void AutoScroll(object? sender, ScrollChangedEventArgs e)
        {
            FixOddBugThatRandomlyScrollConsoleToTheTop();

            if (ConsoleAutoScroll && e.ExtentDelta.Y != 0 && _logEntryCountJustChanged)
            {
                ConsoleScrollViewer.ScrollToEnd();

                _logEntryCountJustChanged = false;
            }

            _oldScrollOffset = ConsoleScrollViewer.Offset;
        }

        private void FixOddBugThatRandomlyScrollConsoleToTheTop()
        {
            if (ConsoleScrollViewer.Offset.NearlyEquals(new Vector(0, 0)))
            {
                ConsoleScrollViewer.Offset = _oldScrollOffset;
            }
        }

        public async void OnLogEntryDoubleClick(object sender, RoutedEventArgs e)
        {
            if (sender is TextBox textBox)
            {
                await ShowDetailedLogEntry(textBox);
            }
        }

        private static async Task ShowDetailedLogEntry(TextBox textBox)
        {
            var detailedLogEntryWindow = new DetailedLogEntryWindow();
            detailedLogEntryWindow.DataContext = new DetailedLogEntryWindowViewModel(textBox.Text);
            if (Application.Current.ApplicationLifetime is IClassicDesktopStyleApplicationLifetime desktop)
            {
                await detailedLogEntryWindow.ShowDialog(desktop.MainWindow);
            }
        }
    }
}
