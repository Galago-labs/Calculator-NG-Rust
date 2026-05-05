#![windows_subsystem = "windows"]

use std::ffi::OsStr;
use std::iter::once;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use std::ptr;

type HWND      = *mut std::ffi::c_void;
type HINSTANCE = *mut std::ffi::c_void;
type HMENU     = *mut std::ffi::c_void;
type HDC       = *mut std::ffi::c_void;
type HBRUSH    = *mut std::ffi::c_void;
type HFONT     = *mut std::ffi::c_void;
type HGDIOBJ   = *mut std::ffi::c_void;
type WPARAM    = usize;
type LPARAM    = isize;
type LRESULT   = isize;
type BOOL      = i32;
type DWORD     = u32;
type UINT      = u32;
type LONG      = i32;
type ATOM      = u16;
type COLORREF  = u32;
type HPEN      = *mut std::ffi::c_void;

const WS_OVERLAPPED:   DWORD = 0x00000000;
const WS_CAPTION:      DWORD = 0x00C00000;
const WS_SYSMENU:      DWORD = 0x00080000;
const WS_MINIMIZEBOX:  DWORD = 0x00020000;
const WS_CLIPCHILDREN: DWORD = 0x02000000;

const WM_DESTROY:    UINT = 0x0002;
const WM_PAINT:      UINT = 0x000F;
const WM_ERASEBKGND: UINT = 0x0014;
const WM_LBUTTONDOWN:UINT = 0x0201;
const WM_LBUTTONUP:  UINT = 0x0202;
const WM_MOUSEMOVE:  UINT = 0x0200;
const WM_MOUSELEAVE: UINT = 0x02A3;
const WM_KEYDOWN:    UINT = 0x0100;
const WM_CHAR:       UINT = 0x0102;
const WM_TIMER:      UINT = 0x0113;
const WM_CREATE:     UINT = 0x0001;

const IDC_ARROW: usize = 32512;
const PS_SOLID:  i32   = 0;
const TRANSPARENT: i32 = 1;
const NULL_PEN:  i32   = 8;

const DT_CENTER:      UINT = 0x00000001;
const DT_VCENTER:     UINT = 0x00000004;
const DT_SINGLELINE:  UINT = 0x00000020;
const DT_RIGHT:       UINT = 0x00000002;
const DT_END_ELLIPSIS:UINT = 0x00008000;

const FW_NORMAL: i32 = 400;
const FW_BOLD:   i32 = 700;
const ANSI_CHARSET:        u8 = 0;
const OUT_TT_PRECIS:       u8 = 4;
const CLIP_DEFAULT_PRECIS: u8 = 0;
const CLEARTYPE_QUALITY:   u8 = 5;
const FF_SWISS:    u8 = 0x20;
const VARIABLE_PITCH: u8 = 2;

const SW_SHOW:  i32 = 5;
const CS_HREDRAW: DWORD = 0x0002;
const CS_VREDRAW: DWORD = 0x0001;
const GWLP_USERDATA: i32 = -21;

const VK_RETURN:  usize = 0x0D;
const VK_ESCAPE:  usize = 0x1B;
const VK_BACK:    usize = 0x08;
const VK_CONTROL: usize = 0x11;
const VK_C:       usize = 0x43;
const VK_V:       usize = 0x56;

const CF_UNICODETEXT: UINT = 13;
const GMEM_MOVEABLE:  UINT = 0x0002;
const SRCCOPY:        DWORD = 0x00CC0020;
const TME_LEAVE:      DWORD = 0x00000002;

// ── Colors (Windows BGR) ──────────────────────────────────────────
const BG_COLOR:     COLORREF = 0x001E1E1E;
const PANEL_COLOR:  COLORREF = 0x00262526;
const BTN_COLOR:    COLORREF = 0x002D2D2D;
const BTN_HOVER:    COLORREF = 0x003A3A3A;
const BTN_PRESS:    COLORREF = 0x00484848;
const ACCENT_COLOR: COLORREF = 0x00DE8D3A; // blue in BGR = #3a8dde
const ACCENT_HOVER: COLORREF = 0x00EF9D4A;
const ACCENT_PRESS: COLORREF = 0x00C07828;
const OP_TEXT:      COLORREF = 0x0044AAFF; // operator label colour
const TEXT_WHITE:   COLORREF = 0x00FFFFFF;
const TEXT_GRAY:    COLORREF = 0x00AAAAAA;
const CLEAR_COLOR:  COLORREF = 0x003A3A2A;
const CLEAR_HOVER:  COLORREF = 0x004A4A3A;
const CLEAR_PRESS:  COLORREF = 0x00585848;
const ERROR_COLOR:  COLORREF = 0x004444EE;
const SEPARATOR:    COLORREF = 0x003A3A3A;

