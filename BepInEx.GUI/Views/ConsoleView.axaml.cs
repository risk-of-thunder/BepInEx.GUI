using Avalonia;
using Avalonia.Controls;
using System;
using System.Collections;
using System.Collections.ObjectModel;
using static BepInEx.GUI.ViewModels.ConsoleViewModel;

namespace BepInEx.GUI.Views
{
    public partial class ConsoleView : UserControl
    {
        public ConsoleView()
        {
            InitializeComponent();

            TextListConsole.GetObservable(ItemsControl.ItemsProperty).Subscribe(ScrollToEnd);
        }

        private void ScrollToEnd(IEnumerable obj)
        {
            if (TextListConsole.Items != null)
            {
                if (TextListConsole.Items is ObservableCollection<ColoredEntry> items)
                {
                    TextListConsole.SelectedIndex = items.Count - 1;
                }
            }
        }
    }
}
