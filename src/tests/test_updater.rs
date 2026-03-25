#[cfg(test)]
mod updater_tests {
    use crate::updater::{
        is_update_available, needs_update, start_update_script_args, update_bat_content,
    };

    #[test]
    fn needs_update_false_when_remote_is_none() {
        assert!(!needs_update(None));
    }

    #[test]
    fn needs_update_false_when_hashes_match() {
        let local = crate::updater::LOCAL_HASH;
        assert!(!needs_update(Some(local)));
    }

    #[test]
    fn needs_update_true_when_hashes_differ() {
        let local = crate::updater::LOCAL_HASH;
        if local != "unknown" {
            assert!(needs_update(Some(&"a".repeat(40))));
        }
    }

    #[test]
    fn needs_update_false_when_local_is_unknown() {
        let fake_remote = "b".repeat(40);
        let result = if crate::updater::LOCAL_HASH == "unknown" {
            !needs_update(Some(&fake_remote))
        } else {
            true
        };
        assert!(result);
    }

    #[test]
    fn is_update_available_true_when_hashes_differ() {
        assert!(is_update_available("01234567", "89abcdef"));
    }

    #[test]
    fn is_update_available_false_when_hashes_match_or_invalid() {
        assert!(!is_update_available("01234567", "01234567"));
        assert!(!is_update_available("unknown", "89abcdef"));
        assert!(!is_update_available("", "89abcdef"));
        assert!(!is_update_available("01234567", ""));
    }

    #[test]
    fn update_bat_content_contains_expected_steps() {
        let content = update_bat_content();
        assert!(content.contains("timeout /t 3 /nobreak >nul"));
        assert!(content.contains("cargo install --force --git https://github.com/cat2151/claude-chat-code"));
        assert!(content.contains("del \"%~f0\""));
    }

    #[test]
    fn start_update_script_args_passes_raw_bat_path_for_cmd_start() {
        let args = start_update_script_args(r"C:\Users\Test User\AppData\Local\Temp\update_script.bat");
        assert_eq!(args, vec![
            "/C".to_string(),
            "start".to_string(),
            "".to_string(),
            "C:\\Users\\Test User\\AppData\\Local\\Temp\\update_script.bat".to_string(),
        ]);
    }
}
