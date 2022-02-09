using Avalonia;
using Avalonia.ReactiveUI;
using BepInEx.GUI.Config;
using BepInEx.GUI.ViewModels;
using ReactiveUI;
using System;

namespace BepInEx.GUI.Views
{
    public partial class MainWindow : ReactiveWindow<MainWindowViewModel>
    {
        public MainWindow()
        {
            InitializeComponent();

            MakeTabItemsAutoSized();

#if DEBUG
            this.AttachDevTools();
#endif

            if (MainConfig.ShowOneTimeOnlyDisclaimerConfig.Value)
            {
                this.WhenActivated((d) =>
                {
                    ShowUserDisclaimer();
                });
            }
        }

        private async void ShowUserDisclaimer()
        {
            var disclaimerWindow = new DisclaimerWindow();
            disclaimerWindow.DataContext = new DisclaimerWindowViewModel();
            await disclaimerWindow.ShowDialog(this);

            MainConfig.ShowOneTimeOnlyDisclaimerConfig.Value = false;
            MainConfig.File.Save();
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