// ── Win32 structs ─────────────────────────────────────────────────
#[repr(C)] struct WNDCLASSEXW {
    cb_size:         UINT,
    style:           UINT,
    lpfn_wnd_proc:   Option<unsafe extern "system" fn(HWND,UINT,WPARAM,LPARAM)->LRESULT>,
    cb_cls_extra:    i32,
    cb_wnd_extra:    i32,
    h_instance:      HINSTANCE,
    h_icon:          *mut std::ffi::c_void,
    h_cursor:        *mut std::ffi::c_void,
    hbr_background:  HBRUSH,
    lpsz_menu_name:  *const u16,
    lpsz_class_name: *const u16,
    h_icon_sm:       *mut std::ffi::c_void,
}
#[repr(C)] struct MSG {
    hwnd: HWND, message: UINT, w_param: WPARAM, l_param: LPARAM,
    time: DWORD, pt_x: LONG, pt_y: LONG,
}
#[repr(C)] struct PAINTSTRUCT {
    hdc: HDC, f_erase: BOOL, rc_paint: RECT,
    f_restore: BOOL, f_inc_update: BOOL, rgb_reserved: [u8;32],
}
#[repr(C)] #[derive(Clone,Copy,Default)] struct RECT {
    left: LONG, top: LONG, right: LONG, bottom: LONG,
}
#[repr(C)] struct POINT { x: LONG, y: LONG }
#[repr(C)] struct SIZE  { cx: LONG, cy: LONG }
#[repr(C)] struct TRACKMOUSEEVENT {
    cb_size: DWORD, dw_flags: DWORD, hwnd_track: HWND, dw_hover_time: DWORD,
}

// ── Win32 imports ─────────────────────────────────────────────────
#[link(name="user32")] extern "system" {
    fn RegisterClassExW(lpwcx: *const WNDCLASSEXW) -> ATOM;
    fn CreateWindowExW(dw_ex_style:DWORD,lp_class_name:*const u16,lp_window_name:*const u16,
        dw_style:DWORD,x:i32,y:i32,n_width:i32,n_height:i32,
        h_wnd_parent:HWND,h_menu:HMENU,h_instance:HINSTANCE,lp_param:*mut std::ffi::c_void)->HWND;
    fn ShowWindow(h_wnd:HWND,n_cmd_show:i32)->BOOL;
    fn UpdateWindow(h_wnd:HWND)->BOOL;
    fn GetMessageW(lp_msg:*mut MSG,h_wnd:HWND,w_msg_filter_min:UINT,w_msg_filter_max:UINT)->BOOL;
    fn TranslateMessage(lp_msg:*const MSG)->BOOL;
    fn DispatchMessageW(lp_msg:*const MSG)->LRESULT;
    fn DefWindowProcW(h_wnd:HWND,msg:UINT,w_param:WPARAM,l_param:LPARAM)->LRESULT;
    fn PostQuitMessage(n_exit_code:i32);
    fn BeginPaint(h_wnd:HWND,lp_paint:*mut PAINTSTRUCT)->HDC;
    fn EndPaint(h_wnd:HWND,lp_paint:*const PAINTSTRUCT)->BOOL;
    fn InvalidateRect(h_wnd:HWND,lp_rect:*const RECT,b_erase:BOOL)->BOOL;
    fn GetClientRect(h_wnd:HWND,lp_rect:*mut RECT)->BOOL;
    fn GetSystemMetrics(n_index:i32)->i32;
    fn LoadCursorW(h_instance:HINSTANCE,lp_cursor_name:*const u16)->*mut std::ffi::c_void;
    fn SetFocus(h_wnd:HWND)->HWND;
    fn OpenClipboard(h_wnd_new_owner:HWND)->BOOL;
    fn EmptyClipboard()->BOOL;
    fn SetClipboardData(u_format:UINT,h_mem:*mut std::ffi::c_void)->*mut std::ffi::c_void;
    fn GetClipboardData(u_format:UINT)->*mut std::ffi::c_void;
    fn CloseClipboard()->BOOL;
    fn IsClipboardFormatAvailable(format:UINT)->BOOL;
    fn GetWindowLongPtrW(h_wnd:HWND,n_index:i32)->isize;
    fn SetWindowLongPtrW(h_wnd:HWND,n_index:i32,dw_new_long:isize)->isize;
    fn TrackMouseEvent(lp_event_track:*mut TRACKMOUSEEVENT)->BOOL;
    fn SetTimer(h_wnd:HWND,n_id_event:usize,u_elapse:UINT,lp_timer_func:*mut std::ffi::c_void)->usize;
    fn KillTimer(h_wnd:HWND,u_id_event:usize)->BOOL;
    fn GetKeyState(n_virt_key:i32)->i16;
}
#[link(name="gdi32")] extern "system" {
    fn CreateSolidBrush(color:COLORREF)->HBRUSH;
    fn CreatePen(fn_pen_style:i32,n_width:i32,cr_color:COLORREF)->HPEN;
    fn SelectObject(hdc:HDC,h:HGDIOBJ)->HGDIOBJ;
    fn DeleteObject(h:HGDIOBJ)->BOOL;
    fn SetTextColor(hdc:HDC,color:COLORREF)->COLORREF;
    fn SetBkMode(hdc:HDC,mode:i32)->i32;
    fn GetStockObject(i:i32)->HGDIOBJ;
    fn FillRect(hdc:HDC,lprc:*const RECT,hbr:HBRUSH)->i32;
    fn DrawTextW(hdc:HDC,lp_str:*const u16,n_count:i32,lp_rect:*mut RECT,u_format:UINT)->i32;
    fn CreateFontW(h:i32,w:i32,e:i32,o:i32,weight:i32,italic:DWORD,underline:DWORD,
        strikeout:DWORD,charset:DWORD,out_prec:DWORD,clip_prec:DWORD,quality:DWORD,
        pitch_family:DWORD,face:*const u16)->HFONT;
    fn RoundRect(hdc:HDC,left:i32,top:i32,right:i32,bottom:i32,w:i32,h:i32)->BOOL;
    fn MoveToEx(hdc:HDC,x:i32,y:i32,lp_point:*mut POINT)->BOOL;
    fn LineTo(hdc:HDC,x:i32,y:i32)->BOOL;
    fn CreateCompatibleDC(hdc:HDC)->HDC;
    fn CreateCompatibleBitmap(hdc:HDC,cx:i32,cy:i32)->*mut std::ffi::c_void;
    fn BitBlt(hdc_dest:HDC,x:i32,y:i32,cx:i32,cy:i32,hdc_src:HDC,x1:i32,y1:i32,rop:DWORD)->BOOL;
    fn DeleteDC(hdc:HDC)->BOOL;
}
#[link(name="kernel32")] extern "system" {
    fn GetModuleHandleW(lp_module_name:*const u16)->HINSTANCE;
    fn GlobalAlloc(u_flags:UINT,dw_bytes:usize)->*mut std::ffi::c_void;
    fn GlobalLock(h_mem:*mut std::ffi::c_void)->*mut std::ffi::c_void;
    fn GlobalUnlock(h_mem:*mut std::ffi::c_void)->BOOL;
    fn lstrlenW(lp_string:*const u16)->i32;
}

