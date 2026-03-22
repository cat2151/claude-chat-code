#[cfg(test)]
mod main_tests {
    use crate::{
        app::{AppState, AppStatus},
        can_restart_cargo_run, prepare_cargo_run_restart,
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
}
