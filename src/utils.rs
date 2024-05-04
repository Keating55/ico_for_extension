use winreg::enums::*;
use winreg::RegKey;

pub struct IcoReg {
    pub extension: String,
    pub ico: String,
    pub app: String,
    pub root: bool,
    pub name: String,
}

impl IcoReg {
    // check executable file extension
    pub fn check_extension(&self) -> bool {
        let white_extension = [
            "bat", "bin", "cab", "cmd", "com", "cpl", "ex_", "exe", "gadget", "inf", "ins", "inx",
            "isu", "job", "js", "jse", "lnk", "msc", "msi", "msp", "mst", "paf", "pif", "ps1",
            "reg", "rgs", "scr", "sct", "shb", "shs", "u3p", "vb", "vbe", "vbs", "vbscript", "ws",
            "wsf", "wsh",
        ];
        white_extension
            .binary_search(&self.extension.as_str())
            .is_ok()
    }
    ///HKEY_CLASSES_ROOT or HKEY_LOCAL_MACHINE\Software\Classes or HKEY_CURRENT_USER\Software\Classes
    /// 
    /// if you write values to a key under HKEY_CLASSES_ROOT, 
    /// and the key already exists under HKEY_CURRENT_USER\Software\Classes, 
    /// the system will store the information there instead of under HKEY_LOCAL_MACHINE\Software\Classes.
    fn get_root_reg(&self) -> RegKey {
        if self.root {
            return RegKey::predef(HKEY_LOCAL_MACHINE)
                .open_subkey(r#"Software\Classes"#)
                .expect(r#"failed HKEY_LOCAL_MACHINE\Software\Classes"#);
        } else {
            return RegKey::predef(HKEY_CURRENT_USER)
                .open_subkey(r#"Software\Classes"#)
                .expect(r#"failed HKEY_CURRENT_USER\Software\Classes"#);
        }
    }
    /// return custom_type {extension}file
    fn get_custom_type(&self) -> String {
        format!(r#"{}file"#, self.extension)
    }
    // set_reg
    pub fn set_reg(&self, reg_path: &str, reg_key: &str, reg_value: &str) {
        let key = self
            .get_root_reg()
            .create_subkey(reg_path)
            .expect(&format!("failed open {}", reg_path));
        key.0
            .set_value(reg_key, &reg_value)
            .expect(&format!("failed {} set value {}", reg_key, reg_value));
    }
    /// ```reg
    /// [Software\Classes\{custom_type}\DefaultIcon]
    /// @="{ico},0"
    /// ```
    fn reg_type_defaulticon(&self) -> String {
        return format!(r#"{}\DefaultIcon"#, self.get_custom_type());
    }
    /// ```reg
    /// [Software\Classes\{custom_type}\shell\open\command]
    /// @="\"{app_path}" \"%1\""
    /// ```
    fn reg_type_shell_open_command(&self) -> String {
        return format!(r#"{}\shell\open\command"#, self.get_custom_type());
    }
    /// ```reg
    /// [Software\Classes\.{extension}]
    /// @="{custom_type}"
    /// ```
    fn reg_extension(&self) -> String {
        return format!(r#".{}"#, self.extension);
    }
    /// ```reg
    /// [Software\Classes\.{extension}\DefaultIcon]
    /// @="{ico},0"
    /// ```
    fn reg_extension_defaulticon(&self) -> String {
        return format!(r#".{}\DefaultIcon"#, self.extension);
    }
    /// ```reg
    /// [HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\FileExts\.{}\UserChoice]
    /// "ProgId"="{progid}"
    /// ```
    fn reg_userchoice(&self) -> String {
        return format!(
            r#"Software\Microsoft\Windows\CurrentVersion\Explorer\FileExts\.{}\UserChoice"#,
            self.extension
        );
    }
    /// ```reg
    /// [HKEY_CURRENT_USER\Software\Classes\Applications\{app_name}\shell\open\command]
    /// @="{app_path}" "%1""
    /// ```
    fn reg_applications(&self) -> String {
        if self.root {
            format!(
                r#"Software\Classes\Applications\{}\shell\open\command"#,
                self.name
            )
        } else {
            format!(r#"Applications\{}\shell\open\command"#, self.name)
        }
    }

    pub fn set_file_extision_type(&self) {
        self.set_reg(
            &self.reg_type_defaulticon(),
            "",
            &format!("{},0", &self.ico),
        );
        self.set_reg(
            &self.reg_type_shell_open_command(),
            "",
            &format!(r#"{}"#, &format!(r#""{}" "%1""#, &self.app)),
        );
    }

    pub fn set_file_extision(&self) {
        self.set_reg(&self.reg_extension(), "", &self.get_custom_type());
        self.set_reg(
            &self.reg_extension_defaulticon(),
            "",
            &format!("{},0", &self.ico),
        );
    }

    pub fn set_create_applications(&self) {
        self.set_reg(
            &self.reg_applications(),
            "",
            &format!(r#""{}" "%1""#, &self.app),
        );
    }
    ///
    ///
    pub fn set_default_open_app(&self) {
        self.set_reg(
            &self.reg_userchoice(),
            "ProgId",
            &format!(r#"Applications\{}"#, self.name),
        );
    }

    pub fn new(extension: String, ico: String, app: String, root: bool, name: String) -> IcoReg {
        IcoReg {
            extension: extension.to_lowercase(),
            ico,
            app,
            root,
            name,
        }
    }
}