// ── Helpers ───────────────────────────────────────────────────────
fn to_wstring(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(once(0)).collect()
}
unsafe fn wstring_to_string(ptr: *const u16) -> String {
    if ptr.is_null() { return String::new(); }
    let len = lstrlenW(ptr) as usize;
    String::from_utf16_lossy(std::slice::from_raw_parts(ptr, len))
}

// ── Calculator logic ──────────────────────────────────────────────
#[derive(Clone, PartialEq)]
enum Token { Number(f64), Plus, Minus, Mul, Div, LParen, RParen }

fn tokenize(expr: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = expr.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            ' ' => { i += 1; }
            '+' => { tokens.push(Token::Plus);   i += 1; }
            // Accept both ascii minus and unicode minus
            '-' | '\u{2212}' => { tokens.push(Token::Minus); i += 1; }
            '×' | '*' => { tokens.push(Token::Mul); i += 1; }
            '÷' | '/' => { tokens.push(Token::Div); i += 1; }
            '(' => { tokens.push(Token::LParen); i += 1; }
            ')' => { tokens.push(Token::RParen); i += 1; }
            '0'..='9' | '.' => {
                let start = i;
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') { i += 1; }
                let s: String = chars[start..i].iter().collect();
                tokens.push(Token::Number(s.parse().map_err(|_| format!("Bad number: {s}"))?));
            }
            c => return Err(format!("Unknown char: {c}")),
        }
    }
    Ok(tokens)
}

struct Parser { tokens: Vec<Token>, pos: usize }
impl Parser {
    fn new(t: Vec<Token>) -> Self { Parser { tokens: t, pos: 0 } }
    fn peek(&self) -> Option<&Token> { self.tokens.get(self.pos) }
    fn consume(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() { let t = self.tokens[self.pos].clone(); self.pos += 1; Some(t) } else { None }
    }
    fn parse_expr(&mut self) -> Result<f64,String> { self.parse_add() }
    fn parse_add(&mut self) -> Result<f64,String> {
        let mut l = self.parse_mul()?;
        loop { match self.peek() {
            Some(Token::Plus)  => { self.consume(); l += self.parse_mul()?; }
            Some(Token::Minus) => { self.consume(); l -= self.parse_mul()?; }
            _ => break,
        }} Ok(l)
    }
    fn parse_mul(&mut self) -> Result<f64,String> {
        let mut l = self.parse_unary()?;
        loop { match self.peek() {
            Some(Token::Mul) => { self.consume(); l *= self.parse_unary()?; }
            Some(Token::Div) => {
                self.consume();
                let r = self.parse_unary()?;
                if r == 0.0 { return Err("Division by zero".into()); }
                l /= r;
            }
            _ => break,
        }} Ok(l)
    }
    fn parse_unary(&mut self) -> Result<f64,String> {
        match self.peek() {
            Some(Token::Minus) => { self.consume(); Ok(-self.parse_primary()?) }
            Some(Token::Plus)  => { self.consume(); self.parse_primary() }
            _ => self.parse_primary(),
        }
    }
    fn parse_primary(&mut self) -> Result<f64,String> {
        match self.peek() {
            Some(Token::Number(_)) => {
                if let Some(Token::Number(n)) = self.consume() { Ok(n) }
                else { Err("Expected number".into()) }
            }
            Some(Token::LParen) => {
                self.consume();
                let v = self.parse_expr()?;
                if self.consume() != Some(Token::RParen) { return Err("Expected ')'".into()); }
                Ok(v)
            }
            _ => Err("Unexpected token".into()),
        }
    }
}

