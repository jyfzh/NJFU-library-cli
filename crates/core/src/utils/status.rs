use colored::*;

pub fn get_color_str_from_resv_status(resv_status: u32) -> ColoredString {
    match resv_status {
        1169 => "已违约".color(Color::BrightRed).bold(),
        1233 => "已违约".color(Color::BrightRed).bold(),
        5265 => "已违约".color(Color::BrightRed).bold(),
        1027 => "未开始".color(Color::Green).bold().italic(),
        1029 => "已开始".color(Color::Yellow).bold().blink(),
        1093 => "已签到".color(Color::BrightBlue).bold().italic(),
        1217 => "已结束".color(Color::White).dimmed().strikethrough(),
        5313 => "已结束".color(Color::White).dimmed().strikethrough(),
        _ => resv_status.to_string().color("red").reversed(),
    }
}
