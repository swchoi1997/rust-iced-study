// iced 크레이트에서 Sandbox와 Settings를 가져옵니다.
// Sandbox: 간단한 GUI 애플리케이션을 만들기 위한 트레이트
// Settings: 애플리케이션의 설정을 관리하는 구조체
use iced::{
    alignment::{Horizontal, Vertical},
    font::Family,
    widget::{
        button, column, row, text, text_input,
        text_input::{Icon, Side},
        TextInput,
    },
    Font, Sandbox, Settings
};

// 메인 함수: 프로그램의 진입점
// iced::Result를 반환하여 애플리케이션 실행 중 발생할 수 있는 오류를 처리합니다.
fn main() -> iced::Result {
    // MyApp을 기본 설정으로 실행합니다.
    MyApp::run(Settings::default())
}

#[derive(Debug, Clone)]
enum MyAppMessage{
    Update3(String),
    Update4(String),
    Update5(String),
    Paste5(String),
    Update6(String),
    Submit6,
    Update7(String),
    Update11(String),
}

// MyApp 구조체 정의: 애플리케이션의 상태를 나타냅니다.
#[derive(Default)]
struct MyApp{
    text3: String,
    text4: String,
    text5: String,
    info5: String,
    text6: String,
    info6: String,
    text7: String,
    text11: String,
}

// MyApp에 대한 Sandbox 트레이트 구현
impl Sandbox for MyApp {
    // Message 타입 정의: 애플리케이션 내의 이벤트를 나타냅니다.
    // 현재는 아무 메시지도 없으므로 빈 튜플 ()을 사용합니다.
    type Message = MyAppMessage;

    // new 함수: 애플리케이션의 초기 상태를 생성합니다.
    fn new() -> Self {
        Self::default()
    }

    // title 함수: 애플리케이션 창의 제목을 반환합니다.
    fn title(&self) -> String {
        String::from("Bookmark App")
    }

    // update 함수: 메시지를 받아 애플리케이션의 상태를 업데이트합니다.
    // 현재는 아무 동작도 하지 않습니다.
    fn update(&mut self, _message: Self::Message) {
        match _message {
            MyAppMessage::Update3(s) => self.text3 = s,
            MyAppMessage::Update4(s) => self.text4 = s,
            MyAppMessage::Update5(s) => {
                self.text5 = s;
                self.info5 = "".into();
            },
            MyAppMessage::Paste5(s) => {
                self.text5 = s;
                self.info5 = "Pasted".into();
            },
            MyAppMessage::Update6(s) => {
                self.text6 = s;
                self.info6 = "".into();
            },
            MyAppMessage::Submit6 => self.info6 = "Submitted".into(),
            MyAppMessage::Update7(s) => self.text7 = s,
            MyAppMessage::Update11(s) => self.text11 = s,
        }
    }

    // view 함수: 현재 애플리케이션 상태를 기반으로 UI를 생성합니다.
    // 현재는 단순히 "Hello World" 텍스트를 표시합니다.
    fn view(&self) -> iced::Element<Self::Message> {
        column![
            text_input("Construct from function", ""),
            TextInput::new("Construct from struct", ""),
            text_input("Emable text input", self.text3.as_str())
                .on_input(|s| MyAppMessage::Update3(s)),
            text_input("Shorter on input", self.text4.as_str())
                .on_input(MyAppMessage::Update4),
            text_input("Press Ctrl/Cmd + v", self.text5.as_str())
                .on_input(MyAppMessage::Update5)
                .on_paste(MyAppMessage::Paste5),
            text(self.info5.as_str()),
            text_input("Press enter", self.text6.as_str())
                .on_input(MyAppMessage::Update6)
                .on_submit(MyAppMessage::Submit6),
            text(self.info6.as_str()),
            text_input("Password", self.text7.as_str())
                .secure(true)
                .on_input(MyAppMessage::Update7),
            text_input("Different font", "")
                .font(Font {
                    family: Family::Fantasy,
                    ..Font::DEFAULT
                }),
            text_input("Larger text", "").size(24),
            text_input("With padding", "").padding(20),
            text_input("Icon", self.text11.as_str())
                .icon(Icon{
                    font: Font::DEFAULT,
                    code_point: '\u{2705}',
                    size: None,
                    spacing: 10 as f32,
                    side: Side::Left,
                }).on_input(MyAppMessage::Update11),
        ]
        .into()
    }
}