fn evaluate(expr: &str) -> Result<f64,String> {
    let expr = expr.trim();
    if expr.is_empty() { return Err("Empty".into()); }
    // Normalise operators before tokenizing
    let expr = expr.replace('\u{2212}', "-").replace('×', "*").replace('÷', "/");
    let tokens = tokenize(&expr)?;
    if tokens.is_empty() { return Err("Empty expression".into()); }
    let mut p = Parser::new(tokens);
    let r = p.parse_expr()?;
    if p.pos != p.tokens.len() { return Err("Unexpected tokens".into()); }
    if !r.is_finite() { return Err("Overflow".into()); }
    Ok(r)
}

fn format_result(n: f64) -> String {
    if n.is_nan()      { return "Error".into(); }
    if n.is_infinite() { return if n > 0.0 { "∞".into() } else { "-∞".into() }; }
    if n.fract() == 0.0 && n.abs() < 1e15 { return format!("{}", n as i64); }
    let s = format!("{:.10}", n);
    s.trim_end_matches('0').trim_end_matches('.').to_string()
}

// ── Button definitions ────────────────────────────────────────────
#[derive(Clone, PartialEq)]
enum BtnKind { Digit, Op, Equals, Clear, Backspace, Dot, Negate, Paren }

struct Btn { label: &'static str, kind: BtnKind, row: i32, col: i32 }

const BUTTONS: &[Btn] = &[
    Btn { label:"C",   kind:BtnKind::Clear,     row:0, col:0 },
    Btn { label:"()",  kind:BtnKind::Paren,     row:0, col:1 },
    Btn { label:"←",   kind:BtnKind::Backspace, row:0, col:2 },
    Btn { label:"÷",   kind:BtnKind::Op,        row:0, col:3 },
    Btn { label:"7",   kind:BtnKind::Digit,     row:1, col:0 },
    Btn { label:"8",   kind:BtnKind::Digit,     row:1, col:1 },
    Btn { label:"9",   kind:BtnKind::Digit,     row:1, col:2 },
    Btn { label:"×",   kind:BtnKind::Op,        row:1, col:3 },
    Btn { label:"4",   kind:BtnKind::Digit,     row:2, col:0 },
    Btn { label:"5",   kind:BtnKind::Digit,     row:2, col:1 },
    Btn { label:"6",   kind:BtnKind::Digit,     row:2, col:2 },
    Btn { label:"−",   kind:BtnKind::Op,        row:2, col:3 },
    Btn { label:"1",   kind:BtnKind::Digit,     row:3, col:0 },
    Btn { label:"2",   kind:BtnKind::Digit,     row:3, col:1 },
    Btn { label:"3",   kind:BtnKind::Digit,     row:3, col:2 },
    Btn { label:"+",   kind:BtnKind::Op,        row:3, col:3 },
    Btn { label:"+/-", kind:BtnKind::Negate,    row:4, col:0 },
    Btn { label:"0",   kind:BtnKind::Digit,     row:4, col:1 },
    Btn { label:".",   kind:BtnKind::Dot,       row:4, col:2 },
    Btn { label:"=",   kind:BtnKind::Equals,    row:4, col:3 },
];

// ── App state ─────────────────────────────────────────────────────
struct AppState {
    expression:       String,
    display_main:     String,
    display_sub:      String,
    just_calculated:  bool,
    error:            bool,
    hovered_btn:      i32,
    pressed_btn:      i32,
    font_main:        HFONT,
    font_sub:         HFONT,
    font_btn:         HFONT,
}

impl AppState {
    fn new() -> Self {
        AppState {
            expression: String::new(),
            display_main: "0".into(),
            display_sub: String::new(),
            just_calculated: false,
            error: false,
            hovered_btn: -1,
            pressed_btn: -1,
            font_main: ptr::null_mut(),
            font_sub:  ptr::null_mut(),
            font_btn:  ptr::null_mut(),
        }
    }

    fn clear(&mut self) {
        self.expression.clear();
        self.display_main = "0".into();
        self.display_sub.clear();
        self.just_calculated = false;
        self.error = false;
    }

