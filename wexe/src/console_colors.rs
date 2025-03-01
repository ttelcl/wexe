// String constants for console colors and styling.
// See https://en.wikipedia.org/wiki/ANSI_escape_code#Select_Graphic_Rendition_parameters

#![allow(dead_code)]
#![allow(non_upper_case_globals)]

/// Reset all console styling (foreground, background, effects).
pub const rst: &str = stl::reset;

/// Bold
pub const stl_b: &str = stl::bold;

/// Dim
pub const stl_d: &str = stl::dim;

/// Italic
pub const stl_i: &str = stl::italic;

/// Underline
pub const stl_u: &str = stl::underline;

/// strikethrough
pub const stl_s: &str = stl::strike;

/// Cancels dim or bold
pub const stl_n: &str = stl::normal_itensity;

/// Foreground black
pub const fg_K: &str = fg::black;

/// Foreground dark red
pub const fg_R: &str = fg::dark_red;

/// Foreground dark green
pub const fg_G: &str = fg::dark_green;

/// Foreground dark yellow (orange-ish)
pub const fg_Y: &str = fg::dark_yellow;

/// Foreground dark blue
pub const fg_B: &str = fg::dark_blue;

/// Foreground dark magenta
pub const fg_M: &str = fg::dark_magenta;

/// Foreground dark cyan
pub const fg_C: &str = fg::dark_cyan;

/// Foreground light gray ("dark white")
pub const fg_W: &str = fg::light_gray;

/// Foreground dark gray ("light black")
pub const fg_k: &str = fg::dark_gray;

/// Foreground light red
pub const fg_r: &str = fg::red;

/// Foreground light green
pub const fg_g: &str = fg::green;

/// Foreground light yellow
pub const fg_y: &str = fg::yellow;

/// Foreground light blue
pub const fg_b: &str = fg::blue;

/// Foreground light magenta
pub const fg_m: &str = fg::magenta;

/// Foreground light cyan
pub const fg_c: &str = fg::cyan;

/// Foreground white
pub const fg_w: &str = fg::white;

/// Foreground orange-ish ("dark yellow")
pub const fg_O: &str = fg::dark_yellow;

/// Foreground orange-ish ("dark yellow")
pub const fg_o: &str = fg::dark_yellow;

/// Background black
pub const bg_K: &str = bg::black;

/// Background dark red
pub const bg_R: &str = bg::dark_red;

/// Background dark green
pub const bg_G: &str = bg::dark_green;

/// Background dark yellow (orange-ish)
pub const bg_Y: &str = bg::dark_yellow;

/// Background dark blue
pub const bg_B: &str = bg::dark_blue;

/// Background dark magenta
pub const bg_M: &str = bg::dark_magenta;

/// Background dark cyan
pub const bg_C: &str = bg::dark_cyan;

/// Background light gray ("dark white")
pub const bg_W: &str = bg::light_gray;

/// Background dark gray ("light black")
pub const bg_k: &str = bg::dark_gray;

/// Background light red
pub const bg_r: &str = bg::red;

/// Background light green
pub const bg_g: &str = bg::green;

/// Background light yellow
pub const bg_y: &str = bg::yellow;

/// Background light blue
pub const bg_b: &str = bg::blue;

/// Background light magenta
pub const bg_m: &str = bg::magenta;

/// Background light cyan
pub const bg_c: &str = bg::cyan;

/// Background white
pub const bg_w: &str = bg::white;

/// Background orange-ish ("dark yellow")
pub const bg_O: &str = bg::dark_yellow;

/// Background orange-ish ("dark yellow")
pub const bg_o: &str = bg::dark_yellow;

/// Console effects other than colors
pub mod stl {
    pub const reset: &str = "\x1B[0m";

    pub const bold: &str = "\x1B[1m";
    pub const dim: &str = "\x1B[2m";
    pub const italic: &str = "\x1B[3m";
    pub const underline: &str = "\x1B[4m";
    pub const reverse: &str = "\x1B[7m";
    pub const strike: &str = "\x1B[9m";
    pub const normal_itensity: &str = "\x1B[22m";
}

/// Foreground colors
pub mod fg {
    pub const black: &str = "\x1B[30m";
    pub const dark_red: &str = "\x1B[31m";
    pub const dark_green: &str = "\x1B[32m";
    pub const dark_yellow: &str = "\x1B[33m";
    pub const dark_blue: &str = "\x1B[34m";
    pub const dark_magenta: &str = "\x1B[35m";
    pub const dark_cyan: &str = "\x1B[36m";
    pub const light_gray: &str = "\x1B[37m";

    pub const dark_gray: &str = "\x1B[90m";
    pub const red: &str = "\x1B[91m";
    pub const green: &str = "\x1B[92m";
    pub const yellow: &str = "\x1B[93m";
    pub const blue: &str = "\x1B[94m";
    pub const magenta: &str = "\x1B[95m";
    pub const cyan: &str = "\x1B[96m";
    pub const white: &str = "\x1B[97m";
}

/// Background colors
pub mod bg {

    pub const black: &str = "\x1B[40m";
    pub const dark_red: &str = "\x1B[41m";
    pub const dark_green: &str = "\x1B[42m";
    pub const dark_yellow: &str = "\x1B[43m";
    pub const dark_blue: &str = "\x1B[44m";
    pub const dark_magenta: &str = "\x1B[45m";
    pub const dark_cyan: &str = "\x1B[46m";
    pub const light_gray: &str = "\x1B[47m";

    pub const dark_gray: &str = "\x1B[100m";
    pub const red: &str = "\x1B[101m";
    pub const green: &str = "\x1B[102m";
    pub const yellow: &str = "\x1B[103m";
    pub const blue: &str = "\x1B[104m";
    pub const magenta: &str = "\x1B[105m";
    pub const cyan: &str = "\x1B[106m";
    pub const white: &str = "\x1B[107m";

    // Convenience aliases
    pub const K: &str = black;
    pub const R: &str = dark_red;
    pub const G: &str = dark_green;
    pub const Y: &str = dark_yellow;
    pub const B: &str = dark_blue;
    pub const M: &str = dark_magenta;
    pub const C: &str = dark_cyan;
    pub const W: &str = light_gray;

    pub const k: &str = dark_gray;
    pub const r: &str = red;
    pub const g: &str = green;
    pub const y: &str = yellow;
    pub const b: &str = blue;
    pub const m: &str = magenta;
    pub const c: &str = cyan;
    pub const w: &str = white;

    pub const orange: &str = dark_yellow;
    pub const O: &str = orange;
    pub const o: &str = orange;
}
