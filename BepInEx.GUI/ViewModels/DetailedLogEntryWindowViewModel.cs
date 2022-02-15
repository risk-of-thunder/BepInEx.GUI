namespace BepInEx.GUI.ViewModels
{
    public class DetailedLogEntryWindowViewModel : ViewModelBase
    {
        public string Text { get; }

        public DetailedLogEntryWindowViewModel(string text)
        {
            Text = text;
        }
    }
}