    fn backspace(&mut self) {
        if self.just_calculated || self.error { self.clear(); return; }
        if !self.expression.is_empty() {
            let mut chars = self.expression.chars();
            chars.next_back();
            self.expression = chars.as_str().to_string();
            self.display_main = if self.expression.is_empty() { "0".into() } else { self.expression.clone() };
        }
    }

    fn input_char(&mut self, ch: &str) {
        if self.error { self.clear(); }
        if self.just_calculated {
            let is_op = matches!(ch, "+" | "−" | "×" | "÷");
            if is_op {
                self.expression = self.display_main.replace('−', "-").replace('×', "*").replace('÷', "/");
            } else {
                self.expression.clear();
                self.display_sub.clear();
            }
            self.just_calculated = false;
        }
        if self.expression.len() > 64 { return; }

        // Prevent consecutive operators
        let last = self.expression.chars().last();
        let is_op = matches!(ch, "+" | "−" | "×" | "÷");
        let last_is_op = matches!(last, Some('+') | Some('-') | Some('*') | Some('/'));
        if is_op && last_is_op {
            // Replace last operator
            self.expression.pop();
        }

        // Map display chars to internal ascii for expression storage
        let internal = match ch {
            "−" => "-", "×" => "*", "÷" => "/", other => other,
        };
        self.expression.push_str(internal);
        self.display_main = self.expression
            .replace('*', "×").replace('/', "÷").replace('-', "−");
    }

    fn calculate(&mut self) {
        if self.expression.is_empty() { return; }
        let expr_display = self.expression.replace('*', "×").replace('/', "÷").replace('-', "−");
        match evaluate(&self.expression) {
            Ok(result) => {
                let s = format_result(result);
                self.display_sub  = format!("{} =", expr_display);
                self.display_main = s.clone();
                self.expression   = s;
                self.just_calculated = true;
                self.error = false;
            }
            Err(_) => {
                self.display_sub  = expr_display;
                self.display_main = "Error".into();
                self.expression.clear();
                self.error = true;
                self.just_calculated = false;
            }
        }
    }

    fn toggle_paren(&mut self) {
        let open  = self.expression.chars().filter(|&c| c == '(').count();
        let close = self.expression.chars().filter(|&c| c == ')').count();
        if open == close || self.expression.is_empty() {
            self.input_char("(");
        } else {
            self.input_char(")");
        }
    }

    fn negate(&mut self) {
        if self.expression.is_empty() || self.display_main == "0" {
            self.expression   = "-".into();
            self.display_main = "-".into();
            return;
        }
        if self.expression.starts_with('-') && !self.expression[1..].contains('-') {
            self.expression   = self.expression[1..].to_string();
        } else {
            self.expression = format!("-({})", self.expression);
        }
        self.display_main = self.expression.replace('*',"×").replace('/',"÷").replace('-',"−");
    }

