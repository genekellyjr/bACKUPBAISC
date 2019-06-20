//disable annoying warning about my variable names and my parenthesis use
#![allow(non_snake_case)] //welcome to namingThingsLikeThis
#![allow(unused_parens)] //welcome to if( stuff == ffuts )
//-----Import Libraries (called crates)-----
extern crate winapi;
//-----Import Built-in Libraries (not called crates)-----
use std::process::Command; //use cmd.exe
use std::mem::{size_of, zeroed}; //get size of stuff and init with zeros
use std::ptr::null_mut; //use a null pointer (I think)
use std::char; //use some char
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt; //convert from string to UTF-16 (1 or 2 uint16 apparently)
//use std::io::prelude::*; //provides error handling for writing files - bypassed now with renaming files

//-----Define Constants-----
const MAX_COMPUTERNAME_LENGTH: u32 = 32; //apparently it's really 15 but why not be sure because on MAC systems it could be 31 + null
const MAX_PATH_WIN: u32 = 1024; // Maximum Windows length allowed
//const MAX_PATH_UNC: u32 = 32767; //max UNC length allowed, unused atm
const UNC_PREFIX: &str = "\\\\?\\"; //UNC prefix needed for local paths in Windows
const MAX_SZTIP_LEN: u32 = 128; //Max allowed tip length in Windows
//UNC not fully supported b/c it goes to ~32,767 and this is 1024 so
//Max path is apparently more like 255 or 260 depending on where you are at the time, unless its specially enabled in Win10 but we don't talk about that go away

//-----Start error handler function-----
fn main() //an error handler function for Rust because it needs help! NOTE THAT ANYTHING NAMED main() CAN'T RETURN ANYTHING!
{
    let exitCode = realMain();
    std::process::exit(exitCode);
}

//GOAL: RUN ROBOCOPY TO COPY FILES FROM LIST OF SOURCES TO DESTINATION. READ DESTINATION LOCATION AND SOURCE LIST FROM TEXT FILES.


