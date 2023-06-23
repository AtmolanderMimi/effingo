# Effingo

a command line tool for making backups that make sure that the contents of shortcuts are kept with the backup. Based on the word copy in latin.

Started 2023-06-17

## Usage

Simply use the effingo command and provide it with the `directory to copy` and `the place to copy`:

```text
C:\Users\Admin> effingo ./backup_dir D:\
                        copied -> target
```

## Limitations
Entries that are named using unexpected characters such as: letters with accents, symbols and others will make the program panic.