    fn handle_button(&mut self, idx: usize) {
        match BUTTONS[idx].kind {
            BtnKind::Digit     => self.input_char(BUTTONS[idx].label),
            BtnKind::Op        => self.input_char(BUTTONS[idx].label),
            BtnKind::Equals    => self.calculate(),
            BtnKind::Clear     => self.clear(),
            BtnKind::Backspace => self.backspace(),
            BtnKind::Negate    => self.negate(),
            BtnKind::Paren     => self.toggle_paren(),
            BtnKind::Dot       => {
                let last_op  = self.expression.rfind(|c:char| "+-*/".contains(c)).unwrap_or(0);
                let last_dot = self.expression.rfind('.').unwrap_or(usize::MAX);
                if last_dot == usize::MAX || last_dot < last_op {
                    self.input_char(".");
                }
            }
        }
    }
}

// ── Layout ────────────────────────────────────────────────────────
const WIN_W:    i32 = 320;
const WIN_H:    i32 = 490;
const DISP_H:   i32 = 130;
const PAD:      i32 = 12;
const BTN_GAP:  i32 = 8;
const COLS:     i32 = 4;
const ROWS:     i32 = 5;

fn btn_rect(idx: usize, cw: i32, ch: i32) -> RECT {
    let b = &BUTTONS[idx];
    let aw = cw - PAD * 2;
    let ah = ch - DISP_H - PAD * 2;
    let bw = (aw - BTN_GAP * (COLS - 1)) / COLS;
    let bh = (ah - BTN_GAP * (ROWS - 1)) / ROWS;
    let x  = PAD + b.col * (bw + BTN_GAP);
    let y  = DISP_H + PAD + b.row * (bh + BTN_GAP);
    RECT { left: x, top: y, right: x + bw, bottom: y + bh }
}

fn hit_test(mx: i32, my: i32, cw: i32, ch: i32) -> i32 {
    for i in 0..BUTTONS.len() {
        let r = btn_rect(i, cw, ch);
        if mx >= r.left && mx < r.right && my >= r.top && my < r.bottom { return i as i32; }
    }
    -1
}

// ── Drawing ───────────────────────────────────────────────────────
unsafe fn draw_rrect(hdc: HDC, r: &RECT, radius: i32, fill: COLORREF) {
    let brush = CreateSolidBrush(fill);
    let pen   = CreatePen(PS_SOLID, 0, fill); // border same as fill
    let ob = SelectObject(hdc, brush as HGDIOBJ);
    let op = SelectObject(hdc, pen   as HGDIOBJ);
    RoundRect(hdc, r.left, r.top, r.right, r.bottom, radius, radius);
    SelectObject(hdc, ob); SelectObject(hdc, op);
    DeleteObject(brush as HGDIOBJ); DeleteObject(pen as HGDIOBJ);
}

unsafe fn draw_text(hdc: HDC, text: &str, mut r: RECT, font: HFONT, color: COLORREF, flags: UINT) {
    let of = SelectObject(hdc, font as HGDIOBJ);
    SetTextColor(hdc, color);
    SetBkMode(hdc, TRANSPARENT);
    let ws = to_wstring(text);
    DrawTextW(hdc, ws.as_ptr(), -1, &mut r, flags);
    SelectObject(hdc, of);
}

unsafe fn paint(hwnd: HWND, state: &AppState) {
    let mut ps: PAINTSTRUCT = mem::zeroed();
    let hdc_win = BeginPaint(hwnd, &mut ps);
    let mut cr: RECT = mem::zeroed();
    GetClientRect(hwnd, &mut cr);
    let (cw, ch) = (cr.right, cr.bottom);

    // Double buffer
    let mem_dc  = CreateCompatibleDC(hdc_win);
    let hbm     = CreateCompatibleBitmap(hdc_win, cw, ch);
    let old_bm  = SelectObject(mem_dc, hbm);

    // Background
    let bg_br = CreateSolidBrush(BG_COLOR);
    FillRect(mem_dc, &cr, bg_br);
    DeleteObject(bg_br as HGDIOBJ);

    // Display panel
    let dp = RECT { left:0, top:0, right:cw, bottom:DISP_H };
    let dp_br = CreateSolidBrush(PANEL_COLOR);
    FillRect(mem_dc, &dp, dp_br);
    DeleteObject(dp_br as HGDIOBJ);

    // Sub-expression
    if !state.display_sub.is_empty() {
        let r = RECT { left:PAD, top:8, right:cw-PAD, bottom:42 };
        draw_text(mem_dc, &state.display_sub, r, state.font_sub, TEXT_GRAY,
            DT_RIGHT | DT_SINGLELINE | DT_VCENTER);
    }

    // Main display — shrink font if long
    {
        let font = if state.display_main.len() > 10 { state.font_sub } else { state.font_main };
        let r    = RECT { left:PAD, top:40, right:cw-PAD, bottom:DISP_H-8 };
        let col  = if state.error { ERROR_COLOR } else { TEXT_WHITE };
        draw_text(mem_dc, &state.display_main, r, font, col,
            DT_RIGHT | DT_SINGLELINE | DT_VCENTER | DT_END_ELLIPSIS);
    }

    // Separator
    {
        let pen = CreatePen(PS_SOLID, 1, SEPARATOR);
        let op  = SelectObject(mem_dc, pen as HGDIOBJ);
        MoveToEx(mem_dc, 0, DISP_H, ptr::null_mut());
        LineTo(mem_dc, cw, DISP_H);
        SelectObject(mem_dc, op);
        DeleteObject(pen as HGDIOBJ);
    }

    // Buttons
    for i in 0..BUTTONS.len() {
        let btn = &BUTTONS[i];
        let r   = btn_rect(i, cw, ch);
        let hov = state.hovered_btn == i as i32;
        let prs = state.pressed_btn == i as i32;

        let (fill, text_col) = match btn.kind {
            BtnKind::Equals => (
                if prs { ACCENT_PRESS } else if hov { ACCENT_HOVER } else { ACCENT_COLOR },
                TEXT_WHITE,
            ),
            BtnKind::Op => (
                if prs { 0x00404040u32 } else if hov { BTN_HOVER } else { BTN_COLOR },
                OP_TEXT,
            ),
            BtnKind::Clear | BtnKind::Backspace => (
                if prs { CLEAR_PRESS } else if hov { CLEAR_HOVER } else { CLEAR_COLOR },
                TEXT_WHITE,
            ),
            _ => (
                if prs { BTN_PRESS } else if hov { BTN_HOVER } else { BTN_COLOR },
                TEXT_WHITE,
            ),
        };

        draw_rrect(mem_dc, &r, 12, fill);
        draw_text(mem_dc, btn.label, r, state.font_btn, text_col,
            DT_CENTER | DT_SINGLELINE | DT_VCENTER);
    }

    BitBlt(hdc_win, 0, 0, cw, ch, mem_dc, 0, 0, SRCCOPY);
    SelectObject(mem_dc, old_bm);
    DeleteObject(hbm);
    DeleteDC(mem_dc);
    EndPaint(hwnd, &ps);
}

// ── Clipboard ─────────────────────────────────────────────────────
unsafe fn copy_to_clipboard(hwnd: HWND, text: &str) {
    let ws    = to_wstring(text);
    let hmem  = GlobalAlloc(GMEM_MOVEABLE, ws.len() * 2);
    if hmem.is_null() { return; }
    let ptr   = GlobalLock(hmem) as *mut u16;
    if ptr.is_null() { return; }
    std::ptr::copy_nonoverlapping(ws.as_ptr(), ptr, ws.len());
    GlobalUnlock(hmem);
    OpenClipboard(hwnd); EmptyClipboard();
    SetClipboardData(CF_UNICODETEXT, hmem);
    CloseClipboard();
}

unsafe fn paste_from_clipboard() -> Option<String> {
    if IsClipboardFormatAvailable(CF_UNICODETEXT) == 0 { return None; }
    OpenClipboard(ptr::null_mut());
    let hmem = GetClipboardData(CF_UNICODETEXT);
    if hmem.is_null() { CloseClipboard(); return None; }
    let ptr = GlobalLock(hmem) as *const u16;
    if ptr.is_null() { CloseClipboard(); return None; }
    let s = wstring_to_string(ptr);
    GlobalUnlock(hmem);
    CloseClipboard();
    Some(s)
}

// ── Font init ─────────────────────────────────────────────────────
unsafe fn make_font(size: i32, weight: i32) -> HFONT {
    let face = to_wstring("Segoe UI");
    CreateFontW(size, 0, 0, 0, weight,
        0, 0, 0,
        ANSI_CHARSET as DWORD, OUT_TT_PRECIS as DWORD,
        CLIP_DEFAULT_PRECIS as DWORD, CLEARTYPE_QUALITY as DWORD,
        (VARIABLE_PITCH | FF_SWISS) as DWORD,
        face.as_ptr())
}

unsafe fn init_fonts(state: &mut AppState) {
    state.font_main = make_font(38, FW_NORMAL);
    state.font_sub  = make_font(18, FW_NORMAL);
    state.font_btn  = make_font(19, FW_NORMAL);
}

// ── Window procedure ──────────────────────────────────────────────
unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: UINT, wp: WPARAM, lp: LPARAM) -> LRESULT {
    let sp = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut AppState;

    match msg {
        WM_CREATE => {
            SetTimer(hwnd, 1, 16, ptr::null_mut());
            0
        }
        WM_TIMER => {
            InvalidateRect(hwnd, ptr::null(), 0);
            0
        }
        WM_PAINT => {
            if !sp.is_null() { paint(hwnd, &*sp); } else {
                let mut ps: PAINTSTRUCT = mem::zeroed();
                BeginPaint(hwnd, &mut ps); EndPaint(hwnd, &ps);
            }
            0
        }
        WM_ERASEBKGND => 1,

        WM_LBUTTONDOWN => {
            if sp.is_null() { return DefWindowProcW(hwnd,msg,wp,lp); }
            let s  = &mut *sp;
            let mx = (lp & 0xFFFF) as i16 as i32;
            let my = ((lp >> 16) & 0xFFFF) as i16 as i32;
            let mut cr: RECT = mem::zeroed(); GetClientRect(hwnd, &mut cr);
            let idx = hit_test(mx, my, cr.right, cr.bottom);
            if idx >= 0 { s.pressed_btn = idx; InvalidateRect(hwnd, ptr::null(), 0); }
            SetFocus(hwnd);
            0
        }
        WM_LBUTTONUP => {
            if sp.is_null() { return DefWindowProcW(hwnd,msg,wp,lp); }
            let s  = &mut *sp;
            let mx = (lp & 0xFFFF) as i16 as i32;
            let my = ((lp >> 16) & 0xFFFF) as i16 as i32;
            let mut cr: RECT = mem::zeroed(); GetClientRect(hwnd, &mut cr);
            let idx = hit_test(mx, my, cr.right, cr.bottom);
            if idx >= 0 && s.pressed_btn == idx { s.handle_button(idx as usize); }
            s.pressed_btn = -1;
            InvalidateRect(hwnd, ptr::null(), 0);
            0
        }
        WM_MOUSEMOVE => {
            if sp.is_null() { return DefWindowProcW(hwnd,msg,wp,lp); }
            let s  = &mut *sp;
            let mx = (lp & 0xFFFF) as i16 as i32;
            let my = ((lp >> 16) & 0xFFFF) as i16 as i32;
            let mut cr: RECT = mem::zeroed(); GetClientRect(hwnd, &mut cr);
            let idx = hit_test(mx, my, cr.right, cr.bottom);
            if s.hovered_btn != idx {
                s.hovered_btn = idx;
                InvalidateRect(hwnd, ptr::null(), 0);
                let mut tme = TRACKMOUSEEVENT {
                    cb_size: mem::size_of::<TRACKMOUSEEVENT>() as DWORD,
                    dw_flags: TME_LEAVE, hwnd_track: hwnd, dw_hover_time: 0,
                };
                TrackMouseEvent(&mut tme);
            }
            0
        }
        WM_MOUSELEAVE => {
            if !sp.is_null() {
                let s = &mut *sp;
                s.hovered_btn = -1; s.pressed_btn = -1;
                InvalidateRect(hwnd, ptr::null(), 0);
            }
            0
        }
        WM_KEYDOWN => {
            if sp.is_null() { return DefWindowProcW(hwnd,msg,wp,lp); }
            let s   = &mut *sp;
            let ctrl = (GetKeyState(VK_CONTROL as i32) & 0x8000u16 as i16) != 0;
            match wp {
                VK_RETURN => s.calculate(),
                VK_ESCAPE => s.clear(),
                VK_BACK   => s.backspace(),
                VK_C if ctrl => copy_to_clipboard(hwnd, &s.display_main),
                VK_V if ctrl => {
                    if let Some(t) = paste_from_clipboard() {
                        for ch in t.chars() {
                            if ch.is_ascii_digit() || ch == '.' { s.input_char(&ch.to_string()); }
                            else if "+-*/".contains(ch) { s.input_char(&ch.to_string()); }
                        }
                    }
                }
                _ => return DefWindowProcW(hwnd,msg,wp,lp),
            }
            InvalidateRect(hwnd, ptr::null(), 0);
            0
        }
        WM_CHAR => {
            if sp.is_null() { return DefWindowProcW(hwnd,msg,wp,lp); }
            let s    = &mut *sp;
            let ctrl = (GetKeyState(VK_CONTROL as i32) & 0x8000u16 as i16) != 0;
            if ctrl { return 0; }
            let ch = char::from_u32(wp as u32).unwrap_or('\0');
            match ch {
                '0'..='9' => s.input_char(&ch.to_string()),
                '+'  => s.input_char("+"),
                '-'  => s.input_char("−"),
                '*'  => s.input_char("×"),
                '/'  => s.input_char("÷"),
                '.'|',' => s.input_char("."),
                '('  => s.input_char("("),
                ')'  => s.input_char(")"),
                '%'  => s.input_char("%"),
                '='  => s.calculate(),
                '\r' | '\x08' => {} // handled in WM_KEYDOWN
                _ => return DefWindowProcW(hwnd,msg,wp,lp),
            }
            InvalidateRect(hwnd, ptr::null(), 0);
            0
        }
        WM_DESTROY => {
            KillTimer(hwnd, 1);
            if !sp.is_null() {
                let s = &*sp;
                if !s.font_main.is_null() { DeleteObject(s.font_main as HGDIOBJ); }
                if !s.font_sub.is_null()  { DeleteObject(s.font_sub  as HGDIOBJ); }
                if !s.font_btn.is_null()  { DeleteObject(s.font_btn  as HGDIOBJ); }
                let _ = Box::from_raw(sp);
            }
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, msg, wp, lp),
    }
}

