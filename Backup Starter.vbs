'launch.vbs
Set WshShell = CreateObject("WScript.Shell")
WshShell.Run "Backup.exe", 0
WshShell = Null