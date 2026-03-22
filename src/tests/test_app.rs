#[cfg(test)]
mod app_tests {
    use crate::app::{AppState, AppStatus};

    #[test]
    fn push_log_respects_max_200_lines() {
        let mut state = AppState::new_for_test();
        for i in 0..250 {
            state.push_log(format!("line {}", i));
        }
        assert!(state.log.len() <= 200, "ログは最大200行であるべきだが {} 行あった", state.log.len());
    }

    #[test]
    fn push_log_contains_timestamp_prefix() {
        let mut state = AppState::new_for_test();
        state.log.clear();
        state.push_log("hello");
        let last = state.log.last().unwrap();
        assert!(last.starts_with('['), "ログ行はタイムスタンプで始まるべきだが: {:?}", last);
        assert!(last.contains("hello"), "メッセージが含まれていない: {:?}", last);
    }

    #[test]
    fn set_status_updates_status() {
        let mut state = AppState::new_for_test();
        state.set_status(AppStatus::Moving);
        assert_eq!(state.status, AppStatus::Moving);
    }

    #[test]
    fn app_status_is_error_and_is_done() {
        assert!(matches!(AppStatus::Error("x".into()), AppStatus::Error(_)));
        assert!(!matches!(AppStatus::Done("x".into()), AppStatus::Error(_)));
        assert!(matches!(AppStatus::Done("x".into()), AppStatus::Done(_)));
        assert!(!matches!(AppStatus::Error("x".into()), AppStatus::Done(_)));
    }

    #[test]
    fn app_status_label_is_nonempty() {
        let statuses = [
            AppStatus::Watching,
            AppStatus::ZipDetected("test.zip".into()),
            AppStatus::Moving,
            AppStatus::BackingUp,
            AppStatus::Extracting,
            AppStatus::Touching,
            AppStatus::Building,
            AppStatus::Error("err".into()),
            AppStatus::Done("ok".into()),
        ];
        for s in &statuses {
            assert!(!s.label().is_empty(), "label() が空: {:?}", s);
        }
    }
}
