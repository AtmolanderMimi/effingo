# Effingo

a command line tool for making backups that make sure that the contents of shortcuts (`.lnk` files) are kept with the backup. Based on the word copy in latin.

Started 2023-06-17

## Usage

Simply use the effingo command and provide it with the `directory to copy` and `the place to copy`:

```text
C:\Users\Admin> effingo <copy_dir> <target_dir>
                        copied -> target
```

Think of it like the copy_dir is a blueprint that references directories and
other files in your file system in one place through shortcuts (`.lnk` files).
So that we you have to create a backup, you won't need to suffle around to be
sure you have backed up all you want.

## Limitations
* Entries that are named using unexpected characters such as: letters with accents, symbols and others will make the program panic.
* If the target directory already contains an entry of the same name as one that is copying the program will run into an error.
