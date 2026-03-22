/// ファイル名のタイムスタンプサフィックスから経過時間ラベルを生成する。
/// backup_YYYYMMDD_HHMMSS や foo_YYYYMMDD_HHMMSS.zip 形式を対象とする。

use chrono::{DateTime, Local, NaiveDateTime, TimeZone};

/// ファイル名からタイムスタンプを解析し、経過時間ラベルを返す。
/// パース失敗時は空文字列を返す。
pub fn age_label(name: &str) -> String {
    match parse_ts_from_name(name) {
        Some(dt) => elapsed_label(dt),
        None      => String::new(),
    }
}

// ─── タイムスタンプ解析 ───────────────────────────────────────────────────────

/// ファイル名末尾の _YYYYMMDD_HHMMSS を抽出して DateTime に変換する
fn parse_ts_from_name(name: &str) -> Option<DateTime<Local>> {
    // 拡張子を除く stem を取得
    let stem = if let Some(pos) = name.rfind('.') {
        &name[..pos]
    } else {
        name
    };

    // 末尾 15 文字が _YYYYMMDD_HHMMSS（アンダースコア込み）= 16 文字
    // 例: backup_20250318_153042 → 末尾の "20250318_153042" を取る
    if stem.len() < 16 { return None; }
    let tail = &stem[stem.len() - 15..]; // "YYYYMMDD_HHMMSS"
    let sep  = &stem[stem.len() - 16..stem.len() - 15]; // "_"
    if sep != "_" { return None; }

    let naive = NaiveDateTime::parse_from_str(tail, "%Y%m%d_%H%M%S").ok()?;
    Local.from_local_datetime(&naive).single()
}

// ─── 経過時間ラベル ───────────────────────────────────────────────────────────

fn elapsed_label(dt: DateTime<Local>) -> String {
    let now     = Local::now();
    let elapsed = now.signed_duration_since(dt);
    let secs    = elapsed.num_seconds();

    if secs < 0 {
        return "now".to_string(); // 時刻ずれ対策
    }

    if secs < 60 {
        return "now".to_string();
    }

    let mins = elapsed.num_minutes();
    if mins < 60 {
        return format!("{}min", mins);
    }

    let hours = elapsed.num_hours();
    if hours < 24 {
        return format!("{}h", hours);
    }

    let days = elapsed.num_days();
    if days < 7 {
        return format!("{}d", days);
    }

    let weeks = days / 7;
    format!("{}w", weeks.min(99))
}

// ─── テスト ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    fn name_with_offset(prefix: &str, offset: Duration) -> String {
        let dt = Local::now() - offset;
        format!("{}_{}", prefix, dt.format("%Y%m%d_%H%M%S"))
    }

    #[test]
    fn now_within_59s() {
        let name = name_with_offset("backup", Duration::seconds(30));
        assert_eq!(age_label(&name), "now");
    }

    #[test]
    fn one_min() {
        let name = name_with_offset("backup", Duration::seconds(90));
        assert_eq!(age_label(&name), "1min");
    }

    #[test]
    fn one_hour() {
        let name = name_with_offset("backup", Duration::minutes(90));
        assert_eq!(age_label(&name), "1h");
    }

    #[test]
    fn one_day() {
        let name = name_with_offset("backup", Duration::hours(36));
        assert_eq!(age_label(&name), "1d");
    }

    #[test]
    fn one_week() {
        let name = name_with_offset("backup", Duration::days(8));
        assert_eq!(age_label(&name), "1w");
    }

    #[test]
    fn zip_with_extension() {
        let name = name_with_offset("foo", Duration::seconds(30)) + ".zip";
        assert_eq!(age_label(&name), "now");
    }

    #[test]
    fn unrelated_name_returns_empty() {
        assert_eq!(age_label("Cargo.toml"), "");
        assert_eq!(age_label("foo.zip"), "");
    }
}
