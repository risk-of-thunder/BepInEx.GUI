using System;
using System.Collections.Generic;
using System.Linq;
using System.Windows.Forms;

namespace BepInEx.GUI
{
    public static class Program
    {
        /// <summary>
        /// The main entry point for the application.
        /// </summary>
        [STAThread]
        public static void Main(params string[] args)
        {
            Application.EnableVisualStyles();
            Application.SetCompatibleTextRenderingDefault(false);
            Form1 form = new Form1();
            form.Text = string.Join(" ", args);
            Application.Run(form);
        }
    }
}
