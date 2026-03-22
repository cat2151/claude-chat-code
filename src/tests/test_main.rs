#[cfg(test)]
mod main_tests {
    use crate::{
        app::{AppState, AppStatus},
        prepare_cargo_run_restart,
    };

    #[test]
    fn prepare_cargo_run_restart_sets_building_and_logs_f5_message() {
        let mut state = AppState::new_for_test();

        prepare_cargo_run_restart(&mut state);

        assert_eq!(state.status, AppStatus::Building);
        assert!(
            state.log.iter().any(|line| line.contains("F5 → cargo run を起動します")),
            "F5 再実行ログが追加されていない: {:?}",
            state.log
        );
    }
}
