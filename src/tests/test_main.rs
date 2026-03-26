#[cfg(test)]
mod main_tests {
    use crate::{
        app::{AppState, AppStatus},
        can_restart_cargo_run, cli, prepare_cargo_run_restart, try_get_matches_from,
    };
    use clap::error::ErrorKind;

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
    fn update_subcommand_only_matches_in_first_argument() {
        let update =
            try_get_matches_from(["claude-chat-code", "update"]).expect("update should parse");
        assert_eq!(update.subcommand_name(), Some("update"));

        let no_subcommand =
            try_get_matches_from(["claude-chat-code"]).expect("empty args should parse");
        assert_eq!(no_subcommand.subcommand_name(), None);

        assert!(try_get_matches_from(["claude-chat-code", "--help", "update"]).is_err());
    }

    #[test]
    fn help_flags_return_display_help() {
        let long_help = try_get_matches_from(["claude-chat-code", "--help"]).unwrap_err();
        assert_eq!(long_help.kind(), ErrorKind::DisplayHelp);

        let short_help = try_get_matches_from(["claude-chat-code", "-h"]).unwrap_err();
        assert_eq!(short_help.kind(), ErrorKind::DisplayHelp);
    }

    #[test]
    fn help_text_contains_usage_and_update_command() {
        let mut command = cli();
        let help = command.render_long_help().to_string();

        assert!(help.contains("claude-chat-code"));
        assert!(help.contains("Usage:"));
        assert!(help.contains("update"));
        assert!(help.contains("--help"));
    }
}
