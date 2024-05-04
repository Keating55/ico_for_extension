# set-ico

A utility to setting ico ICONS for different extension files.

[microsoft_regedit](https://learn.microsoft.com/en-us/windows/win32/sysinfo/hkey-classes-root-key)

## Usage

```powershell
set-ico -e json -i D:\json.ico -a D:\notepad++.exe
```

### args

#### extension
file extension. `json`
#### ico
ico file absolute path.  `D:\json.ico`
#### app
app file absolute path. `D:\notepad++.exe`
#### root
default only write in HKCU  
add flag `-r` (**Administrator**) will write in HKLM and HKCU.

```powershell
set-ico -e json -i D:\json.ico -a D:\notepad++.exe -r
```

### del icon cache

if not effective please run `del_icon_cache.bat` to del icon cache