fn realMain() -> i32 //this is the real stuff, uses fn main() above as an error handler because Rust is special
{
    //Suppress command window so it doesn't steal screen real-estate and bump out of movies etc.
    // to navigate calling with the winapi "crate" use the search function at link
    // https://docs.rs/winapi/*/x86_64-pc-windows-msvc/winapi/um/wincon/fn.GetConsoleWindow.html
    let hWnd = winapi::um::wincon::GetConsoleWindow; //gets the current console window handle inspired by https://stackoverflow.com/questions/11812095/hide-the-console-window-of-a-c-program via Anthropos
    // https://msdn.microsoft.com/en-us/library/windows/desktop/ms633548%28v=vs.85%29.aspx
    //unsafe { winapi::um::winuser::ShowWindow(hWnd(), winapi::um::winuser::SW_MINIMIZE) }; //won't hide the window without SW_MINIMIZE https://msdn.microsoft.com/en-us/library/windows/desktop/ms633548(v=vs.85).aspx check this
    //unsafe { winapi::um::winuser::ShowWindow(hWnd(), winapi::um::winuser::SW_HIDE) }; //Hide window
    //unsafe{ winapi::um::winuser::ShowWindow(hWnd(), winapi::um::winuser::SW_SHOW) }; //Show window


    //System Tray Icon support - here it is
    let WM_MYMESSAGE = winapi::um::winuser::WM_APP + 100; //prep WM_MYMESSAGE
    let mut trayToolTip = "Basic Bitch Backup is running in the background".to_string(); //record tooltip words for the icon

    let mut trayToolTipInt: [u16; MAX_SZTIP_LEN as usize] = [0; MAX_SZTIP_LEN as usize]; //fill with 0's
    let trayToolTipStrStep: &str = &*trayToolTip; //these two types of strings are hella annoying
    let mut trayToolTipStepOS = OsStr::new(trayToolTipStrStep); //convert to OS string format or something
    let mut trayToolTipStepUTF16 = trayToolTipStepOS.encode_wide().collect::<Vec<u16>>(); //now actually convert to UTF16 format for the OS
    if( trayToolTipStepUTF16.len() > ((MAX_SZTIP_LEN-1) as usize) ) //leave room for null at the end (check UTF-16 for length b/c that's what matters to the OS!)
        {
            //If the length is greater than MAX_SZTIP_LEN-1 then the null at the end is def gone and the letters need to be elipsised away...
            trayToolTipStepUTF16.truncate(MAX_SZTIP_LEN as usize); //make sure it isn't bigger than MAX_SZTIP_LEN
            //trayToolTipStepUTF16.splice( ((MAX_SZTIP_LEN-3) as usize)..(MAX_SZTIP_LEN as usize), ); //vecs are dumb
            trayToolTipStepUTF16[ (MAX_SZTIP_LEN as usize) - 3 ] = 46; //force a .
            trayToolTipStepUTF16[ (MAX_SZTIP_LEN as usize) - 2 ] = 46; //force a . 
            trayToolTipStepUTF16[ (MAX_SZTIP_LEN as usize) - 1 ] = 0; //force a null terminator at the end
            //making sure this doesn't do anything weird with UTF16 characters that are 2xUTF16s together is hard and I can't test it, so I won't sorry
        }
    trayToolTipInt[0..trayToolTipStepUTF16.len()].copy_from_slice(&trayToolTipStepUTF16); //record it in that nice integer holder

    //Start a holder for the system tray icon
    let mut nid: winapi::um::shellapi::NOTIFYICONDATAW = unsafe{ zeroed() }; //thing that has info on window and system tray stuff in it 
    unsafe
    {
        //Fill the holder with req'd info
        nid.cbSize = size_of::<winapi::um::shellapi::NOTIFYICONDATAW>() as u32; //prep
        nid.hWnd = hWnd(); //links the console window
        nid.uID = 1001; //it's a number
        nid.uCallbackMessage = WM_MYMESSAGE; //whoknows should be related to click capture but doesn't so
        nid.hIcon = winapi::um::winuser::LoadIconW(null_mut(), winapi::um::winuser::IDI_APPLICATION); //icon idk
        nid.szTip = trayToolTipInt; //tooltip for the icon
        //nid.szTip[47] = '\0'; //null at the end of it
        nid.uFlags = winapi::um::shellapi::NIF_MESSAGE | winapi::um::shellapi::NIF_ICON | winapi::um::shellapi::NIF_TIP; //who knows
    };

    //let mut nidszTipLength = trayToolTipStepUTF16.len() as u64; //gets the size of nid.szTip (tooltip length) for the UTF-16 format, which is what Windows cares about

    unsafe{ winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_ADD, &mut nid) }; //shows the icon

    //Introduce program
    println!("  ____           _____ _____ _____   ____ _____ _______ _____ _    _   ____          _____ _  ___    _ _____  ");
    println!(" |  _ \\   /\\    / ____|_   _/ ____| |  _ \\_   _|__   __/ ____| |  | | |  _ \\   /\\   / ____| |/ / |  | |  __ \\ ");
    println!(" | |_) | /  \\  | (___   | || |      | |_) || |    | | | |    | |__| | | |_) | /  \\ | |    | ' /| |  | | |__) |");
    println!(" |  _ < / /\\ \\  \\___ \\  | || |      |  _ < | |    | | | |    |  __  | |  _ < / /\\ \\| |    |  < | |  | |  ___/ ");
    println!(" | |_) / ____ \\ ____) |_| || |____  | |_) || |_   | | | |____| |  | | | |_) / ____ \\ |____| . \\| |__| | |     ");
    println!(" |____/_/    \\_\\_____/|_____\\_____| |____/_____|  |_|  \\_____|_|  |_| |____/_/    \\_\\_____|_|\\_\\\\____/|_|     ");
    println!("\t\t\tNota Bene: Omne plebes sunt elementa canes in meo oculo.\n");
    //Any gender can be a basic bitch so don't get too hung up on it this is gender-inclusive. #woke?


    //***************************DECLARE CONSTANT-SIZED VARIABLES*************************************************************
    //COUNTERS AND FLAGS THAT ARE REUSED A LOT #NOTMYVARIABLE
    let mut k: u64; //legit only counter or flag
    let mut m: u64; //legit only counter or flag

    //COMMAND-RELATED STUFF
    //let commandRobo = "robocopy"; //make sure you count those numbers
    //let commandOptsComb = "/MIR /copy:DAT /MT:32 /Z /R:2 /W:03 /v /LOG:"; //make sure you count those numbers //removed /eta
    let commandOpt1 = "/MIR"; //mirror directories
    let commandOpt2 = "/copy:DAT"; //copy attributes
    let commandOpt3 = "/MT:32"; //use 32 I/O threads (low CPU still, but better bandwidth utilization)
    let commandOpt4 = "/Z"; //idr
    let commandOpt5 = "/R:2"; //Retry twice
    let commandOpt6 = "/W:03"; //Wait 3 sec between tries
    let commandOpt7 = "/v"; //verbose logging

    let roboError2 = "ERROR 2 (0x00000002) Accessing Source Directory"; //within robocopy log file - error means source directory does not exist
    let roboError3 = "ERROR 3 (0x00000003) Creating Destination Directory"; //within robocopy log file - error means destination directory does not exist
    let roboError123 = "ERROR 123 (0x0000007B) Opening Log File"; //within robocopy log file or cmd output - error means log file directory was incorrect which is !V BAD! cause it should, it's just the .exe location!
    let roboWarningLogSaveAddition = "FILECOPYFAILURE.log"; // save this for adding
    let pathExeLogPostfix = "-backup.log".to_string(); //prep constant insert


    //***************************DECLARE FUNCTION IS GOING DOWN*************************************************************
    println!("*********************BACKUP PROGRAM HAS BEEN ENGAGED*********************");


    //***************************FIND RUNNING PATH*************************************************************
    //Inspired by link below, but I didn't keep the memory allocation stuff.
    //hmjd @ https://stackoverflow.com/questions/9112893/how-to-get-path-to-executable-in-c-running-on-windows

    let pathUTF: [u16; MAX_PATH_WIN as usize] = [0; MAX_PATH_WIN as usize]; //fill with 0's, will hold UTF-16 encoding windows uses
    //let mut pathUTFUNC: [u16; MAX_PATH_UNC as usize] = [0; MAX_PATH_UNC as usize]; //fill with 0's, will hold UTF-16 encoding windows uses
    let pathPtr = &pathUTF as *const u16 as *mut u16; //make a pointer to it (a mutable pointer - starts it as a constant pointer then goes to a mutable one because Rust is ~special~ apparently)
    let pathLength = unsafe{ winapi::um::libloaderapi::GetModuleFileNameW(null_mut(), pathPtr, MAX_PATH_WIN) }; //gets the path in pathUTF using pathPtr and the result is I think the length of the path
    let lastError = unsafe{ winapi::um::errhandlingapi::GetLastError() }; //gets the last error, which prob happened right after above ran, and will check it later

    let mut pathFLG: u8 = 1; //flag for if running path is OK or not
    if(0 == pathLength) //if pathLength is empty, something weird happened
    {
        pathFLG = 0; //then set the running path flag to 0
    }
    if( (pathLength == (MAX_PATH_WIN - 1)) | ( winapi::shared::winerror::ERROR_INSUFFICIENT_BUFFER == lastError ) ) //if the result is the size of the max path constant, prob an issue
    { //also checks for if insufficient buffer error occurred - will error out then
        
        //not right now, rust is silly
        //If insufficient buffer existed, switch to UNC sizing and try again
        //pathUTF: [u16; MAX_PATH_UNC as usize] = [0; MAX_PATH_UNC as usize]; //fill with 0's, will hold UTF-16 encoding windows uses
        //pathPtr = &pathUTF as *const u16 as *mut u16; //make a pointer to it (a mutable pointer - starts it as a constant pointer then goes to a mutable one because Rust is ~special~ apparently)
        //pathLength = unsafe{ winapi::um::libloaderapi::GetModuleFileNameW(null_mut(), pathPtr, MAX_PATH_UNC) }; //gets the path in pathUTF using pathPtr and the result is I think the length of the path
        //lastError = unsafe{ winapi::um::errhandlingapi::GetLastError() }; //gets the last error, which prob happened right after above ran, and will check it later

        //check again to be sure
        //if( (pathLength == (MAX_PATH_UNC - 1 - 4)) | ( winapi::shared::winerror::ERROR_INSUFFICIENT_BUFFER == lastError ) ) //if the result is the size of the max path constant, prob an issue
        //{
        let mut path = String::from_utf16(&pathUTF).unwrap(); //convert from UTF-16 to String format
        path = path.trim_right_matches(char::from(0)).to_string(); //remove nulls from UTF-16 being started with 0's which are nulls
        unsafe{ winapi::um::winuser::ShowWindow(hWnd(), winapi::um::winuser::SW_SHOWNA) }; //bring the window back into the world because of an issue/error
        println!("\n************************************************************************************");
        println!("ERROR: Path length of {} is greater than allowed Windows path length {}.\nAssociated error is: {}\n
        Path read:{}\n",pathLength,lastError,MAX_PATH_WIN,path);
        let _ = Command::new("cmd.exe").arg("/c").arg("pause").status(); // wait for user input
        unsafe{ winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_DELETE, &mut nid) }; //deletes system tray icon when done
        return 4; //return exit code (using extra function allows this to work)   
        //}

        //Because in Rust declaring variables in an if statement means they DON'T EXIST OUTSIDE OF IT
        //pathFLG = 2; //use the flag variable to show it worked and didn't error out, and redo it outside (?)
    }

    if( pathFLG == 0 ) //if path is 0 this trips
    {
        unsafe{ winapi::um::winuser::ShowWindow(hWnd(), winapi::um::winuser::SW_SHOWNA) }; //bring the window back into the world because of an issue/error
        println!("\n************************************************************************************");
        println!("ERORR: Path exe finding error failure: {}\nI'm not sure why it failed or what to do about that so sorry lad&orlass\nEXITINGGG SRRY SO SRRY\n", lastError);
        let _ = Command::new("cmd.exe").arg("/c").arg("pause").status(); // wait for user input
        unsafe{ winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_DELETE, &mut nid) }; //deletes system tray icon when done
        return 5; //return exit code (using extra function allows this to work)
    }

    let pathLocal = String::from_utf16(&pathUTF).unwrap(); //convert from UTF-16 to String format
    let mut pathLocalStr: &str = &*pathLocal; //these two types of strings are hella annoying
    let mut path = format!("{}{}", UNC_PREFIX.to_string(), pathLocal ); //add UNC support (\\?\ before the C:\ stuff) for very long path names
    pathLocalStr = pathLocalStr.trim_right_matches(char::from(0)); //remove nulls from UTF-16 being started with 0's which are nulls
    //let pathLocal = pathLocalStr.to_string(); //convert back to String
    path = path.trim_right_matches(char::from(0)).to_string(); //remove nulls from UTF-16 being started with 0's which are nulls
    let pathLength = path.chars().count(); //get the path length after \\?\ is added
    //println!("TEST-STR LEN = {}\nPRINTING PATH:{}|end",pathLength,path);

    let pathStr: &str = &*path; //these two types of strings are hella annoying
    //find last instance of \
    //k = path.chars().position(|c| c == '\\').unwrap() as u64; //goes from front, not helpful here
    k = path.rfind('\\').unwrap() as u64; //find where last \ is    
    
    let pathBase = &pathStr[0..pathStr.char_indices().nth( (k as usize) +1).unwrap().0].to_string(); //get a bit of that string
    //println!("TEST-PATH BASE:{}\n",pathBase);

    //CREATE DIRECT DESTINATION FILE LOCALE AND SAVE FILE LOCALE FOR GUARANTEED READING
    //printf("TEST-STRING MAKING\n");
    let destinationFilePath; //gotta declare it outside of an if statement in Rust (oh dear)
    if( (pathLength+29) > MAX_PATH_WIN as usize ) //make sure we don't go over the limit (29 is length of Backup_destinationLocale.txt and a '\0' null char - I think in Rust it's just gonna be a UTF-16 0)
    {
        println!("Error - Max char limit for file location where destination locale is held, rest was cut off - only using relative path {}\nIt's gonna error out later sorry so sorry lass&orlad\n","Backup_destinationLocale.txt");
        destinationFilePath = "Backup_destinationLocale.txt".to_string(); //create destination file path (LOCAL ONLY!)
    }
    else
    {
        //let destinationFilePath = [pathBase, "Backup_destinationLocale.txt".to_string()].concat(); //create destination file path (no work)
        destinationFilePath = format!("{}{}", pathBase, "Backup_destinationLocale.txt".to_string() ); //create destination file path (weird but work)
    }
    let destinationFilePathStr: &str = &*destinationFilePath; //these two types of strings are hella annoying
    //let destinationFilePathUTF = OsStr::new(destinationFilePathStr); //convert to UTF-16 afaik

    let saveFilePath; //gotta declare it outside of an if statement in Rust (oh dear)
    if( (pathLength+22) > MAX_PATH_WIN as usize ) //make sure we don't go over the limit (22 is length of Backup_saveLocales.txt and a '\0' null char, Rust is 0 in UTF-16 currently)
    {
        println!("Error - Max char limit for file location where save locales are held, rest was cut off - only using relative path {}\nIt's gonna error out later sorry so sorry lass&orlad\n","Backup_saveLocales.txt");
        saveFilePath = "Backup_saveLocales.txt".to_string(); //create save file path (LOCAL ONLY!)
    }
    else
    {
        saveFilePath = format!("{}{}", pathBase, "Backup_saveLocales.txt".to_string() ); //create save file path
    }
    let saveFilePathStr: &str = &*saveFilePath; //these two types of strings are hella annoying
    //let saveFilePathUTF = OsStr::new(saveFilePathStr); //convert to UTF-16 afaik

    //println!("TEST-Current destination path: {}\nCurrent save path: {}\n",destinationFilePath,saveFilePath);


    // ***************************OPEN AND READ DESTINATION LOCATION*************************************************************
    let fdestExist = std::path::Path::new(destinationFilePathStr).exists(); //check if file exists
    //println!("TEST-File exist return is: {}",fdestExist);

    if( !fdestExist ) //if file exist is false this is then true
    {
        unsafe{ winapi::um::winuser::ShowWindow(hWnd(), winapi::um::winuser::SW_SHOWNA) }; //bring the window back into the world because of an issue/error
        println!("\n************************************************************************************");
        println!("ERROR: No destination location file found, giving up.\n");
        let _ = Command::new("cmd.exe").arg("/c").arg("pause").status(); // wait for user input
        unsafe{ winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_DELETE, &mut nid) }; //deletes system tray icon when done
        return 5; //return exit code (using extra function allows this to work)
    }

    let mut destinationRead = std::fs::read_to_string(destinationFilePath).expect("ERROR: Unable to read file. Giving up soon.");
    let mut destination = "makeRustHappy".to_string(); //prep, needs to be something because if the if
    //let destinationReadLines = destinationRead.lines(); //split by lines
    destinationRead = destinationRead.trim_right().to_string(); //remove spaces, nulls, \n's, \r\n's and other stuff
    //let destination = format!("{}{}", UNC_PREFIX, destinationRead ); //add UNC support for very long path names
    //let destinationStr: &str = &*destination; //these two types of strings are hella annoying
    let destinationLines = destinationRead.lines(); //get the lines
    k = 0; //prep counter
    for line in destinationLines //cound't figure out how to get the line number any other way
    {
        if( k == 0 ) //Only want first line
        {
            destination = line.trim_right().to_string(); //remove spaces, nulls, \n's, \r\n's and other stuff
        }
        k = k + 1; //increment
    }

    if( destination.chars().last().unwrap() != "\\".to_string().chars().last().unwrap() ) //make sure last is a \
    //also am I a hacker? apparently. what code am I right damn
    {   
        destination = format!("{}\\",destination); //stick on a \ to make Z: become Z:\ or whatever
    }

    let destinationLength = destination.chars().count(); //get the number of characters

    if( destinationLength > ((MAX_PATH_WIN-1) as usize) )
    {
        unsafe{ winapi::um::winuser::ShowWindow(hWnd(), winapi::um::winuser::SW_SHOWNA) }; //bring the window back into the world because of an issue/error
        println!("\n************************************************************************************");
        println!("ERROR: Destination length of {} is greater than allowed Windows path length {}.\n
        Consider setting HKEY_LOCAL_MACHINE\\SYSTEM\\CurrentControlSet\\Control\\FileSystem LongPathsEnabled (Type: REG_DWORD) to 1 to increase path, must be in Windows 10 v1607+ (version found in Settings -> System -> Info)\n
        Destination path read:{}\n",destinationLength,MAX_PATH_WIN,destination);
        let _ = Command::new("cmd.exe").arg("/c").arg("pause").status(); // wait for user input
        unsafe{ winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_DELETE, &mut nid) }; //deletes system tray icon when done
        return 4; //return exit code (using extra function allows this to work)
    }

    if( destinationLength == 0 )
    {
        unsafe{ winapi::um::winuser::ShowWindow(hWnd(), winapi::um::winuser::SW_SHOWNA) }; //bring the window back into the world because of an issue/error
        println!("\n************************************************************************************");
        println!("ERROR: No destination location in file found, giving up.\n");
        let _ = Command::new("cmd.exe").arg("/c").arg("pause").status(); // wait for user input
        unsafe{ winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_DELETE, &mut nid) }; //deletes system tray icon when done
        return 2; //return exit code (using extra function allows this to work)
    }

    println!("Backup Destination:\n{}\n",destination);

    // ***************************DETERMINE IF NETWORK DRIVE*************************************************************
    //DISABLED UNTIL OBVIOUS IT IS NEEDED - IT DOES MAKE A UNC PATH TO THE REQUESTED NETWORK DRIVE THO LET ME TELL YOU ITS REAL

    //READING FROM A COMMAND SOURCED FROM https://stackoverflow.com/questions/646241/c-run-a-system-command-and-get-output
    //THX2BILL it works on Windows too
    //let mut FLG_destinationNetwork: u64; //prep flag if a networked destination is detected
    //let mut FLG_destinationNetworkUnavail: u64;; //prep flag if a networked destination is currently unavailable
    let mut destinationUNCStr: &str; //prep n clear
    let mut destinationUNC: String; //prep n clear
    let destinationTwoCharStr = &destination[0..destination.char_indices().nth(2).unwrap().0]; //gets the first two characters (Rust has some awful . adding crap)
    let destinationTwoChar = destinationTwoCharStr.to_string(); //write to string
    let destinationTwoCharAfterStr = &destination[destination.char_indices().nth(2).unwrap().0..destination.len()]; //gets the first two characters (Rust has some awful . adding crap)
    let destinationTwoCharAfter = destinationTwoCharAfterStr.to_string(); //write to string
    //println!("TEST-dest first 2 char:{}",destinationTwoChar);

    // Open the command stream "net use" which gives networked drives full paths (e.g. \\servername\)
    //let fcmd = Command::new("cmd.exe").arg("/c").arg("net use").output().unwrap(); //run cmd.exe net use
    let fcmd = Command::new("net").arg("use").output().unwrap(); //run net use
    //println!("TEST-status: {}", fcmd.status.success()); //reports success (true or false) of command
    //println!("TEST-stdout: {}", String::from_utf8_lossy(&fcmd.stdout)); //reports output of command
    //println!("TEST-stderr: {}", String::from_utf8_lossy(&fcmd.stderr)); //reports error words of command

    if( !fcmd.status.success() ) //make sure something happened
    {
        unsafe{ winapi::um::winuser::ShowWindow(hWnd(), winapi::um::winuser::SW_SHOWNA) }; //bring the window back into the world because of an issue/error
        println!("\n************************************************************************************");
        println!("ERROR: Failed to run cmd.exe net use to figure out if drive given is a network drive, giving up.\n");
        let _ = Command::new("cmd.exe").arg("/c").arg("pause").status(); // wait for user input
        unsafe{ winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_DELETE, &mut nid) }; //deletes system tray icon when done
        return 1; //1 for sass
    }

    let fcmdOutput = String::from_utf8_lossy(&fcmd.stdout); //get the output
    //println!("TEST-output: {}",fcmdOutput);

    let fcmdSearch: bool; //prep outside because things can't start in if statements in Ruist
    if( (destinationTwoCharStr.contains("\\\\")) & (!destination.contains(":") ) ) //if it has \\ AND no :
    {
        fcmdSearch = false; //prep it as false so the network stuff doesn't try to network it up
    }
    else //otherwise it's prob. a normal drive like Z:\
    {
        fcmdSearch = fcmdOutput.contains(destinationTwoCharStr); //check if output contains drive letter
    }
    //println!("TEST-output search: {}",fcmdSearch);

    if( fcmdSearch ) //if search is true, turn on the flag and do some prep
    {
        //FLG_destinationNetwork = 1; //turn on flag that shows drive is a networked destination locale
        //println!("\nTEST-NETWORK DESTINATION DETECTED\n");

        destinationUNCStr = &fcmdOutput[fcmdOutput.find(destinationTwoCharStr).unwrap()..fcmdOutput.len()]; //find the destination letters and then get everything to the end
        let mut fcmdDestUNCLoc = destinationUNCStr.find("\\\\").unwrap(); //finds the first instance of \\ which is like right after the D: drive letter
        //LOCAL VAR ONLY
        destinationUNCStr = &destinationUNCStr[fcmdDestUNCLoc..destinationUNCStr.len()]; //cut off some more
        fcmdDestUNCLoc = destinationUNCStr.find(" ").unwrap(); //finds first space (which signifies end of the UNC string)
        destinationUNCStr = &destinationUNCStr[0..fcmdDestUNCLoc]; //cut off some more
        destinationUNC = destinationUNCStr.to_string(); //convert to string
        destinationUNC = format!("{}{}",destinationUNC,destinationTwoCharAfter); //tack on anything after
        //destinationUNCStr = &*destinationUNC; //convert to &str

        //Check if network is unavailable
        let mut fcmdDestUNCLinePrev = &fcmdOutput[0..fcmdOutput.find(destinationTwoCharStr).unwrap()]; //find the beginning to the destination letters
        //LOCAL VAR ONLY
        fcmdDestUNCLoc = fcmdDestUNCLinePrev.rfind("\n").unwrap(); //finds the last line break (which happens right before the status)
        //setup: \nOK     Z:      \\UNCPATH\       Microsoft Windows Newtork\n,basically
        fcmdDestUNCLinePrev = &fcmdDestUNCLinePrev[fcmdDestUNCLoc..fcmdDestUNCLinePrev.len()]; //basically is just the status for the destination drive (OK, Unavailable, Disconnected are opts I've seen so far!)

        if( !fcmdDestUNCLinePrev.contains("OK") ) //if anything that's not OK, OK won't be found
        {
            //printf("\nTEST: UNAVAILABLE FOUND\n");
            //FLG_destinationNetworkUnavail = 1; //turn on flag that it's unavailable ATM and take steps to make it available

            //try to get it going if it's not going
        
            println!("*********NOTA BENE: Network drive is currently disconnected, attempting to force a reconnection. An error catch will trigger later if reconnection attempt fails.*********");
            //NOTE: Add string length checks
            //this could go bad most def yolo
            let destinationNetworkFixEngage = format!("net use {} {}",destinationTwoChar,destinationUNC); // make a string to tack on anything after
            //let destinationNetworkFixEngageStr = &*destinationNetworkFixEngage; //convert to &str

            //strcat(destinationNetworkFixStart, "pushd "); //add to the string
            //strcat(destinationNetworkFixStart, destinationUNC); //add to the string

            //strcat(destinationNetworkFixEnd, "popd "); //add to the string
            //strcat(destinationNetworkFixEnd, destinationUNC); //add to the string

            Command::new("net").arg("use").arg(&*destinationTwoChar).arg(&*destinationUNC).output().unwrap(); //force some mappin action HARDCORE LOOK OUT
            println!("RAN COMMAND TO FORCE CONNECTION: {}\n\n",destinationNetworkFixEngage ); //report command used
            //system(destinationNetworkFixStart); //hopefully engage the drive
        }

    }
    else
    {
        if( (destinationTwoCharStr.contains("\\\\")) & (!destination.contains(":") ) ) //if so, it's already UNC because it's \\somenetwork\destination\
        {
            destinationUNC =  format!("{}",destination); //already UNC, use format! to prevent borrowing issues and crap
            //destinationUNCStr = &*destinationUNC; //convert to &str
        }
        else //otherwise it's local Z:\ and gets a prefix
        {
            //destinationUNC = format!("{}{}",UNC_PREFIX.to_string(),destination); //make UNC path anyway using local \\?\ prefix
            destinationUNC = format!("{}",destination); //robocopy does NOT like UNC so skipping - and this way prevents borrowing issues that I've never bothered to learn
            //destinationUNCStr = &*destinationUNC; //convert to &str
        }
        
    }
    //println!("TEST-Dest UNC:{}|\nStringV:{}|",destinationUNCStr,destinationUNC);


    // ***************************OPEN AND READ SAVES NUMBER*************************************************************
    let fsavesExist = std::path::Path::new(saveFilePathStr).exists(); //check if file exists

    //println!("TEST-File exist return is: {}",fsavesExist);

    if( !fsavesExist ) //if file exist is false this is then true
    {
        unsafe{ winapi::um::winuser::ShowWindow(hWnd(), winapi::um::winuser::SW_SHOWNA) }; //bring the window back into the world because of an issue/error
        println!("\n************************************************************************************");
        println!("ERROR: No backup source location file found, giving up.\n");
        let _ = Command::new("cmd.exe").arg("/c").arg("pause").status(); // wait for user input
        unsafe{ winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_DELETE, &mut nid) }; //deletes system tray icon when done
        return 5; //return exit code (using extra function allows this to work)
    }

    let mut saves_read = std::fs::read_to_string(saveFilePath).expect("ERROR: Unable to read file. Giving up soon.");
    saves_read = saves_read.trim_right().to_string(); //remove spaces, nulls, \n's, \r\n's and other stuff at the end of the file only
    //let saves_readStr: &str = &*saves_read; //these two types of strings are hella annoying

    
    let saves_lines = saves_read.lines(); //get the lines
    let mut saves_number: u64 = 0; //prep counter to record number of save locations
    for line in saves_lines //cound't figure out how to get the line number any other way
    {
        saves_number = saves_number + 1; //count the lines
        //println!("{}|", line);
        if( !line.contains("\\") ) //make sure it doesn't have a \
        {
            saves_number = saves_number - 1; //remove that line count if there's no \ which is req for a proper path
        }
    }
    //println!("TEST-Line num:{}",k);

    if( saves_number == 0)
    {
        //NOTE: may not activate HOPE IT DOES ALWAYS
        unsafe{ winapi::um::winuser::ShowWindow(hWnd(), winapi::um::winuser::SW_SHOWNA) }; //bring the window back into the world because of an issue/error
        println!("\n************************************************************************************");
        println!("ERROR: No backup source locations in file were found, giving up.\n");
        let _ = Command::new("cmd.exe").arg("/c").arg("pause").status(); // wait for user input
        unsafe{ winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_DELETE, &mut nid) }; //deletes system tray icon when done
        return 2; //2 for sass
    }
    println!("Number of save locations: {}",saves_number); //print save number

    let mut saves_locales: Vec<String> = vec![String::new(); saves_number as usize]; //prep an array of strings based on a variable, so using vec!
    let mut saves_localesUNC: Vec<String> = vec![String::new(); saves_number as usize]; //prep an array of strings based on a variable, so using vec!

    let saves_lines = saves_read.lines(); //get the lines again cause it's a complainy thing and I don't know what it wants
    k = 0; //prep counter
    for line in saves_lines //cound't figure out how to get the line number any other way
    {
        if( line.contains("\\") ) //make sure it HAS a \ now
        {
            saves_locales[k as usize] = line.to_string(); //record the line that's a real line
            saves_localesUNC[k as usize] = format!("{}{}",UNC_PREFIX.to_string(),saves_locales[k as usize]); //record the line that's a real line, prefix with local UNC \\?\
            k = k + 1; //increment as we go
        }
    }

    //REPORT LOCATIONS TO BE BACKED-UP
    for i in 0..saves_number //cound't figure out how to get the line number any other way
    {
        println!("{}",saves_locales[i as usize]);
    }

    // ***************************FIND COMPUTER NAME*************************************************************
    let compNameUTF: [u16; MAX_PATH_WIN as usize] = [0; MAX_PATH_WIN as usize]; //fill with 0's, will hold UTF-16 encoding windows uses
    let compNameUTFPtr = &compNameUTF as *const u16 as *mut u16; //make a pointer to it (a mutable pointer - starts it as a constant pointer then goes to a mutable one because Rust is ~special~ apparently)
    let MAX_COMPNAME_LEN: u32 = MAX_COMPUTERNAME_LENGTH;
    let MAX_COMPUTERNAME_LENGTHPtr = &MAX_COMPNAME_LEN as *const u32 as *mut u32; //make a pointer to it (a mutable pointer - starts it as a constant pointer then goes to a mutable one because Rust is ~special~ apparently)
    unsafe{ winapi::um::winbase::GetComputerNameW( compNameUTFPtr , MAX_COMPUTERNAME_LENGTHPtr ) }; //read computer name
    let mut compName = String::from_utf16(&compNameUTF).unwrap(); //convert from UTF-16 to String format
    compName = compName.trim_right_matches(char::from(0)).to_string(); //remove nulls only, needed as trim didn't normally apparently
    //let compNameStr: &str = &*compName; //these two types of strings are hella annoying


    println!("\nComputer Name: {}\n",compName);


    // ***************************CREATE DESTINATION FOLDERS*************************************************************
    let mut destination_saves: Vec<String> = vec![String::new(); saves_number as usize]; //prep an array of strings based on a variable, so using vec!
    let mut destination_savesTempStr: &str; //prep n clear
    let mut destination_savesTemp: String; //prep n clear

    for i in 0..saves_number
    {
        destination_savesTempStr = &*saves_locales[i as usize];
        //destination_savesTemp = destination_savesTemp.find(":").map(|c| &destination_savesTemp[c..]).unwrap();
        destination_savesTemp = destination_savesTempStr.chars().filter(|&c| !":".contains(c)).collect::<String>(); //remove : using this hella thing

        //destination_savesTemp = &destination_savesTemp[destination_savesTemp.find(":").unwrap()..fcmdOutput.len()]

        destination_saves[i as usize] = format!("{}{}\\{}",destinationUNC,compName,destination_savesTemp); //build the destination save names
    }

    //REPORT
    println!("Destination Saves:");
    for i in 0..saves_number
    {
        println!("{}",destination_saves[i as usize]);
    }


    // ***************************CREATE LOG FILE LOCATION + NAME*************************************************************
    let pathNoExeStr = &pathLocalStr[0..pathLocalStr.rfind(".").unwrap()]; //remove the .exe at the end
    let pathNoExe = pathNoExeStr.to_string(); //convert ot string
    //let pathExeLog = format!("\"{}{}\"",pathNoExe,pathExeLogPostfix); //build the log file path with "'s around it
    let pathExeLogNoParenth = format!("{}{}",pathNoExe,pathExeLogPostfix); //build the log file path with no "'s
    let pathExeLogNoParenthStr: &str = &*pathExeLogNoParenth; //these two types of strings are hella annoying

    let roboWarningLogSavePathTempStr = &pathLocalStr[0..pathLocalStr.rfind("\\").unwrap()]; //Gets the exe path, cuts off the \Backup.exe
    //println!("TEST-log path:{}\nlog path w/o \"'s:{}",pathExeLog,pathExeLogNoParenth);


    // ***************************CREATE COMMAND AND CALL IT*************************************************************
    //let roboWarningLogNumberFinder = (((saves_number as f64).log10()).floor() + (1 as f64)) as u64; //nifty calc for number of number places, e.g. 10 is 2, 100 is 3

    println!("\nBEGINNING BACKUP");
    for i in 0..saves_number
    {
        //let command = format!("\"{}\" \"{}\" {}{}",saves_locales[i as usize],destination_saves[i as usize],commandOptsComb,pathExeLog); //build the command
        //let command = format!("{} {} {} {}{}",commandRobo,saves_locales[i as usize],destination_saves[i as usize],commandOptsComb,pathExeLogNoParenth); //build the command
        //let commandStr: &str = &*command; //these two types of strings are hella annoying

        //let commandSave = format!("\"{}\"",saves_locales[i as usize]);
        let commandSave = format!("{}",saves_locales[i as usize]); //no "'s
        let commandSaveStr: &str = &*commandSave; //these two types of strings are hella annoying
        //let commandDest = format!("\"{}\"",destination_saves[i as usize]);
        let commandDest = format!("{}",destination_saves[i as usize]); //no "'s
        let commandDestStr: &str = &*commandDest; //these two types of strings are hella annoying
        //let commandLog = format!("/LOG:{}",pathExeLog);
        let commandLog = format!("/LOG:{}",pathExeLogNoParenth); //no "'s
        let commandLogStr: &str = &*commandLog; //these two types of strings are hella annoying

        //println!("TEST-Command #{} for robocopy:{}",i,command);

        println!("*********************Backing up: {}*********************",saves_locales[i as usize]);

        //-----Alg for updating the tooltip in Rust-----
        trayToolTip = format!("Backing up {}/{}: {}",i+1,saves_number,saves_locales[i as usize]); //update the tooltip string
        trayToolTipInt = [0; MAX_SZTIP_LEN as usize]; //fill with 0's
        let trayToolTipStrStep: &str = &*trayToolTip; //these two types of strings are hella annoying
        trayToolTipStepOS = OsStr::new(trayToolTipStrStep); //convert to OS string format or something
        trayToolTipStepUTF16 = trayToolTipStepOS.encode_wide().collect::<Vec<u16>>(); //now actually convert to UTF16 format for the OS
        if( trayToolTipStepUTF16.len() > ((MAX_SZTIP_LEN-1) as usize) ) //leave room for null at the end (check UTF-16 for length b/c that's what matters to the OS!)
        {
            //If the length is greater than MAX_SZTIP_LEN-1 then the null at the end is def gone and the letters need to be elipsised away...
            trayToolTipStepUTF16.truncate(MAX_SZTIP_LEN as usize); //make sure it isn't bigger than MAX_SZTIP_LEN
            //trayToolTipStepUTF16.splice( ((MAX_SZTIP_LEN-3) as usize)..(MAX_SZTIP_LEN as usize), ); //vecs are dumb
            trayToolTipStepUTF16[ (MAX_SZTIP_LEN as usize) - 3 ] = 46; //force a .
            trayToolTipStepUTF16[ (MAX_SZTIP_LEN as usize) - 2 ] = 46; //force a . 
            trayToolTipStepUTF16[ (MAX_SZTIP_LEN as usize) - 1 ] = 0; //force a null terminator at the end
            //making sure this doesn't do anything weird with UTF16 characters that are 2xUTF16s together is hard and I can't test it, so I won't sorry
        }
        trayToolTipInt[..trayToolTipStepUTF16.len()].copy_from_slice(&trayToolTipStepUTF16); //record it in that nice integer holder
        nid.szTip = trayToolTipInt; //tooltip for the icon
        unsafe{ winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_MODIFY, &mut nid) }; //updates system tray icon
        //-----end alg for updating the tooltip in Rust-----
        
        //let m = Command::new("robocopy").arg(commandStr).output().unwrap(); //one shot does not work here
        let robo = Command::new("robocopy")
            .arg(commandSaveStr)
            .arg(commandDestStr)
            .arg(commandOpt1)
            .arg(commandOpt2)
            .arg(commandOpt3)
            .arg(commandOpt4)
            .arg(commandOpt5)
            .arg(commandOpt6)
            .arg(commandOpt7)
            .arg(commandLogStr)
            .output().unwrap(); //run cmd.exe net use 
        let roboOutput = String::from_utf8_lossy(&robo.stdout); //get the output
        //let roboOutputStr: &str = &*roboOutput; //these two types of strings are hella annoying
        //println!("TEST-robo output: {}",roboOutput);
        //println!("TEST-robo stderr: {}", String::from_utf8_lossy(&robo.stderr)); //reports error words of command
        //println!("TEST-robo status: {}", robo.status); //reports success (true or false) of command
        let roboStatus= format!("{:?}", robo.status); //get the success code, it's a string that looks like ExitStatus(ExitStatus(#))
        let mut roboStatusStr: &str = &*roboStatus; //these two types of strings are hella annoying
        roboStatusStr = &roboStatusStr[roboStatusStr.find("(").unwrap()+1..roboStatusStr.len()]; //cut off some more now - ExitStatus(#))
        roboStatusStr = &roboStatusStr[roboStatusStr.find("(").unwrap()+1..roboStatusStr.len()]; //cut off some more now - #))
        roboStatusStr = &roboStatusStr[0..roboStatusStr.find(")").unwrap()]; //cut off some more now - #
        //finish it out
        m = roboStatusStr.to_string().parse::<u64>().unwrap(); //convert to int
        //println!("TEST-robo direct output: {}", m); //reports success (true or false) of command
        //let _ = Command::new("cmd.exe").arg("/c").arg("pause").status(); // wait for user input

        //println!("###########  for robo, m was {}  ###############",m);

        //ROBOCOPY ERROR CODES (some aren't errors tho ayy) - from https://blogs.technet.microsoft.com/deploymentguys/2008/06/16/robocopy-exit-codes/
        //Code 	Meaning
        //0 	No errors occurred and no files were copied.
        //1 	One of more files were copied successfully.
        //2 	Extra files or directories were detected.  Examine the log file for more information.
        //4 	Mismatched files or directories were detected.  Examine the log file for more information.
        //8 	Some files or directories could not be copied and the retry limit was exceeded.
        //16 	Robocopy did not copy any files.  Check the command line parameters and verify that Robocopy has enough rights to write to the destination folder.
        //123   Incorrect snytax on log file path. Robocopy won't run.

        //I HANDLE 8/16 as issues (idk if they all are - 16 is for sure), and support two specific errors in 16
        if( m == 2 || m == 3 )
        {
            println!("\nWARNING in Robocopy: Extra files or directories detected - not sure? But I think it is ok.");
        }
        else if(m == 4 || m == 5 || m == 6 || m == 7)
        {
            println!("\nWARNING in Robocopy: Mismtached files or directories detected - I think it is deleting files that don't exist in the source anymore, should be ok but noting!");
        }
        else if(m == 8 || m == 9 || m == 10 || m == 11 || m == 12 || m == 13 || m == 14 || m == 15)
        {
            let roboWarningLogSave = format!("{:02}{}", i+1,roboWarningLogSaveAddition); //create log file name in format ##FILECOPYFAILURE.log e.g. 04FILECOPYFAILURE.log to show 4th place is failure
            //let roboWarningLogSaveStr: &str = &*roboWarningLogSave; //these two types of strings are hella annoying

            println!("\nWARNING in Robocopy: Failed to copy some files but other files copied successfully. Re-running Backup.exe again may fix this issue (idk bb it did when I made this code to handle this)");
            println!("The source directory was {}\nThe log file can be found in {} (corresponds to line # in the Backup_saveLocales.txt list)\nLog file is in same directory as main log file and Backup.exe\nCONTINUING ON\n",saves_locales[i as usize],roboWarningLogSave);
                        
            let roboWarningLogSavePathRenameStr = format!("{}\\{}",roboWarningLogSavePathTempStr,roboWarningLogSave); //makes full path to the ##FILECOPYFAILURE.log
            let roboWarningLogSavePathRename = roboWarningLogSavePathRenameStr.to_string(); //convert ot string for printing later b/c string crap

            //Rename the file instead of copying it to a new file - had crashes copying large files (~30MB) with the Rust std::fs::read_to_string - should be faster anyway
            match std::fs::rename(pathExeLogNoParenthStr,roboWarningLogSavePathRenameStr) //match to catch error or OK
            { 
                Err(_why) => 
                {
                    unsafe{ winapi::um::winuser::ShowWindow(hWnd(), winapi::um::winuser::SW_SHOWNA) }; //bring the window back into the world because of an issue/error
                    println!("\n************************************************************************************");
                    println!("ERROR TRYING TO RENAME ROBOCOPY LOG OF A FILE COPY FAILURE: COULDN'T RENAME FILE, IDK WHY IT FAILED TO MAKE RENAME THE LOG FILE FROM\n{}\n TO \n{}\n IN THE Backup.exe directory. Exiting - fix it pls.",pathExeLogNoParenthStr,roboWarningLogSavePathRename);
                    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status(); // wait for user input
                    unsafe{ winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_DELETE, &mut nid) }; //deletes system tray icon when done
                    return 3; //return 3 for sass
                },
                Ok(roboWarningLogSavePathRename) => roboWarningLogSavePathRename, //I just put something here cause just 1 made it >:V angry idk why
            };
            
        }
        else if(m == 16) //general error check is occuring - specific ones are checked for and if not found the general error is issued
        {
            let roboWarningLogSave = format!("{:02}{}", i+1,roboWarningLogSaveAddition); //create log file name in format ##FILECOPYFAILURE.log e.g. 04FILECOPYFAILURE.log to show 4th place is failure
            //let roboWarningLogSaveStr: &str = &*roboWarningLogSave; //these two types of strings are hella annoying

            println!("\nWARNING in Robocopy: Failed to copy some files but other files copied successfully. Re-running Backup.exe again may fix this issue (idk bb it did when I made this code to handle this)");
            println!("The source directory was {}\nThe log file can be found in {} (corresponds to line # in the Backup_saveLocales.txt list)\nLog file is in same directory as main log file and Backup.exe\nCONTINUING ON\n",saves_locales[i as usize],roboWarningLogSave);
                        
            let roboWarningLogSavePathRenameStr = format!("{}\\{}",roboWarningLogSavePathTempStr,roboWarningLogSave); //makes full path to the ##FILECOPYFAILURE.log
            let roboWarningLogSavePathRename = roboWarningLogSavePathRenameStr.to_string(); //convert ot string for printing later b/c string crap

            //Rename the file instead of copying it to a new file - had crashes copying large files (~30MB) with the Rust std::fs::read_to_string - should be faster anyway
            match std::fs::rename(pathExeLogNoParenthStr,roboWarningLogSavePathRenameStr) //match to catch error or OK
            { 
                Err(_why) => 
                {
                    unsafe{ winapi::um::winuser::ShowWindow(hWnd(), winapi::um::winuser::SW_SHOWNA) }; //bring the window back into the world because of an issue/error
                    println!("\n************************************************************************************");
                    println!("ERROR TRYING TO RENAME ROBOCOPY LOG OF A FILE COPY FAILURE: COULDN'T RENAME FILE, IDK WHY IT FAILED TO MAKE RENAME THE LOG FILE FROM\n{}\n TO \n{}\n IN THE Backup.exe directory. Exiting - fix it pls.",pathExeLogNoParenthStr,roboWarningLogSavePathRename);
                    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status(); // wait for user input
                    unsafe{ winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_DELETE, &mut nid) }; //deletes system tray icon when done
                    return 3; //return 3 for sass
                },
                Ok(roboWarningLogSavePathRename) => roboWarningLogSavePathRename, //I just put something here cause just 1 made it >:V angry idk why
            };

            if( roboOutput.contains(roboError2) ) //check if output contains error 2
            {
                unsafe{ winapi::um::winuser::ShowWindow(hWnd(), winapi::um::winuser::SW_SHOWNA) }; //bring the window back into the world because of an issue/error
                println!("\n************************************************************************************");
                println!("ERROR 2 IN ROBOCOPY (Error code {}): Source location of '{}' was not found to exist.\nThe log file can be found in {} (corresponds to line # in the Backup_saveLocales.txt list)\nBackup of that folder has failed. Exiting - fix it pls, (SEE LOG FILE!).",m,saves_locales[i as usize],roboWarningLogSave);
                let _ = Command::new("cmd.exe").arg("/c").arg("pause").status(); // wait for user input
                unsafe{ winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_DELETE, &mut nid) }; //deletes system tray icon when done
                return 3; //return 3 for sass
            }
            else if( roboOutput.contains(roboError3) ) //check if output contains error 3
            {
                unsafe{ winapi::um::winuser::ShowWindow(hWnd(), winapi::um::winuser::SW_SHOWNA) }; //bring the window back into the world because of an issue/error
                println!("\n************************************************************************************");
                println!("ERROR 3 IN ROBOCOPY (Error code {}): Destination drive of '{}' was not found to exist.\nThe log file can be found in {} (corresponds to line # in the Backup_saveLocales.txt list)\nBackup of any files and folders has failed. Exiting - fix it pls, (SEE LOG FILE!).",m,destination,roboWarningLogSave);
                let _ = Command::new("cmd.exe").arg("/c").arg("pause").status(); // wait for user input
                unsafe{ winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_DELETE, &mut nid) }; //deletes system tray icon when done
                return 3; //return 3 for sass
            }
            else if( roboOutput.contains(roboError123) ) //check if output contains error 123
            {
                unsafe{ winapi::um::winuser::ShowWindow(hWnd(), winapi::um::winuser::SW_SHOWNA) }; //bring the window back into the world because of an issue/error
                println!("\n************************************************************************************");
                println!("ERROR 123 IN ROBOCOPY (Error code {}): Invalid log file path '{}'.\nThe log file can be found in {} (corresponds to line # in the Backup_saveLocales.txt list)\nBackup of any files and folders has failed. Exiting - fix it pls, (SEE LOG FILE!).",m,destination,roboWarningLogSave);
                let _ = Command::new("cmd.exe").arg("/c").arg("pause").status(); // wait for user input
                unsafe{ winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_DELETE, &mut nid) }; //deletes system tray icon when done
                return 3; //return 3 for sass
            }
            else
            {
                //let pathExeLogNoParenthPath = std::path::Path::new(pathExeLogNoParenthStr); //prep a path special thing (this fixes permissions crap)
                let fLogOutput = std::fs::read_to_string(roboWarningLogSavePathRename).expect("ERROR: Unable to read file. Giving up soon."); //read log

                unsafe{ winapi::um::winuser::ShowWindow(hWnd(), winapi::um::winuser::SW_SHOWNA) }; //bring the window back into the world because of an issue/error
                println!("\n************************************************************************************");
                println!("ERROR IN ROBOCOPY: Not sure what the error code is exactly, but there's a bunch. Here's the output from Robocopy, and the log file will have very similar info.\n{}",fLogOutput);
                println!("The log file can be found in {} (corresponds to line # in the Backup_saveLocales.txt list)\nBackup of any files and folders has failed. Exiting - fix it pls, (SEE LOG FILE!).",roboWarningLogSave);
                let _ = Command::new("cmd.exe").arg("/c").arg("pause").status(); // wait for user input
                unsafe{ winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_DELETE, &mut nid) }; //deletes system tray icon when done
                return 3; //return 3 for sass
            }
            
        }
        else if( m > 16)
        {
            let roboWarningLogSave = format!("{:02}{}", i+1,roboWarningLogSaveAddition); //create log file name in format ##FILECOPYFAILURE.log e.g. 04FILECOPYFAILURE.log to show 4th place is failure
            //let roboWarningLogSaveStr: &str = &*roboWarningLogSave; //these two types of strings are hella annoying

            println!("\nWARNING in Robocopy: Failed to copy some files but other files copied successfully. Re-running Backup.exe again may fix this issue (idk bb it did when I made this code to handle this)");
            println!("The source directory was {}\nThe log file can be found in {} (corresponds to line # in the Backup_saveLocales.txt list)\nLog file is in same directory as main log file and Backup.exe\nCONTINUING ON\n",saves_locales[i as usize],roboWarningLogSave);
                        
            let roboWarningLogSavePathRenameStr = format!("{}\\{}",roboWarningLogSavePathTempStr,roboWarningLogSave); //makes full path to the ##FILECOPYFAILURE.log
            let roboWarningLogSavePathRename = roboWarningLogSavePathRenameStr.to_string(); //convert ot string for printing later b/c string crap

            //Rename the file instead of copying it to a new file - had crashes copying large files (~30MB) with the Rust std::fs::read_to_string - should be faster anyway
            match std::fs::rename(pathExeLogNoParenthStr,roboWarningLogSavePathRenameStr) //match to catch error or OK
            { 
                Err(_why) => 
                {
                    unsafe{ winapi::um::winuser::ShowWindow(hWnd(), winapi::um::winuser::SW_SHOWNA) }; //bring the window back into the world because of an issue/error
                    println!("\n************************************************************************************");
                    println!("ERROR TRYING TO RENAME ROBOCOPY LOG OF A FILE COPY FAILURE: COULDN'T RENAME FILE, IDK WHY IT FAILED TO MAKE RENAME THE LOG FILE FROM\n{}\n TO \n{}\n IN THE Backup.exe directory. Exiting - fix it pls.",pathExeLogNoParenthStr,roboWarningLogSavePathRename);
                    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status(); // wait for user input
                    unsafe{ winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_DELETE, &mut nid) }; //deletes system tray icon when done
                    return 3; //return 3 for sass
                },
                Ok(roboWarningLogSavePathRename) => roboWarningLogSavePathRename, //I just put something here cause just 1 made it >:V angry idk why
            };

            unsafe{ winapi::um::winuser::ShowWindow(hWnd(), winapi::um::winuser::SW_SHOWNA) }; //bring the window back into the world because of an issue/error
            println!("\n************************************************************************************");
            println!("ERROR IN ROBOCOPY: Not sure what but the exit code wasn't 0 or 1. YOUR ERROR CODE: {}\n0 = OK, no new files\n1 = OK, new files\n2 = Extra files/directories detected see log (not error)",m);
            println!("4 = Mismatched files/directories detected see log (not error)\n8 = Some files/directories could not be copied and exceeded retry limit (seems solid error zone less srry)");
            println!("16 = nothing copied due to issue, check log.\nThe log file can be found in {} (corresponds to line # in the Backup_saveLocales.txt list)\nExiting - fix it pls, (SEE LOG FILE!).",roboWarningLogSave);
            let _ = Command::new("cmd.exe").arg("/c").arg("pause").status(); // wait for user input
            unsafe{ winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_DELETE, &mut nid) }; //deletes system tray icon when done
            return 3; //return 3 for sass
        }
    }

    println!("\n************************************************************************************");
    println!("BACKUP WAS COMPLETED SUCCESSFULLY AYY");
    println!("\n************************************************************************************");


    //-----Alg for updating the tooltip in Rust-----
    trayToolTip = "Basic Bitch Backup has completed successfully".to_string(); //update the tooltip string
    trayToolTipInt = [0; MAX_SZTIP_LEN as usize]; //fill with 0's
    let trayToolTipStrStep: &str = &*trayToolTip; //these two types of strings are hella annoying
    trayToolTipStepOS = OsStr::new(trayToolTipStrStep); //convert to OS string format or something
    trayToolTipStepUTF16 = trayToolTipStepOS.encode_wide().collect::<Vec<u16>>(); //now actually convert to UTF16 format for the OS
    if( trayToolTipStepUTF16.len() > ((MAX_SZTIP_LEN-1) as usize) ) //leave room for null at the end (check UTF-16 for length b/c that's what matters to the OS!)
        {
            //If the length is greater than MAX_SZTIP_LEN-1 then the null at the end is def gone and the letters need to be elipsised away...
            trayToolTipStepUTF16.truncate(MAX_SZTIP_LEN as usize); //make sure it isn't bigger than MAX_SZTIP_LEN
            //trayToolTipStepUTF16.splice( ((MAX_SZTIP_LEN-3) as usize)..(MAX_SZTIP_LEN as usize), ); //vecs are dumb
            trayToolTipStepUTF16[ (MAX_SZTIP_LEN as usize) - 3 ] = 46; //force a . (apparently 46 in decimal!)
            trayToolTipStepUTF16[ (MAX_SZTIP_LEN as usize) - 2 ] = 46; //force a . (apparently 46 in decimal!)
            trayToolTipStepUTF16[ (MAX_SZTIP_LEN as usize) - 1 ] = 0; //force a null terminator at the end
            //making sure this doesn't do anything weird with UTF16 characters that are 2xUTF16s together is hard and I can't test it, so I won't sorry
        }
    trayToolTipInt[..trayToolTipStepUTF16.len()].copy_from_slice(&trayToolTipStepUTF16); //record it in that nice integer holder
    nid.szTip = trayToolTipInt; //tooltip for the icon
    unsafe{ winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_MODIFY, &mut nid) }; //updates system tray icon
    //-----end alg for updating the tooltip in Rust-----

    println!("\nWaiting 5 seconds so you can read some stuff and then exiting.");
    let _ = Command::new("cmd.exe").arg("/c").arg("timeout 5").status();
    unsafe{ winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_DELETE, &mut nid) }; //deletes system tray icon when done

    return 0; //finish it out
}







