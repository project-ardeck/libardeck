use serde::{Deserialize, Serialize};

/// Arduinoに接続されているスイッチの種類を示す列挙型
#[derive(Clone, Copy, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum SwitchKind {
    /// デジタルスイッチ ex: タクトスイッチ, トグルスイッチ
    Digital = 0,
    /// アナログスイッチ ex: ポテンションメーター, アナログジョイスティック
    Analog = 1,
}

/// デバイスによって押されたスイッチの情報を保持する構造体
#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SwitchInfo {
    /// スイッチの種類
    kind: SwitchKind,
    /// スイッチが接続されているArduino上のピン番号
    pin: u8,
    /// スイッチの状態を表す数値
    state: u16,
    /// データが取得された時刻(正確には)
    timestamp: i64,
}
