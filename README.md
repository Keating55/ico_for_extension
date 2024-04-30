# set_ico

A utility to setting ico ICONS for different extension files.



write in  `HKEY_CURRENT_USER\Software\Classes` and `HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\FileExts\{ }\UserChoice`
```powershell

set_ico -e "json" -i "D:\json.ico" -a "D:\notepad++.exe" -p "Applications\notepad++.exe"

```
write in `HKEY_CLASSES_ROOT` and `HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\FileExts\{ }\UserChoice`

```powershell
set_ico -e "json" -i "D:\file.ico" -a "D:\notepad++.exe" -p "Applications\notepad++.exe" -r
```

## args

### extension
for example `json` , `go` ,`rs`,`py`
### ico
ico file path. for example `"D:\json.ico"`
### app
app file path. for example `"D:\notepad++.exe"`
### ProgID
find in `HKEY_CLASSES_ROOT\Applications\{ }` or `HKEY_CURRENT_USER\Software\Classes\Applications\{ }`

default: `"Applications\notepad++.exe"`  
### root
default `false`
if you want to write in `HKEY_CLASSES_ROOT`  
plese switch **Administrator** and add `-r`