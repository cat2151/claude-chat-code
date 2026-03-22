#[cfg(test)]
mod updater_tests {
    use crate::updater::needs_update;

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
}
