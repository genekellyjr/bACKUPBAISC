# bACKUPBAISC
**basic bitch backup ok**


**GOAL:**

Copy files you want to a backup drive reliably and easily with minimal effort, hassle, and care. Do it way more reliably & easier than File History (Backup program built into Windows - called Backup in the Settings Win10 program). NOTE: does not do file history. Just backs up. It's basic. But it works, which I think is more important.


**USAGE:**

1. Download release files, put all in same folder to run from (maybe C:\BackupProgram why not)


2. Read BACKUP HOW TO USE.txt to know how to prep Backup_destinationLocale.txt and Backup_saveLocales.txt - put the path you want to send data to (destination) and the paths you want to save (saves) in the text files. There's no options or configs.


3. Once those files are set, launch Backup.exe. It will keep you appraised of what is going on with a system tray icon. If there is an error the main window will pop-up and tell you about it.


4. See BACKUP HOW TO USE.txt step 3 for how to set Backup.exe on a regular schedule using Task Scheduler. There is a vbs script included that launches it without a brief pop-up that interupts full screen things (like movies). If you don't want to use the vbs script, you can launch Backup.exe directly - I wasn't able to surpress the initial instantaneous window pop-up without the vbs script. The BACKUP HOW TO USE.txt tells you how to make the vbs script yourself if you like as a bonus.

5. Don't run Backup.exe as Admin, isn't needed and it will mess with networked drives. Basically if Z:\ is a network share that is really \\\servercomp\share\ only the user mode has \\\servercomp\share\ mapped to Z:\, the admin mode doesn't have Z:\ mapped to \\\servercomp\share\. Top troll from Windows. TBH you can get around it by replacing Z:\ with \\servercomp\share\ in Backup-destination.txt if you really need 2


**HOW IT WORKS:**

Source code is provided. It is written in rust now. It takes the stuff you want to save (Backup_saveLocales.txt) and the backup destination (Backup_destinationLocale.txt) and makes the paths needed out of them. It then calls Robocopy (built into Windows, reliable) to do the file moving in a robust way. You then succeed.


**WHY I DO THIS THO:**

Cause Windows has built in Backup program (Settings -> Update & Secuyrity -> Backup) and it MISSED FILES ON THE REG LIKE NBD. If you used that and trusted it then were like "o no i lost file let me recover" and one of them was one of the many it randomly decides to skip? ded.

So I write this. Wrapper for Robocopy.
