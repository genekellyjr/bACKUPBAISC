# bACKUPBAISC
basic backup ok


GOAL: 

Copy files you want to a backup drive reliably and easily with minimal effort, hassle, and care. Do it way more reliably & easier than File History (Backup program built into Windows - called Backup in the Settings Win10 program).


USAGE: 

1. Download release files


2. Read BACKUP HOW TO USE.txt to know how to prep Backup_destinationLocale.txt and Backup_saveLocales.txt - put the path you want to send data to (destination) and the paths you want to save (saves) in the text files. There's no options or configs.


3. Once those files are set, launch Backup.exe. It will keep you appraised of what is going on.


4. See BACKUP HOW TO USE.txt step 4 for how to set Backup.exe on a regular schedule using Task Scheduler.


HOW IT WORKS:

Source code is provided. It is written in C. It takes the stuff you want to save (Backup_saveLocales.txt) and the backup destination (Backup_destinationLocale.txt) and makes the paths needed out of them. It then calls Robocopy (built into Windows, reliable) to do the file moving in a robust way. You then succeed.


RANT:


MADE BECAUSE WINDOWS FILE HISTORY (WHICH IS ALSO THE THING IN SETTINGS->UPDATE & SECURITY->BACKUP)

LEAVES

OUT

FILES AND FOLDERS

AND 

WHAT KIND OF BACKUP SYSTEM IS THAT HOLY SHIT

LIKE IT WOULD JUST SKIP A FOLDER

OR NOT DO AN ENTIRE DIRECTORY I ASKED

LOOK I DID IT RIGHT

DON'T TELL ME I DIDN'T I GOOGLED IT AND PEEPS HAVE THE SAME RPOBLEMFF


so we have ez backup powered by ROBOCOPY best copy around built into windows already there wowo


download the files


Read BACKUP HOW TO USE.txt to know how to prep Backup_destinationLocale.txt and Backup_saveLocales.txt

run Backup.exe after you're set

Win!

BACKUP HOW TO USE.txt step 3 tells you how to automate Backup.exe for easy file uploading


Check out Backup-SOURCE_BUILDUROWN.txt to build your own from the source

which is in Backup-SOURCE.txt

This is written in C in notepad.
