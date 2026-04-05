# heic2jpg
This program was made for a specific task of converting zip files containing
subdirectory storing `.heic` files. Why? Because I need to convert
`.heic` files stored in sorted catalogues on a Google Drive and when downloading
such catalogue I get it as a zip file.

Therefore, if you want to convert your heic files and not dig into the code
just move all your `.heic` files into a catalogue and then zip the catalogue.
After that in the same catalogue as the executable create src and dest directories
and move the zip into the src directory.

If you're good with rust, then rewriting my code won't be tricky for you.

Note: This is my first actual rust project so the code might be a mess for experienced rustaceans.

---
## Building

My problem had to be solved on a Windows 8.1 machine so while probably nobody who might use the program
has Windows 8.1 I will still drop some info on how to compile it.

If you're on Windows 8.1 you need .NET and last working .NET for Windows 8.1 is [.NET 4.8](https://dotnet.microsoft.com/en-us/download/dotnet-framework/net48).
If the wizard refuses to install, the issue might be because of the antivirus. Try to disable it and do it again.

On Windows 10/11 you can get latest .NET and install rust.

The program also requires Libheif installed on the system
and to do so you need to also install `vcpkg` and get
`libheif:x64-windows-static-md`. At least that's what worked for me :D

After all that you should be able to compile the code.

If you see lots of Error messages in the output, then that means
that Libheif tries to load plugins - That is what 
AI told me, and it's (working) solution was to
add new Environment variable `LIBHEIF_PLUGIN_PATH` with some value such as `EMPTY`.
This suppresses the errors.

Good luck and happy converting!
