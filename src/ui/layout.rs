/// 上半分3ペインの動的幅計算に責任を持つ。
///
/// 優先順位:
///   1. 3つ全員欠落なし → 余白均等（全ペインが広くなる）
///   2. それ以外        → Backups固定(+1余白) + Watch/Archives 従来算出

const BORDER_PAD: u16 = 2; // 左右 border 各1
const AGE_COL:    u16 = 5; // "99min" の最大幅
const ITEM_PAD:   u16 = 1; // age と名前の間の空白

/// 3ペインの幅を返す: (watch_w, backup_w, archives_w)
pub fn upper_widths(
    total_w:       u16,
    watch_names:   &[String],
    backup_names:  &[String],
    archive_names: &[String],
    _watch_label:  &str,
) -> (u16, u16, u16) {
    let watch_need   = required_width(watch_names,   false);
    let backup_need  = required_width(backup_names,  true);
    let archive_need = required_width(archive_names, true);

    let watch_min   = watch_need   + BORDER_PAD;
    let backup_min  = backup_need  + BORDER_PAD;
    let archive_min = archive_need + BORDER_PAD;

    // ── 優先1: 3つ全員欠落なし → 余白均等 ───────────────────────────────────
    if watch_min + backup_min + archive_min <= total_w {
        let extra = total_w - watch_min - backup_min - archive_min;
        let add_each = extra / 3;
        let remainder = extra % 3;
        // 余りは watch → backup の順に1ずつ配分
        let w_add = add_each + if remainder > 0 { 1 } else { 0 };
        let b_add = add_each + if remainder > 1 { 1 } else { 0 };
        let a_add = add_each;
        return (watch_min + w_add, backup_min + b_add, archive_min + a_add);
    }

    // ── 優先2: 従来算出（Backups固定 +1、Watch/Archives 残り分割） ────────────
    // backup幅: 必要幅 + border + 右端1スペース余白
    let backup_w = (backup_need + BORDER_PAD + 1).max(BORDER_PAD + 4);
    let remaining = total_w.saturating_sub(backup_w);

    let w_min = watch_min.min(remaining);
    let a_min = archive_min.min(remaining);

    if w_min + a_min <= remaining {
        let extra   = remaining - w_min - a_min;
        let bonus_w = extra / 2;
        let bonus_a = extra - bonus_w;
        return (w_min + bonus_w, backup_w, a_min + bonus_a);
    }

    // 欠落あり: 欠落量均等
    let half = remaining / 2;
    (half, backup_w, remaining - half)
}

/// ペインのコンテンツに必要な最低文字幅を返す（border除く）
fn required_width(names: &[String], has_age: bool) -> u16 {
    let age_prefix = if has_age { AGE_COL + ITEM_PAD } else { 0 };
    let max_name = names.iter()
        .map(|n| n.chars().count() as u16)
        .max()
        .unwrap_or(0);
    age_prefix + max_name
}
