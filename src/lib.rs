use winreg::enums::*;
use winreg::RegKey;

pub struct IcoReg {
    pub extension: String,
    pub ico: String,
    pub app: String,
    pub progid: String,
    pub root: bool,
}

impl IcoReg {
    ///HKEY_CLASSES_ROOT or HKEY_CURRENT_USER\Software\Classes
    fn get_root_reg(&self) -> RegKey {
        if self.root {
            return RegKey::predef(HKEY_CLASSES_ROOT);
        } else {
            return RegKey::predef(HKEY_CURRENT_USER)
                .open_subkey(r#"Software\Classes"#)
                .expect(r#"failed Software\Classes"#);
        }
    }
    /// return custom_type_{extension}
    fn get_custom_type(&self) -> String {
        return format!(r#"custom_type_{}"#, self.extension);
    }
    /// ```reg
    /// [HKEY_CLASSES_ROOT\{custom_type}\DefaultIcon]
    /// @="{ico},0"
    /// ```
    fn reg_defaulticon(&self) -> String {
        return format!(r#"{}\DefaultIcon"#, self.get_custom_type());
    }
    /// ```reg
    /// [HKEY_CLASSES_ROOT\{custom_type}\shell\open\command]
    /// @="\"{app}" \"%1\""
    /// ```
    fn reg_shell_open_command(&self) -> String {
        return format!(r#"{}\shell\open\command"#, self.get_custom_type());
    }
    /// ```reg
    /// [HKEY_CLASSES_ROOT\.{extension}]
    /// @="{custom_type}"
    /// ```
    fn reg_dot_extension(&self) -> String {
        return format!(r#".{}"#, self.extension);
    }
    /// ```reg
    /// [HKEY_CLASSES_ROOT\.{extension}\DefaultIcon]
    /// @="{ico},0"
    /// ```
    fn reg_dot_defaulticon(&self) -> String {
        return format!(r#".{}\DefaultIcon"#, self.extension);
    }
    /// ```reg
    /// [HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\FileExts\.{}\UserChoice]
    /// "ProgId"="{progid}"
    /// ```
    fn reg_fileexts_dot_userchoice(&self) -> String {
        return format!(
            r#"Software\Microsoft\Windows\CurrentVersion\Explorer\FileExts\.{}\UserChoice"#,
            self.extension
        );
    }

    pub fn set_file_extision_type(&self) {
        let root_reg = self.get_root_reg();

        let reg_1_key = &self.reg_defaulticon();
        let reg_1_value = &format!("{},0", &self.ico);
        let reg_1 = root_reg
            .create_subkey(reg_1_key)
            .expect(&format!("failed open {}", reg_1_key));
        reg_1
            .0
            .set_value("", reg_1_value)
            .expect(&format!("failed {} set value {},0", reg_1_key, reg_1_value));
        let reg_2_key = &self.reg_shell_open_command();
        let reg_2_value = &format!(r#""{}" "%1""#, &self.app);
        let reg_2 = root_reg
            .create_subkey(reg_2_key)
            .expect(&format!("failed open {}", reg_2_key));
        reg_2
            .0
            .set_value("", reg_2_value)
            .expect(&format!("failed {} set value {},0", reg_2_key, reg_2_value));
    }

    pub fn set_file_extision(&self) {
        let root_reg = self.get_root_reg();

        let reg_1_key = &self.reg_dot_extension();
        let reg_1_value = &self.get_custom_type();
        let reg_1 = root_reg
            .create_subkey(reg_1_key)
            .expect(&format!("failed open {}", reg_1_key));
        reg_1
            .0
            .set_value("", reg_1_value)
            .expect(&format!("failed {} set value {},0", reg_1_key, reg_1_value));
        let reg_2_key = &self.reg_dot_defaulticon();
        let reg_2_value = &format!("{},0", &self.ico);
        let reg_2 = root_reg
            .create_subkey(reg_2_key)
            .expect(&format!("failed open {}", reg_2_key));
        reg_2
            .0
            .set_value("", reg_2_value)
            .expect(&format!("failed {} set value {},0", reg_2_key, reg_2_value));
    }

    pub fn set_default_open_app(&self) {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);

        let reg_1_key = &self.reg_fileexts_dot_userchoice();
        let reg_1_value = &self.progid;
        let reg_1 = hkcu
            .create_subkey(reg_1_key)
            .expect(&format!("failed open {}", reg_1_key));
        reg_1
            .0
            .set_value("", reg_1_value)
            .expect(&format!("failed {} set_value {}", reg_1_key, reg_1_value));
    }

    pub fn new(extension: String, ico: String, app: String, progid: String, root: bool) -> IcoReg {
        IcoReg {
            extension,
            ico,
            app,
            progid,
            root,
        }
    }
}
