#[cfg(test)]
mod main_tests {
    use crate::{
        app::{AppState, AppStatus},
        can_restart_cargo_run, is_help_flag, is_update_subcommand, HELP_TEXT,
        prepare_cargo_run_restart,
    };

    #[test]
    fn prepare_cargo_run_restart_sets_building_and_logs_f5_message() {
        let mut state = AppState::new_for_test();

        assert!(prepare_cargo_run_restart(&mut state));

        assert_eq!(state.status, AppStatus::Building);
        assert!(state.cargo_run_launching);
        assert!(
            state.log.iter().any(|line| line.contains("F5 → cargo run を起動します")),
            "F5 再実行ログが追加されていない: {:?}",
            state.log
        );
    }

    #[test]
    fn can_restart_cargo_run_is_false_while_busy() {
        let mut state = AppState::new_for_test();
        state.status = AppStatus::BackingUp;
        assert!(!can_restart_cargo_run(&state));

        state.status = AppStatus::Watching;
        state.cargo_run_launching = true;
        assert!(!can_restart_cargo_run(&state));
    }

    #[test]
    fn prepare_cargo_run_restart_rejects_when_processing() {
        let mut state = AppState::new_for_test();
        state.status = AppStatus::Extracting;

        assert!(!prepare_cargo_run_restart(&mut state));
        assert_eq!(state.status, AppStatus::Extracting);
        assert!(
            state.log.iter().any(|line| line.contains("F5 は現在の処理中は無効です")),
            "F5 無効ログが追加されていない: {:?}",
            state.log
        );
    }

    #[test]
    fn is_update_subcommand_only_matches_update_in_first_argument() {
        assert!(is_update_subcommand(&["claude-chat-code".into(), "update".into()]));
        assert!(!is_update_subcommand(&["claude-chat-code".into()]));
        assert!(!is_update_subcommand(&[
            "claude-chat-code".into(),
            "--help".into(),
            "update".into(),
        ]));
    }

    #[test]
    fn is_help_flag_only_matches_help_in_first_argument() {
        assert!(is_help_flag(&["claude-chat-code".into(), "--help".into()]));
        assert!(is_help_flag(&["claude-chat-code".into(), "-h".into()]));
        assert!(!is_help_flag(&["claude-chat-code".into()]));
        assert!(!is_help_flag(&[
            "claude-chat-code".into(),
            "update".into(),
            "--help".into(),
        ]));
    }

    #[test]
    fn help_text_contains_usage_and_update_command() {
        let help = HELP_TEXT;

        assert!(help.contains("claude-chat-code"));
        assert!(help.contains("USAGE:"));
        assert!(help.contains("update"));
        assert!(help.contains("--help"));
    }
}
