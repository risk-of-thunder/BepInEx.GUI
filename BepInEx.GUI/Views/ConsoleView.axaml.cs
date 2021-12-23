using Avalonia;
using Avalonia.Controls;
using System;

namespace BepInEx.GUI.Views
{
    public partial class ConsoleView : UserControl
    {
        public ConsoleView()
        {
            InitializeComponent();

            // https://github.com/AvaloniaUI/Avalonia/issues/418
            TextBoxConsole.GetObservable(TextBox.TextProperty).Subscribe(ScrollToEnd);
        }

        private void ScrollToEnd(string newText)
        {
            // https://stackoverflow.com/a/58233265
            TextBoxConsole.CaretIndex = int.MaxValue;
        }
    }
}
