:: mount the ISO File
PowerShell Mount-DiskImage -ImagePath "%USERPROFILE%\AppData\Roaming\launcher\iso\jak.iso"

:: extract the folders & files from the ISO File
xcopy G:\*.* %USERPROFILE%\AppData\Roaming\launcher\iso\jak1\. /E /Y

:: unmount the ISO File
PowerShell Dismount-DiskImage -ImagePath "%USERPROFILE%\AppData\Roaming\launcher\iso\jak.iso"