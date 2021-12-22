using Avalonia;
using Avalonia.Controls;
using System;

namespace BepInEx.GUI.Views
{
    public partial class MainWindow : Window
    {
        public MainWindow()
        {
            InitializeComponent();

            MakeTabItemsAutoSized();

#if DEBUG
            this.AttachDevTools();
#endif
        }

        private void MakeTabItemsAutoSized()
        {
            ClientSizeProperty.Changed.Subscribe(OnResize);
            OnResize(null!);
        }

        private void OnResize(AvaloniaPropertyChangedEventArgs obj)
        {
            const float oneThird = 1f / 3.1f;

            var tabWidth = ClientSize.Width * oneThird;

            GeneralTabItem.Width = tabWidth;
            ConsoleTabItem.Width = tabWidth;
            SettingsTabItem.Width = tabWidth;
        }
    }
}
