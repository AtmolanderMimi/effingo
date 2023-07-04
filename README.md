# Effingo

a command line tool for making backups that makes sure that the contents of links are kept with the backup. Based on the word copy in Latin.

For the sake of this README `symbiotic links` (Linux) and `.lnk` files (Windows) will be referred to as links.

**WARNING: THIS IS A PERSONAL PROJECT, I DO NOT TAKE ANY RESPONSABILITY FOR ANY BUGS THAT MIGHT MESS UP YOUR FILE ORGANIZATION**

Started 2023-06-17

## Usage

Simply use the effingo command and provide it with the `directory to copy` and `the path to copy to`:
```text
C:\Users\Admin> effingo <copy_dir> <target_dir>
copied -> target
```

If you are compiling:
```text
C:\Users\Admin\effingo> cargo run -- <copy_dir> <target_dir>
copied -> target
```

Effingo will copy all the files and directories within the `copy_dir` and will also add the files/directories that the links are pointing to.

Think of it like the copy_dir is a blueprint that references directories and
other files in your file system in one place through links.
So that we you have to create a backup, you won't need to shuffle around to be
sure you have backed up all you want.

## Known Limitations
* Links pointing to entries using unexpected characters such as: letters with accents, symbols and others will not be copied.
* If the target directory already contains an entry of the same name as one that is being copied the program will run into an error.
* Functionality on operating systems other then Windows 10 and Linux is not tested
* Link copy depth is only of 1 to prevent an infinite loop (if a link points to a directory containing another link the file the link is referring to will not be copied)
