# set-ico

A utility to setting ico ICONS for different extension files.


```powershell
set-ico -e json -i D:\json.ico -a D:\notepad++.exe
```

## args

### extension
file extension. `json`
### ico
ico file absolute path.  `D:\json.ico`
### app
app file absolute path. `D:\notepad++.exe`
### root
if you want to write in `HKEY_CLASSES_ROOT`,please switch **Administrator** and add `-r`

```powershell
set-ico -e json -i D:\json.ico -a D:\notepad++.exe -r
```

## del icon cache

if not effective please run `del_icon_cache.bat` to del icon cache