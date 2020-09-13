use indoc::indoc;
use ansi_term;
use ansi_term::Color::{self, Fixed, RGB};
use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSetBuilder;
use syntect::highlighting::{self, ThemeSet};
use syntect::util::LinesWithEndings;

// copied from https://github.com/sharkdp/bat/blob/3a85fd767bd1f03debd0a60ac5bc08548f95bc9d/src/terminal.rs
// with slight modifications
fn to_ansi_color(color: highlighting::Color) -> ansi_term::Color {
    if color.a == 0 {
        // Themes can specify one of the user-configurable terminal colors by
        // encoding them as #RRGGBBAA with AA set to 00 (transparent) and RR set
        // to the 8-bit color palette number. The built-in themes ansi-light,
        // ansi-dark, base16, and base16-256 use this.
        match color.r {
            // For the first 8 colors, use the Color enum to produce ANSI escape
            // sequences using codes 30-37 (foreground) and 40-47 (background).
            // For example, red foreground is \x1b[31m. This works on terminals
            // without 256-color support.
            0x00 => Color::Black,
            0x01 => Color::Red,
            0x02 => Color::Green,
            0x03 => Color::Yellow,
            0x04 => Color::Blue,
            0x05 => Color::Purple,
            0x06 => Color::Cyan,
            0x07 => Color::White,
            // For all other colors, use Fixed to produce escape sequences using
            // codes 38;5 (foreground) and 48;5 (background). For example,
            // bright red foreground is \x1b[38;5;9m. This only works on
            // terminals with 256-color support.
            //
            // TODO: When ansi_term adds support for bright variants using codes
            // 90-97 (foreground) and 100-107 (background), we should use those
            // for values 0x08 to 0x0f and only use Fixed for 0x10 to 0xff.
            n => Fixed(n),
        }
    } else {
        RGB(color.r, color.g, color.b)
    }
}

fn main() {
    // Load these once at the start of your program
    let ts = ThemeSet::load_from_folder("assets").unwrap();
    let mut ps = SyntaxSetBuilder::new();
    ps.add_from_folder("assets", true).unwrap();
    let ps = ps.build();

    let syntax = ps.find_syntax_by_name("HTTP").unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["ansi-dark"]);
    let s = indoc! {r#"
        HTTP/1.1 200 OK
        Date: Sun, 13 Sep 2020 16:27:14 GMT
        Content-Type: application/json
        Content-Length: 192
        Server: gunicorn/19.9.0
        Access-Control-Allow-Origin: *
        Access-Control-Allow-Credentials: true

        {
            "headers": {
                "Accept-Encoding": "gzip, deflate", 
                "Cache-Control": "no-cache", 
                "Host": "httpbin.org", 
                "X-Amzn-Trace-Id": "Root=1-5f5e4862-f10c4bb46528ea0c8ee4b16e"
            }
        }
    "#};
    for line in LinesWithEndings::from(s) {
        let highlights = h.highlight(line, &ps);
        for (style, component) in highlights {
            let color = to_ansi_color(style.foreground);
            print!("{}", &color.paint(component));
        }
    }
}