// ── Entry point ───────────────────────────────────────────────────
fn main() {
    unsafe {
        let hinstance  = GetModuleHandleW(ptr::null());
        let class_name = to_wstring("CalcNG");
        let title      = to_wstring("Calculator-NG");

        let state = Box::new(AppState::new());
        let sp    = Box::into_raw(state);

        let wc = WNDCLASSEXW {
            cb_size:         mem::size_of::<WNDCLASSEXW>() as UINT,
            style:           CS_HREDRAW | CS_VREDRAW,
            lpfn_wnd_proc:   Some(wnd_proc),
            cb_cls_extra:    0,
            cb_wnd_extra:    0,
            h_instance:      hinstance,
            h_icon:          ptr::null_mut(),
            h_cursor:        LoadCursorW(ptr::null_mut(), IDC_ARROW as *const u16),
            hbr_background:  ptr::null_mut(),
            lpsz_menu_name:  ptr::null(),
            lpsz_class_name: class_name.as_ptr(),
            h_icon_sm:       ptr::null_mut(),
        };
        RegisterClassExW(&wc);

        let sw = GetSystemMetrics(0);
        let sh = GetSystemMetrics(1);

        let hwnd = CreateWindowExW(
            0,
            class_name.as_ptr(), title.as_ptr(),
            WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX | WS_CLIPCHILDREN,
            (sw - WIN_W) / 2, (sh - WIN_H) / 2, WIN_W, WIN_H,
            ptr::null_mut(), ptr::null_mut(), hinstance, ptr::null_mut(),
        );

        SetWindowLongPtrW(hwnd, GWLP_USERDATA, sp as isize);
        init_fonts(&mut *sp);

        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);

        let mut msg: MSG = mem::zeroed();
        while GetMessageW(&mut msg, ptr::null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}
