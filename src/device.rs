pub mod switch;

use serialport::{SerialPort, SerialPortType, UsbPortInfo};

/// コンピューターに接続されて利用可能なシリアルポートデバイスの情報
pub struct AvailableDeviceInfo {
    /// ポート名
    pub port_name: String,
    /// 取得できたポート情報
    pub usb_port_info: UsbPortInfo,
    /// ポート情報から生成されたデバイスID
    pub device_id: String,
}

/// 接続可能なUSB Port一覧を取得する
/// # Example
/// ```
/// let device = ardeck::device::available_list();
/// ```
pub fn available_list() -> Vec<AvailableDeviceInfo> {
    serialport::available_ports()
        .unwrap_or(Vec::new())
        .into_iter()
        .filter_map(|port| match &port.port_type {
            SerialPortType::UsbPort(e) => Some(AvailableDeviceInfo {
                port_name: port.port_name.clone(),
                usb_port_info: e.clone(),
                device_id: "TODO".into(), // TODO: ID生成
            }),
            _ => None,
        })
        .collect()
}

/// デバイス一覧の実装
pub trait AvailableDeviceInfoList {
    fn arduino_only(self) -> Vec<AvailableDeviceInfo>;
}

impl AvailableDeviceInfoList for Vec<AvailableDeviceInfo> {
    /// デバイス一覧のうち、arduinoのベンダーコードを持つデバイスだけを抽出する
    /// # Example
    /// ```
    /// let device = ardeck::device::available_list().arduino_only();
    /// ```
    fn arduino_only(self) -> Vec<AvailableDeviceInfo> {
        self.into_iter()
            .filter(|port| {
                // 9025: Arduino LA のベンダーID
                if port.usb_port_info.vid == 9025 {
                    true
                } else {
                    false
                }
            })
            .collect()
    }
}

#[derive(Debug)]
enum ArdeckConnectionErrorKind {
    AlreadyConnected,
    NotConnected,
    SerialPort(serialport::Error),
}

/// コネクションのハンドラー
#[derive(Debug)]
enum ArdeckConnectionHandlerType {
    /// 接続済み
    Connected,
    /// 切断済み
    Disconnected,
    /// 通信中にエラーが発生
    Error,
}

pub type ArdeckConnectionHandler = Box<dyn Fn(ArdeckConnectionHandlerType) + Send + Sync + 'static>;

/// Ardeckとの通信を制御したり、データを処理したりする
struct ArdeckConnection {
    device_id: String,
    /// シリアルポートの接続
    serialport: Option<Box<dyn SerialPort>>,
    handler: Option<ArdeckConnectionHandler>,
}

// DRAFT:
// - コネクションインスタンスが生成されると接続先を記録したインスタンスが生成される
// - インスタンスが存在する間はシリアルポートが切断されても再接続を試みる
// - 初回接続時に未接続ならばリトライ・アクセス拒否ならば初期化失敗としてインスタンスを生成しない
impl ArdeckConnection {
    /// 接続
    pub fn new(
        port_name: String,
        baud_rate: u32,
        device_id: String,
        handler: Option<ArdeckConnectionHandler>,
    ) -> Self {
        match serialport::new(port_name, baud_rate).open() {
            Ok(p) => Self {
                device_id,
                handler,
                serialport: Some(p),
            },
            Err(e) => device,
        };
    }
}
