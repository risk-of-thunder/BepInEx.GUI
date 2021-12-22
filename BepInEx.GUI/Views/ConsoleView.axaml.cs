using Avalonia;
using Avalonia.Controls;
using Avalonia.Markup.Xaml;

namespace BepInEx.GUI.Views
{
    public partial class ConsoleView : UserControl
    {
        public ConsoleView()
        {
            InitializeComponent();
        }

        private void InitializeComponent()
        {
            AvaloniaXamlLoader.Load(this);
        }
    }
}
