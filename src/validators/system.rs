use std::sync::LazyLock;

use fancy_regex::Regex;

static REGISTRY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        &[
            r"(?mxi)",
            r"(?!^.*?(?:\^.*$",
            r"|\[\d\-\d\]|\{\d*\,\d*\}))((^[^\n]*?)((^\\*|\\+)",
            r"(HK(EY|LM|CU|U|CC|CR))",
            r"(\\+|_[^\n]+?\\+)|",
            r"^(BCD[-\n]+|",
            r"COMPONENTS|DRIVERS|ELAM|HARDWARE|SAM|Schema|SECURITY|SOFTWARE|SYSTEM|AppEvents|Console|Control( Panel)?",
            r"|Secrets|Environment|EUDC|Keyboard Layout|Network|Printers|Uninstall|Volatile Environment)\\|",
            r"(^\\*|\\+)S\-\d+[^\n]*?\\+|",
            r"\[(Install Path|[Music Path]|Pictures Path|Videos Path|Artist|App Data Path|Name)\]|",
            "\"AppliesTo\"|\"AssociateFiles\"|",
            r"\[App Data Path\]|",
            "\"Common\"|\"CommonEmojiTerminators\"|\"Complete\"|",
            "\"(Configuration|DefaultFeature|Description|DiskPrompt|DocumentationShortcuts|EnvironmentPathNode|EnvironmentPathNpmModules|Extensions|External Program Arguments|File Location.*?|MainApplication|MainFeature|NodeRuntime|Path|Servicing_Key|Shortcuts)\")",
            r"([^\n#]*?)\\*\S+)",
            r"($|[^\\]+$)",
        ]
        .join("")
    ).unwrap()
});
static SQL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        &[
            r"(?xmi)(",
            r"^[^\S\n]*",
            r"(",
            r"INSERT[^\S\n]+((?!^[^\S\n]*$)[\s\S])+?[^\S\n]+VALUES[^\S\n]+.*",
            r"|FROM[^\S\n]+((?!^[^\S\n]*$)[\s\S])+?[^\S\n]+WHERE[^\S\n]+.*",
            r"|(SELECT|PATINDEX)[^\S\n]+((?!^[^\S\n]*$)[\s\S])+?[^\S\n]+(WHERE|FROM|AS)[^\S\n]+.*",
            r"|BEGIN\b((?!^[^\S\n]*$)[\s\S])+?\bEND\b.*",
            r"|(SELECT|UPDATE|DELETE|INSERT[ ]INTO|CREATE[ ]DATABASE|ALTER[ ]DATABASE|CREATE[ ]TABLE|ALTER[ ]TABLE|DROP[ ]TABLE|CREATE[ ]INDEX|DROP[ ]INDEX|DECLARE|SET|TRUNCATE|ADD|WHERE)[^\S\n]+",
            r".*",
            r")",
            r"(\(?((?!^[^\S\n]*$)[\s\S])+?\)[^)\n]*)?",
            r"(\n|$)",
            r")+",
        ]
        .join("")
    ).unwrap()
});
static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        &[r"^.*?(\[\^.*?\]|\[[^\[]+?\-[^\[]+?\]|^\^|\{\d*,?\d*\}|\$$|[\(\)\]]\?).*"].join(""),
    )
    .unwrap()
});
static FILE_PATH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        &[
            r"(?ix)^",
            r"(?:",
            r"[a-z]:[\\\/]",
            r"|[\\\/]+|\.{1,2}[\\\/]+",
            r")",
            r"(?:[^\\\/\n<>:",
            "\"",
            r"\|\?\*]*[\\\/])*",
            r"[^\\\/\.\n<>:",
            "\"",
            r"\|\?\*]*?",
            r"[^\\\/\.\s<>:",
            "\"",
            r"\|\?\*]",
            r"\.\w{3,4}$",
        ]
        .join(""),
    )
    .unwrap()
});

pub fn is_registry_key(value: &str) -> bool {
    //! Checks to see if a Given String is a registry key
    if value.is_empty() || !value.contains('\\') {
        return false;
    }
    REGISTRY.is_match(value).unwrap_or_default()
}

pub fn is_sql(value: &str) -> bool {
    //! Checks to see if a Given String is a SQL statement
    if value.is_empty() {
        return false;
    }

    SQL.is_match(value).unwrap_or_default()
}

pub fn is_regex(value: &str) -> bool {
    //! Checks to see if a Given String is a Regular Expression
    if value.is_empty() {
        return false;
    }

    REGEX.is_match(value).unwrap_or_default() && Regex::new(value).is_ok()
}

pub fn is_file_path(value: &str) -> bool {
    //! Checks to see if a Given String is a File Path
    if value.is_empty() {
        return false;
    }

    FILE_PATH.is_match(value).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_registry_key() {
        // invalid
        assert!(is_registry_key(
            "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion"
        ));
        assert!(is_registry_key(
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion"
        ));
        assert!(is_registry_key(
            "HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion"
        ));
        assert!(is_registry_key(
            "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion"
        ));

        // invalid
        assert!(!is_registry_key("This\nIs\\aRegistryKey"));
        assert!(!is_registry_key("^[U][0-9]{12,15}$"));
    }

    #[test]
    fn test_is_sql() {
        // invalid
        assert!(is_sql("SELECT * FROM xyz"));
        assert!(is_sql("SELECT * FROM xyz WHERE x LIKE '%y%';"));
        assert!(is_sql(
            "INSERT INTO Country(CountryID,CountryName) VALUES (1,'United States')"
        ));
        assert!(is_sql(
            r#"CREATE TABLE table_name (
        column1 INTEGER,
        column2 VARCHAR2,
        column3 INTEGE);"#
        ));

        // invalid
        assert!(!is_registry_key(
            "Select this is NOT a SQL Query WHERE it could get selected="
        ));
    }

    #[test]
    fn test_is_regex() {
        // valid
        assert!(is_regex(r"^[U][0-9]{12,15}$"));
        assert!(is_regex(r"a{12,15}$"));
        assert!(is_regex(r"//[^\r\n]*[\r\n]"));
        assert!(is_regex(r"[^i*&2@]"));
        assert!(is_regex(r"^dog"));
        assert!(is_regex(r"cat$"));
    }

    #[test]
    fn test_is_file_path() {
        // valid
        assert!(is_file_path("/home/user/Documents/foo.rar"));
        assert!(is_file_path("../test.exe"));
        assert!(is_file_path("\\\\Server2\\Share\\Test\\Foo.exe"));
        assert!(is_file_path("C:\\Program Files\\foo\\bar.exe"));
        assert!(is_file_path("c:\\Personal\\MyFolder\\MyFile.exe"));
        assert!(is_file_path("..\\..\\folder\\file.exe"));
        assert!(is_file_path("z:\\folder\\file.jpeg"));

        // invalid
        assert!(!is_file_path(
            "HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion"
        ))
    }
}
