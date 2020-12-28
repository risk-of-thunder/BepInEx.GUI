using System;
using System.Windows.Forms;
using System.Threading;
using System.IO.Pipes;
using System.Runtime.Serialization.Formatters.Binary;

namespace BepInEx.GUI
{
    public static class Program
    {

        private static Form mainForm;
        /// <summary>
        /// The main entry point for the application.
        /// </summary>
        [STAThread]
        public static void Main(params string[] _)
        {
            Application.SetCompatibleTextRenderingDefault(false);
            mainForm = new Form1();
            Application.Run(mainForm);

            Thread thread = new Thread(ClientThread);
            thread.IsBackground = true;
            thread.Start();
        }


        private static void ClientThread()
        {
            NamedPipeClientStream pipeClient;
            BinaryFormatter formatter = new BinaryFormatter();
            try
            {
                pipeClient = new NamedPipeClientStream(".","RoR2-BepInExGUI", PipeDirection.In);
                pipeClient.Connect(1000);
            }
            catch(Exception)
            {
                return;
            }
            while (pipeClient.IsConnected){
                Event e = (Event) formatter.Deserialize(pipeClient);
                if(e != null )
                {
                    //handle event
                }
            }
        }
    }
}
