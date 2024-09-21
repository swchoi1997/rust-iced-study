// iced 크레이트에서 Sandbox와 Settings를 가져옵니다.
// Sandbox: 간단한 GUI 애플리케이션을 만들기 위한 트레이트
// Settings: 애플리케이션의 설정을 관리하는 구조체
use iced::{
    widget::{button, column, row, text},
    Sandbox, Settings
};

// 메인 함수: 프로그램의 진입점
// iced::Result를 반환하여 애플리케이션 실행 중 발생할 수 있는 오류를 처리합니다.
fn main() -> iced::Result {
    // MyApp을 기본 설정으로 실행합니다.
    MyApp::run(Settings::default())
}

#[derive(Debug, Clone)]
enum MyAppMessage{
    PlusButtonPressed,
    MinusButtonPressed
}

// MyApp 구조체 정의: 애플리케이션의 상태를 나타냅니다.
struct MyApp{
    counter: usize
}

// MyApp에 대한 Sandbox 트레이트 구현
impl Sandbox for MyApp {
    // Message 타입 정의: 애플리케이션 내의 이벤트를 나타냅니다.
    // 현재는 아무 메시지도 없으므로 빈 튜플 ()을 사용합니다.
    type Message = MyAppMessage;

    // new 함수: 애플리케이션의 초기 상태를 생성합니다.
    fn new() -> Self {
        Self{ counter: 0 }
    }

    // title 함수: 애플리케이션 창의 제목을 반환합니다.
    fn title(&self) -> String {
        String::from("Bookmark App")
    }

    // update 함수: 메시지를 받아 애플리케이션의 상태를 업데이트합니다.
    // 현재는 아무 동작도 하지 않습니다.
    fn update(&mut self, _message: Self::Message) {
        match _message {
            MyAppMessage::PlusButtonPressed => {
                if self.counter == std::usize::MAX {
                    self.counter = 0;
                    return
                }
                self.counter += 1
            },
            MyAppMessage::MinusButtonPressed => {
                if self.counter == 0{
                    self.counter = std::usize::MAX;
                    return
                }
                self.counter -= 1
            },
        }
    }

    // view 함수: 현재 애플리케이션 상태를 기반으로 UI를 생성합니다.
    // 현재는 단순히 "Hello World" 텍스트를 표시합니다.
    fn view(&self) -> iced::Element<Self::Message> {
        column![
            text("+ or -"),
            text(self.counter),
            row![
                button("+").on_press(MyAppMessage::PlusButtonPressed),
                button("-").on_press(MyAppMessage::MinusButtonPressed),
            ]
        ].into()
    }
}
