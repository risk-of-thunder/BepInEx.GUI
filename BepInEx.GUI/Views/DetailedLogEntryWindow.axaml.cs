using Avalonia;
using Avalonia.Markup.Xaml;
using Avalonia.ReactiveUI;
using BepInEx.GUI.ViewModels;
using ReactiveUI;

namespace BepInEx.GUI.Views
{
    public partial class DetailedLogEntryWindow : ReactiveWindow<DetailedLogEntryWindowViewModel>
    {
        public DetailedLogEntryWindow()
        {
            InitializeComponent();
#if DEBUG
            this.AttachDevTools();
#endif
            this.WhenActivated((d) =>
            {
                HalfSizeOfOwner();
                PositionAtCenterOwner();
            });
        }

        private void HalfSizeOfOwner()
        {
            var parentSize = Owner.DesiredSize;
            ClientSize = parentSize / 1.25;
        }

        private void PositionAtCenterOwner()
        {
            var owner = Owner.PlatformImpl;

            var scaling = owner?.DesktopScaling ?? PlatformImpl?.DesktopScaling ?? 1;

            var rect = new PixelRect(
            PixelPoint.Origin,
            PixelSize.FromSize(ClientSize, scaling));

            var ownerRect = new PixelRect(
                    owner!.Position,
                    PixelSize.FromSize(owner.ClientSize, scaling));
            Position = ownerRect.CenterRect(rect).Position;
        }

        private void InitializeComponent()
        {
            AvaloniaXamlLoader.Load(this);
        }
    }
}
