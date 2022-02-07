using Avalonia;
using Avalonia.Controls;
using System;
using System.Collections;

namespace BepInEx.GUI.Views
{
    public partial class ConsoleView : UserControl
    {
        public static bool ConsoleAutoScroll = true;

        public ConsoleView()
        {
            InitializeComponent();

            ConsoleItemsRepeater.GetObservable(ItemsControl.ItemsProperty).Subscribe(ScrollToEnd);
        }

        private void ScrollToEnd(IEnumerable obj)
        {
            if (ConsoleAutoScroll)
            {
                ConsoleScrollViewer.ScrollToEnd();
            }
        }
    }
}
