# Effingo

a command line tool for making backups that makes sure that the contents of shortcuts (`.lnk` files) are kept with the backup. Based on the word copy in latin.

Started 2023-06-17

## Usage

Simply use the effingo command and provide it with the `directory to copy` and `the path to copy to`:

```text
C:\Users\Admin> effingo <copy_dir> <target_dir>
                          copied  ->  target
```



If you are compiling:
```text
C:\Users\Admin\effingo> cargo run -- <copy_dir> <target_dir>
                                       copied  ->  target
```

Think of it like the copy_dir is a blueprint that references directories and
other files in your file system in one place through shortcuts (`.lnk` files).
So that we you have to create a backup, you won't need to suffle around to be
sure you have backed up all you want.

## Known Limitations
* Shortcuts (`.lnk` files) pointing to entries using unexpected characters such as: letters with accents, symbols and others will not be copied.
* If the target directory already contains an entry of the same name as one that is being copied the program will run into an error.
* Functionality on operating systems other then Windows 10 is not tested
* Shortcut (`.lnk` file) copy depth is only of 1 to prevent an infinite loop (if a `.lnk` points to a directory containing another `.lnk` the file the `.lnk` is referring to will not be copied)
