using System;
using System.Collections.Generic;
using System.Linq;
using System.IO;

namespace csharp_02
{

    class Password
    {
        Int32 minOccurances { get; set; }
        Int32 maxOccurances { get; set; }
        Char requiredCharacter { get; set; }
        String passwordString { get; set; }

        public Password() { }

        public Password(string password)
        {
            string[] parsedArray = password.Split(new Char[] { '-', ':', ' ' }, StringSplitOptions.RemoveEmptyEntries);

            this.minOccurances = Int32.Parse(parsedArray[0]);
            this.maxOccurances = Int32.Parse(parsedArray[1]);
            this.requiredCharacter = parsedArray[2][0];
            this.passwordString = parsedArray[3];
        }

        public bool isValid()
        {
            // count of required characters in password
            int count = passwordString.Count(c => c == this.requiredCharacter);

            if(count <= this.maxOccurances && count >= this.minOccurances)
            {
                return true;
            }

            return false;
        }

    }


    class Program
    {
        static void Main(string[] args)
        {
            string[] rawPasswords = File.ReadAllLines("./sample-01.txt");
            int counter = 0;
            foreach (var rawPassword in rawPasswords)
            {
                Password password = new Password(rawPassword);
                if(password.isValid())
                {
                    counter++;
                }
            }
            System.Console.WriteLine($"Final count: {counter}");
        }
    }
}
