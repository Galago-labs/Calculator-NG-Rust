#![windows_subsystem = "windows"]

use std::collections::VecDeque;
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

// Window styles
const WS_POPUP:        DWORD = 0x80000000;
const WS_CLIPCHILDREN: DWORD = 0x02000000;

// Messages
const WM_DESTROY:     UINT = 0x0002;
const WM_PAINT:       UINT = 0x000F;
const WM_ERASEBKGND:  UINT = 0x0014;
const WM_LBUTTONDOWN: UINT = 0x0201;
const WM_LBUTTONUP:   UINT = 0x0202;
const WM_MOUSEMOVE:   UINT = 0x0200;
const WM_MOUSELEAVE:  UINT = 0x02A3;
const WM_KEYDOWN:     UINT = 0x0100;
const WM_CHAR:        UINT = 0x0102;
const WM_CREATE:      UINT = 0x0001;
const WM_NCHITTEST:   UINT = 0x0084;
const WM_SYSCOMMAND:  UINT = 0x0112;
const SC_MINIMIZE:  WPARAM = 0xF020;

const WM_SETICON:     UINT   = 0x0080;
const ICON_SMALL:     WPARAM = 0;
const ICON_BIG:       WPARAM = 1;
const IMAGE_ICON:     UINT   = 1;
const LR_LOADFROMFILE:UINT   = 0x0010;
const LR_DEFAULTSIZE: UINT   = 0x0040;

const HTCAPTION:  LRESULT = 2;
const HTCLIENT:   LRESULT = 1;

const SW_MINIMIZE: i32 = 6;

const IDC_ARROW:  usize = 32512;
const PS_SOLID:   i32   = 0;
const NULL_PEN:   i32   = 8;  // stock object
const TRANSPARENT: i32  = 1;

const DT_CENTER:       UINT = 0x00000001;
const DT_VCENTER:      UINT = 0x00000004;
const DT_SINGLELINE:   UINT = 0x00000020;
const DT_RIGHT:        UINT = 0x00000002;
const DT_END_ELLIPSIS: UINT = 0x00008000;

const FW_NORMAL: i32 = 400;
const ANSI_CHARSET:        u8 = 0;
const OUT_TT_PRECIS:       u8 = 4;
const CLIP_DEFAULT_PRECIS: u8 = 0;
const CLEARTYPE_QUALITY:   u8 = 5;
const FF_SWISS:      u8 = 0x20;
const VARIABLE_PITCH: u8 = 2;

const SW_SHOW:    i32   = 5;
const CS_HREDRAW: DWORD = 0x0002;
const CS_VREDRAW: DWORD = 0x0001;
const GWLP_USERDATA: i32 = -21;

const VK_RETURN:  usize = 0x0D;
const VK_ESCAPE:  usize = 0x1B;
const VK_BACK:    usize = 0x08;
const VK_CONTROL: usize = 0x11;
const VK_C:       usize = 0x43;
const VK_V:       usize = 0x56;

const CF_UNICODETEXT: UINT  = 13;
const GMEM_MOVEABLE:  UINT  = 0x0002;
const SRCCOPY:        DWORD = 0x00CC0020;
const TME_LEAVE:      DWORD = 0x00000002;

// ── Colors (BGR) ──────────────────────────────────────────────────
const BG_COLOR:      COLORREF = 0x001E1E1E;
const PANEL_COLOR:   COLORREF = 0x00222222;
const BTN_COLOR:     COLORREF = 0x002D2D2D;
const BTN_HOVER:     COLORREF = 0x003D3D3D;
const BTN_PRESS:     COLORREF = 0x00505050;
// = button: green
const EQ_COLOR:      COLORREF = 0x00407040; // dark green BGR
const EQ_HOVER:      COLORREF = 0x00508050;
const EQ_PRESS:      COLORREF = 0x00306030;
const EQ_TEXT:       COLORREF = 0x0088FF88; // light green text
// C button: red
const CLEAR_COLOR:   COLORREF = 0x00203070; // dark red BGR
const CLEAR_HOVER:   COLORREF = 0x00304080;
const CLEAR_PRESS:   COLORREF = 0x00102050;
const CLEAR_TEXT:    COLORREF = 0x006688FF; // light red text
// operators
const OP_COLOR:      COLORREF = 0x002D2D2D;
const OP_HOVER:      COLORREF = 0x003D3D3D;
const OP_PRESS:      COLORREF = 0x00505050;
const OP_TEXT:       COLORREF = 0x0044AAFF;
// advanced row (√ ^ ±)
const ADV_COLOR:     COLORREF = 0x00282830;
const ADV_HOVER:     COLORREF = 0x00383845;
const ADV_PRESS:     COLORREF = 0x00484858;
const ADV_TEXT:      COLORREF = 0x00CC99FF;
// backspace
const BACK_COLOR:    COLORREF = 0x002D2D2D;
const BACK_TEXT:     COLORREF = 0x0088AAFF;
// text
const TEXT_WHITE:    COLORREF = 0x00FFFFFF;
const TEXT_GRAY:     COLORREF = 0x00AAAAAA;
const ERROR_COLOR:   COLORREF = 0x004444EE;
const SEPARATOR:     COLORREF = 0x003A3A3A;
// titlebar close btn
const CLOSE_HOVER:   COLORREF = 0x002222CC;
const CLOSE_PRESS:   COLORREF = 0x001111AA;
// titlebar minimize btn
const MIN_HOVER:     COLORREF = 0x00404040;
const MIN_PRESS:     COLORREF = 0x00303030;

// ── Win32 structs ─────────────────────────────────────────────────
#[repr(C)] struct WNDCLASSEXW {
    cb_size: UINT, style: UINT,
    lpfn_wnd_proc: Option<unsafe extern "system" fn(HWND,UINT,WPARAM,LPARAM)->LRESULT>,
    cb_cls_extra: i32, cb_wnd_extra: i32, h_instance: HINSTANCE,
    h_icon: *mut std::ffi::c_void, h_cursor: *mut std::ffi::c_void,
    hbr_background: HBRUSH, lpsz_menu_name: *const u16,
    lpsz_class_name: *const u16, h_icon_sm: *mut std::ffi::c_void,
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
#[repr(C)] #[derive(Clone,Copy,Default)] struct POINT { x: LONG, y: LONG }
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
    fn GetKeyState(n_virt_key:i32)->i16;
    fn GetCursorPos(lp_point:*mut POINT)->BOOL;
    fn ScreenToClient(h_wnd:HWND,lp_point:*mut POINT)->BOOL;
    fn DestroyWindow(h_wnd:HWND)->BOOL;
    fn LoadImageW(hinst:HINSTANCE,name:*const u16,typ:UINT,cx:i32,cy:i32,fuload:UINT)->*mut std::ffi::c_void;
    fn SendMessageW(h_wnd:HWND,msg:UINT,w_param:WPARAM,l_param:LPARAM)->LRESULT;
    fn DestroyIcon(h_icon:*mut std::ffi::c_void)->BOOL;
    fn PostMessageW(h_wnd:HWND,msg:UINT,w_param:WPARAM,l_param:LPARAM)->BOOL;
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
    fn Ellipse(hdc:HDC,left:i32,top:i32,right:i32,bottom:i32)->BOOL;
}
#[link(name="kernel32")] extern "system" {
    fn GetModuleHandleW(lp_module_name:*const u16)->HINSTANCE;
    fn GlobalAlloc(u_flags:UINT,dw_bytes:usize)->*mut std::ffi::c_void;
    fn GlobalLock(h_mem:*mut std::ffi::c_void)->*mut std::ffi::c_void;
    fn GlobalUnlock(h_mem:*mut std::ffi::c_void)->BOOL;
    fn lstrlenW(lp_string:*const u16)->i32;
}

fn to_wstring(s: &str) -> Vec<u16> { OsStr::new(s).encode_wide().chain(once(0)).collect() }
unsafe fn wstring_to_string(ptr: *const u16) -> String {
    if ptr.is_null() { return String::new(); }
    String::from_utf16_lossy(std::slice::from_raw_parts(ptr, lstrlenW(ptr) as usize))
}

// ── Error types ───────────────────────────────────────────────────
#[derive(Debug, Clone)]
enum CalcError {
    DivisionByZero,
    SqrtNegative,
    Overflow,
    EmptyExpression,
    UnexpectedToken(String),
    UnmatchedParen,
    InvalidNumber(String),
}

impl CalcError {
    /// Short message shown on the display
    fn display_msg(&self) -> &str {
        match self {
            CalcError::DivisionByZero    => "Err: Деление на 0",
            CalcError::SqrtNegative      => "Err: √ от отриц.",
            CalcError::Overflow          => "Err: Переполнение",
            CalcError::EmptyExpression   => "Err: Пусто",
            CalcError::UnexpectedToken(_)=> "Err: Синтаксис",
            CalcError::UnmatchedParen    => "Err: Скобки",
            CalcError::InvalidNumber(_)  => "Err: Число",
        }
    }
    /// Full message for history log
    fn history_msg(&self) -> String {
        match self {
            CalcError::UnexpectedToken(s) => format!("Ошибка: символ \"{}\"", s),
            CalcError::InvalidNumber(s)   => format!("Ошибка: число \"{}\"", s),
            other                         => other.display_msg().replace("Err: ", "Ошибка: "),
        }
    }
}

// ── Tokenizer ─────────────────────────────────────────────────────
#[derive(Clone, PartialEq, Debug)]
enum Token {
    Number(f64),
    Plus, Minus, Mul, Div, Pow,
    Sqrt,   // √  — prefix unary operator
    Pct,    // %  — postfix: resolved in preprocess_percent()
    LParen, RParen,
}

fn tokenize(expr: &str) -> Result<Vec<Token>, CalcError> {
    let mut tokens = Vec::new();
    let mut chars = expr.char_indices().peekable();

    while let Some((i, ch)) = chars.next() {
        match ch {
            ' '  => {}
            '+'  => tokens.push(Token::Plus),
            '-'  => tokens.push(Token::Minus),
            '*'  => tokens.push(Token::Mul),
            '/'  => tokens.push(Token::Div),
            '%'  => tokens.push(Token::Pct),
            '^'  => tokens.push(Token::Pow),
            '√'  => tokens.push(Token::Sqrt),
            '('  => tokens.push(Token::LParen),
            ')'  => tokens.push(Token::RParen),
            '0'..='9' | '.' => {
                // Collect the full number starting at byte offset i.
                // Also handles scientific notation: 1e+20, 3.14e-5
                let start = i;
                let mut end = i + ch.len_utf8();
                let mut dots = if ch == '.' { 1u32 } else { 0 };
                // Consume digits and decimal point
                while let Some(&(_, nc)) = chars.peek() {
                    if nc.is_ascii_digit() {
                        end += nc.len_utf8(); chars.next();
                    } else if nc == '.' {
                        dots += 1; end += nc.len_utf8(); chars.next();
                    } else if nc == 'e' || nc == 'E' {
                        // Peek further: 'e' must be followed by digit or +/-digit
                        end += nc.len_utf8(); chars.next();
                        if let Some(&(_, sc)) = chars.peek() {
                            if sc == '+' || sc == '-' {
                                end += sc.len_utf8(); chars.next();
                            }
                        }
                        // Consume exponent digits
                        while let Some(&(_, dc)) = chars.peek() {
                            if dc.is_ascii_digit() { end += dc.len_utf8(); chars.next(); }
                            else { break; }
                        }
                        break; // 'e' part consumed — done with this number
                    } else {
                        break;
                    }
                }
                let s = &expr[start..end];
                if dots > 1 { return Err(CalcError::InvalidNumber(s.into())); }
                let n = s.parse::<f64>()
                    .map_err(|_| CalcError::InvalidNumber(s.into()))?;
                tokens.push(Token::Number(n));
            }
            c => return Err(CalcError::UnexpectedToken(c.to_string())),
        }
    }
    Ok(tokens)
}

// ── Implicit multiplication ───────────────────────────────────────
// Inserts Mul tokens where multiplication is implied by juxtaposition:
//   2(6)    → 2 * (6)       Number LParen
//   (2)(3)  → (2) * (3)     RParen LParen
//   (4)5    → (4) * 5       RParen Number  (less common but consistent)
//   √(4)(2) → √(4) * (2)    RParen LParen (already handled)
//   2√(9)   → 2 * √(9)      Number Sqrt
fn implicit_mul(tokens: Vec<Token>) -> Vec<Token> {
    let mut out = Vec::with_capacity(tokens.len() + 4);
    for (i, tok) in tokens.iter().enumerate() {
        if i > 0 {
            let prev = &tokens[i - 1];
            let needs_mul = matches!(
                (prev, tok),
                // Number followed by opening context
                (Token::Number(_), Token::LParen) |
                (Token::Number(_), Token::Sqrt)   |
                // Closing paren followed by opening context
                (Token::RParen,    Token::LParen) |
                (Token::RParen,    Token::Number(_)) |
                (Token::RParen,    Token::Sqrt)
            );
            if needs_mul {
                out.push(Token::Mul);
            }
        }
        out.push(tok.clone());
    }
    out
}

// ── Percent preprocessing ─────────────────────────────────────────
// Transforms Pct tokens into proper arithmetic before parsing.
//
// Four rules (matching standard calculator behaviour):
//   n %              → n / 100              e.g. 10%      = 0.1
//   a % b            → a * b / 100          e.g. 123%10   = 12.3
//   base + n %       → base + base*n/100    e.g. 100+10%  = 110
//   base - n %       → base - base*n/100    e.g. 50-15%   = 42.5
//   base * n %       → base * n/100         e.g. 200*10%  = 20
fn preprocess_percent(tokens: Vec<Token>) -> Result<Vec<Token>, CalcError> {
    let mut t = tokens;
    let mut i = 0;
    while i < t.len() {
        if t[i] == Token::Pct {
            let prev_ok = i > 0 && matches!(t[i-1], Token::Number(_) | Token::RParen);
            if !prev_ok {
                return Err(CalcError::UnexpectedToken("%".into()));
            }

            // Rule: "a % b" → "a * b / 100"  (e.g. 123%10 = 12.3)
            // If a number immediately follows %, treat as multiplicative percent
            let next_is_number = matches!(t.get(i + 1), Some(Token::Number(_)));
            if next_is_number {
                // Tokens before: [..., a, Pct, b, ...]
                // Replace Pct with Mul, insert Div 100 after b
                t[i] = Token::Mul;
                t.insert(i + 2, Token::Number(100.0));
                t.insert(i + 2, Token::Div);
                // i+2 = Div, i+3 = 100.0 — skip past them
                i += 3;
                continue;
            }

            // Standard rules: no number follows %

            // Find nearest top-level + or - to the left
            let mut op_pos: Option<usize> = None;
            let mut is_additive = false;
            let mut depth: i32 = 0;
            let mut j = i as i32 - 1;
            while j >= 0 {
                match &t[j as usize] {
                    Token::RParen => depth += 1,
                    Token::LParen => depth -= 1,
                    Token::Plus | Token::Minus if depth == 0 => {
                        op_pos = Some(j as usize);
                        is_additive = true;
                        break;
                    }
                    Token::Mul | Token::Div if depth == 0 => {
                        op_pos = Some(j as usize);
                        is_additive = false;
                        break;
                    }
                    _ => {}
                }
                j -= 1;
            }

            // Replace Pct with Mul Number(0.01)
            t[i] = Token::Number(0.01);
            t.insert(i, Token::Mul);

            // For additive ops: inject "(base) *" after operator
            if is_additive {
                if let Some(op_idx) = op_pos {
                    let base_len = op_idx;
                    let mut inject = Vec::with_capacity(base_len + 3);
                    inject.push(Token::LParen);
                    inject.extend_from_slice(&t[0..op_idx]);
                    inject.push(Token::RParen);
                    inject.push(Token::Mul);
                    let insert_pos = op_idx + 1;
                    for (k, tok) in inject.into_iter().enumerate() {
                        t.insert(insert_pos + k, tok);
                    }
                }
            }
        }
        i += 1;
    }
    Ok(t)
}

// ── Recursive descent parser ──────────────────────────────────────
// Priority (high to low): unary(- √) > ^ > * / > + -
// (% is gone by the time we parse — resolved by preprocess_percent)
struct Parser { tokens: Vec<Token>, pos: usize }

impl Parser {
    fn new(t: Vec<Token>) -> Self { Parser { tokens: t, pos: 0 } }
    fn peek(&self) -> Option<&Token> { self.tokens.get(self.pos) }
    fn consume(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let t = self.tokens[self.pos].clone(); self.pos += 1; Some(t)
        } else { None }
    }

    fn parse_expr(&mut self) -> Result<f64, CalcError> { self.parse_add() }

    fn parse_add(&mut self) -> Result<f64, CalcError> {
        let mut l = self.parse_mul()?;
        loop {
            match self.peek() {
                Some(Token::Plus)  => { self.consume(); l += self.parse_mul()?; }
                Some(Token::Minus) => { self.consume(); l -= self.parse_mul()?; }
                _ => break,
            }
        }
        Ok(l)
    }

    fn parse_mul(&mut self) -> Result<f64, CalcError> {
        let mut l = self.parse_pow()?;
        loop {
            match self.peek() {
                Some(Token::Mul) => { self.consume(); l *= self.parse_pow()?; }
                Some(Token::Div) => {
                    self.consume();
                    let r = self.parse_pow()?;
                    if r == 0.0 { return Err(CalcError::DivisionByZero); }
                    l /= r;
                }
                _ => break,
            }
        }
        Ok(l)
    }

    // ^ is right-associative: 2^3^2 = 2^(3^2) = 512
    fn parse_pow(&mut self) -> Result<f64, CalcError> {
        let base = self.parse_unary()?;
        if let Some(Token::Pow) = self.peek() {
            self.consume();
            let exp = self.parse_pow()?;
            let r = base.powf(exp);
            if r.is_infinite() { return Err(CalcError::Overflow); }
            Ok(r)
        } else {
            Ok(base)
        }
    }

    fn parse_unary(&mut self) -> Result<f64, CalcError> {
        match self.peek() {
            Some(Token::Minus) => { self.consume(); Ok(-self.parse_unary()?) }
            Some(Token::Plus)  => { self.consume(); self.parse_unary() }
            Some(Token::Sqrt)  => {
                self.consume();
                let val = self.parse_unary()?;
                if val < 0.0 { return Err(CalcError::SqrtNegative); }
                Ok(val.sqrt())
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<f64, CalcError> {
        match self.peek() {
            Some(Token::Number(_)) => {
                if let Some(Token::Number(n)) = self.consume() { Ok(n) }
                else { Err(CalcError::UnexpectedToken("?".into())) }
            }
            Some(Token::LParen) => {
                self.consume();
                let v = self.parse_expr()?;
                match self.consume() {
                    Some(Token::RParen) => Ok(v),
                    _ => Err(CalcError::UnmatchedParen),
                }
            }
            _ => Err(CalcError::UnexpectedToken(
                format!("{:?}", self.peek())
            )),
        }
    }
}

fn evaluate(expr: &str) -> Result<f64, CalcError> {
    let expr = expr.trim();
    if expr.is_empty() { return Err(CalcError::EmptyExpression); }
    let raw_tokens = tokenize(expr)?;
    if raw_tokens.is_empty() { return Err(CalcError::EmptyExpression); }
    let with_impl  = implicit_mul(raw_tokens);    // "2(6)" → "2*(6)"
    let tokens     = preprocess_percent(with_impl)?;
    let mut p = Parser::new(tokens);
    let r = p.parse_expr()?;
    if p.pos != p.tokens.len() {
        return Err(CalcError::UnexpectedToken("лишние символы".into()));
    }
    if r.is_nan() { return Err(CalcError::Overflow); }
    if r.is_infinite() { return Err(CalcError::Overflow); }
    Ok(r)
}

fn format_result(n: f64) -> String {
    // Scientific notation for very large or very small values
    if n != 0.0 && (n.abs() >= 1e15 || n.abs() < 1e-9) {
        // e.g. "1.23e+15" — compact and unambiguous
        let s = format!("{:.6e}", n);
        // Trim trailing zeros in mantissa: "1.230000e+15" → "1.23e+15"
        let (mantissa, exp) = s.split_once('e').unwrap_or((&s, ""));
        let m = mantissa.trim_end_matches('0').trim_end_matches('.');
        return format!("{}e{}", m, exp);
    }

    if n.fract() == 0.0 {
        // Integer — add space as thousands separator for readability
        let int_val = n as i64;
        let abs_val = int_val.unsigned_abs();
        let digits = abs_val.to_string();
        let grouped = group_digits(&digits);
        if int_val < 0 {
            return format!("-{}", grouped);
        }
        return grouped;
    }

    // Floating point — trim to at most 10 significant decimal places
    let s = format!("{:.10}", n);
    s.trim_end_matches('0').trim_end_matches('.').to_string()
}

// Clean ASCII representation for use in expression (no thousands separators).
// This is what gets stored in self.expression after calculate().
// All output is pure ASCII — safe for evaluate() and tokenize().
fn format_clean(n: f64) -> String {
    if n.is_nan() || n.is_infinite() { return "0".into(); }
    // Large/small: use same scientific format as format_result, but ASCII-safe
    // format!("{}", 1e20f64) gives "100000000000000000000" (ugly, 21 chars)
    // format_result gives "1e+20" — use the same compact form here
    if n != 0.0 && (n.abs() >= 1e15 || n.abs() < 1e-9) {
        let s = format!("{:.6e}", n);
        let (mantissa, exp) = s.split_once('e').unwrap_or((&s, ""));
        let m = mantissa.trim_end_matches('0').trim_end_matches('.');
        return format!("{}e{}", m, exp);  // e.g. "1e+20", "-3.14e-15"
    }
    if n.fract() == 0.0 && n.abs() < 1e15 {
        return format!("{}", n as i64);
    }
    let s = format!("{:.10}", n);
    s.trim_end_matches('0').trim_end_matches('.').to_string()
}

// Insert narrow no-break space every 3 digits from the right: "1234567" → "1 234 567"
fn group_digits(s: &str) -> String {
    let len = s.len();
    if len <= 3 {
        // No grouping needed — return a copy only because caller needs owned String.
        // For digits-only input this is always ASCII so len == char count.
        return s.into();
    }
    let mut result = String::with_capacity(len + len / 3);
    for (i, ch) in s.chars().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            result.push('\u{202F}'); // narrow no-break space
        }
        result.push(ch);
    }
    result
}

// ── Button definitions ────────────────────────────────────────────
#[derive(Clone, PartialEq)]
enum BtnKind {
    Digit, Op, Equals, Clear, Backspace, Dot,
    Negate, Paren, Percent, Sqrt, Power, History,
}

struct Btn { label: &'static str, kind: BtnKind, row: i32, col: i32, row_span: i32 }

// Row 0:  C   ←    %    ÷
// Row 1:  7   8    9    ×
// Row 2:  4   5    6    −
// Row 3:  1   2    3    =  (tall, rows 3+4)
// Row 4:  0   .   ()    =  (col3 occupied)
// Row 5:  √   ^   ±    (empty — history btn drawn separately)
const BUTTONS: &[Btn] = &[
    // Row 0
    Btn{label:"C",   kind:BtnKind::Clear,    row:0,col:0,row_span:1},
    Btn{label:"←",   kind:BtnKind::Backspace,row:0,col:1,row_span:1},
    Btn{label:"%",   kind:BtnKind::Percent,  row:0,col:2,row_span:1},
    Btn{label:"÷",   kind:BtnKind::Op,       row:0,col:3,row_span:1},
    // Row 1
    Btn{label:"7",   kind:BtnKind::Digit,    row:1,col:0,row_span:1},
    Btn{label:"8",   kind:BtnKind::Digit,    row:1,col:1,row_span:1},
    Btn{label:"9",   kind:BtnKind::Digit,    row:1,col:2,row_span:1},
    Btn{label:"×",   kind:BtnKind::Op,       row:1,col:3,row_span:1},
    // Row 2
    Btn{label:"4",   kind:BtnKind::Digit,    row:2,col:0,row_span:1},
    Btn{label:"5",   kind:BtnKind::Digit,    row:2,col:1,row_span:1},
    Btn{label:"6",   kind:BtnKind::Digit,    row:2,col:2,row_span:1},
    Btn{label:"−",   kind:BtnKind::Op,       row:2,col:3,row_span:1},
    // Row 3
    Btn{label:"1",   kind:BtnKind::Digit,    row:3,col:0,row_span:1},
    Btn{label:"2",   kind:BtnKind::Digit,    row:3,col:1,row_span:1},
    Btn{label:"3",   kind:BtnKind::Digit,    row:3,col:2,row_span:1},
    Btn{label:"=",   kind:BtnKind::Equals,   row:3,col:3,row_span:2}, // TALL =
    // Row 4
    Btn{label:"0",   kind:BtnKind::Digit,    row:4,col:0,row_span:1},
    Btn{label:".",   kind:BtnKind::Dot,      row:4,col:1,row_span:1},
    Btn{label:"( )", kind:BtnKind::Paren,    row:4,col:2,row_span:1},
    // col3 row4 occupied by tall =
    // Row 5 — advanced
    Btn{label:"√",   kind:BtnKind::Sqrt,     row:5,col:0,row_span:1},
    Btn{label:"^",   kind:BtnKind::Power,    row:5,col:1,row_span:1},
    Btn{label:"±",   kind:BtnKind::Negate,   row:5,col:2,row_span:1},
    Btn{label:"🕐",  kind:BtnKind::History,  row:5,col:3,row_span:1},
];

const MAX_HISTORY: usize = 20;
const HISTORY_PANEL_H: i32 = 220; // height of history overlay

// ── Static wstring cache ──────────────────────────────────────────
// Strings that are painted every frame are pre-converted once at startup.
// This avoids Vec<u16> allocations on every WM_PAINT.
struct WStrings {
    title:      Vec<u16>,
    history:    Vec<u16>,
    close:      Vec<u16>,
    minimize:   Vec<u16>,  // "—" minimize button label
    btn_labels: Vec<Vec<u16>>,
}

impl WStrings {
    fn new() -> Self {
        WStrings {
            title:      to_wstring("Calculator-NG"),
            history:    to_wstring("История вычислений"),
            close:      to_wstring("✕"),
            minimize:   to_wstring("—"),
            btn_labels: BUTTONS.iter().map(|b| to_wstring(b.label)).collect(),
        }
    }
}

// ── Font resources ───────────────────────────────────────────────
struct Fonts {
    main:    HFONT,
    sub:     HFONT,
    btn:     HFONT,
    small:   HFONT,
    history: HFONT,
}

impl Fonts {
    fn null() -> Self {
        Fonts {
            main: ptr::null_mut(), sub: ptr::null_mut(),
            btn:  ptr::null_mut(), small: ptr::null_mut(),
            history: ptr::null_mut(),
        }
    }
    unsafe fn load() -> Self {
        Fonts {
            main:    make_font(34, FW_NORMAL),
            sub:     make_font(16, FW_NORMAL),
            btn:     make_font(17, FW_NORMAL),
            small:   make_font(13, FW_NORMAL),
            history: make_font(12, FW_NORMAL),
        }
    }
    unsafe fn free(&self) {
        for &f in &[self.main, self.sub, self.btn, self.small, self.history] {
            if !f.is_null() { DeleteObject(f as HGDIOBJ); }
        }
    }
}

// ── App state ─────────────────────────────────────────────────────
struct AppState {
    expression:      String,
    display_main:    String,
    display_sub:     String,
    just_calculated: bool,
    error:           bool,
    result_raw:      f64,       // clean numeric result, used when chaining after calculate()
    display_char_count: usize,  // chars().count() of display_main — cached to avoid O(n) in paint
    history:         VecDeque<String>,
    show_history:    bool,
    hovered_btn:     i32,
    pressed_btn:     i32,
    close_hovered:   bool,
    close_pressed:   bool,
    min_hovered:     bool,
    min_pressed:     bool,
    fonts:           Fonts,
    ws:              WStrings, // pre-built wstrings for static labels
    btn_rects:       Vec<RECT>, // pre-computed button geometry (fixed window size)
}

impl AppState {
    fn new() -> Self {
        AppState {
            expression: String::new(), display_main: "0".into(), display_sub: String::new(),
            just_calculated: false, error: false, result_raw: 0.0,
            display_char_count: 1, // "0".chars().count() == 1
            history: VecDeque::new(), show_history: false,
            hovered_btn: -1, pressed_btn: -1,
            close_hovered: false, close_pressed: false,
            min_hovered: false, min_pressed: false,
            fonts: Fonts::null(),
            ws: WStrings::new(),
            btn_rects: precompute_btn_rects(), // safe: uses only compile-time constants
        }
    }

    fn push_history(&mut self, entry: String) {
        if self.history.len() >= MAX_HISTORY {
            self.history.pop_front(); // O(1) — VecDeque
        }
        self.history.push_back(entry);
    }

    // Always use this to update display_main — keeps display_char_count in sync.
    fn set_display_main(&mut self, s: String) {
        self.display_char_count = s.chars().count();
        self.display_main = s;
    }

    fn clear(&mut self) {
        self.expression.clear(); self.set_display_main("0".into());
        self.display_sub.clear(); self.just_calculated = false;
        self.error = false; self.result_raw = 0.0;
    }

    fn backspace(&mut self) {
        if self.just_calculated || self.error { self.clear(); return; }
        if !self.expression.is_empty() {
            // Find byte boundary of last char and truncate in-place — no allocation
            let last_char_start = self.expression
                .char_indices()
                .next_back()
                .map(|(i, _)| i)
                .unwrap_or(0);
            self.expression.truncate(last_char_start);
            let dm = if self.expression.is_empty() { "0".into() } else { self.to_display() };
            self.set_display_main(dm);
        }
    }

    // Convert internal expression (ASCII operators) to display string (Unicode symbols).
    // Single pass — one allocation instead of three chained replace() calls.
    fn to_display(&self) -> String {
        let mut out = String::with_capacity(self.expression.len() + 8);
        for ch in self.expression.chars() {
            match ch {
                '*' => out.push('×'),
                '/' => out.push('÷'),
                '-' => out.push('−'),
                c   => out.push(c),
            }
        }
        out
    }

    fn input_str(&mut self, s: &str) {
        if self.error { self.clear(); }

        // Determine once — used in two places below
        let is_op = matches!(s, "+" | "-" | "*" | "/" | "%" | "^");

        if self.just_calculated {
            if is_op {
                // expression already contains clean ASCII from calculate() — use it directly
                // do NOT use display_main which may contain U+202F thousands separator
                // expression is already set correctly, just append the operator below
            } else {
                self.expression.clear(); self.display_sub.clear();
            }
            self.just_calculated = false;
        }
        // Limit expression length by character count (not bytes).
        // '√' is 3 bytes but 1 char — byte limit would be ~21 ops, char limit is 64.
        if self.expression.chars().count() > 64 { return; }

        // Replace trailing binary operator with new one
        if is_op {
            if let Some(last) = self.expression.chars().last() {
                if "+-*/%^".contains(last) { self.expression.pop(); }
            }
        }
        self.expression.push_str(s);
        self.set_display_main(self.to_display());
    }

    fn calculate(&mut self) {
        if self.expression.is_empty() { return; }
        let disp = self.to_display();
        match evaluate(&self.expression) {
            Ok(r) => {
                let formatted = format_result(r);
                // expression must stay clean ASCII — store the raw number as string
                // format_result may add U+202F (thousands sep) — never put that in expression
                let clean = format_clean(r);
                let entry = format!("{} = {}", disp, formatted);
                self.push_history(entry);
                self.display_sub  = format!("{} =", disp);
                self.set_display_main(formatted);
                self.expression   = clean;   // clean ASCII, safe for evaluate()
                self.result_raw   = r;
                self.just_calculated = true; self.error = false;
            }
            Err(e) => {
                self.push_history(format!("{} → {}", disp, e.history_msg()));
                self.display_sub  = disp;
                self.set_display_main(e.display_msg().into());
                self.expression.clear();
                self.result_raw   = 0.0;
                self.error = true; self.just_calculated = false;
            }
        }
    }

    fn toggle_paren(&mut self) {
        if self.error { self.clear(); }
        if self.just_calculated { self.expression.clear(); self.display_sub.clear(); self.just_calculated = false; }
        // Count open/close parens in one pass instead of two filter() calls
        let mut open = 0i32;
        for ch in self.expression.chars() {
            if ch == '(' { open += 1; }
            else if ch == ')' { open -= 1; }
        }
        // open > 0 means there are unclosed parens → insert ')'
        if open > 0 { self.input_str(")"); } else { self.input_str("("); }
    }

    fn negate(&mut self) {
        if self.expression.is_empty() || self.display_main == "0" {
            self.expression = "-".into(); self.set_display_main("−".into()); return;
        }
        // Simple negative number (e.g. "-5"): strip the leading minus in-place
        if self.expression.starts_with('-')
            && !self.expression[1..].contains(['+','-','*','/','%','^','('])
        {
            self.expression.drain(0..1); // removes '-' without reallocating
        } else {
            // Complex expression: wrap in -(...)
            self.expression = format!("-({})", self.expression);
        }
        self.set_display_main(self.to_display());
    }

    fn apply_sqrt(&mut self) {
        if self.error { self.clear(); return; }
        if self.expression.is_empty() { return; }

        // Find where the last operand starts — scan back from end,
        // stopping at a top-level binary operator (outside parens).
        // This ensures "2+" → √ inserts as "2+√(...)" not "√(2+)"
        let bytes = self.expression.as_bytes();
        let mut depth: i32 = 0;
        let mut split: usize = 0; // byte index where last operand begins
        let mut j = bytes.len() as i32 - 1;
        while j >= 0 {
            let b = bytes[j as usize];
            if b == b')' { depth += 1; }
            else if b == b'(' { depth -= 1; }
            else if depth == 0 && b"+-*/^".contains(&b) {
                split = (j + 1) as usize;
                break;
            }
            j -= 1;
        }

        let prefix = &self.expression[..split];
        let operand = &self.expression[split..];

        if operand.is_empty() {
            // Nothing after operator yet — can't apply sqrt
            return;
        }

        self.expression = format!("{}√({})", prefix, operand);
        // Immediately evaluate (√ is a complete function)
        self.calculate();
    }

    fn handle_button(&mut self, idx: usize) {
        match BUTTONS[idx].kind {
            BtnKind::Digit     => self.input_str(BUTTONS[idx].label),
            BtnKind::Op        => {
                let s = match BUTTONS[idx].label { "÷"=>"/", "×"=>"*", "−"=>"-", o=>o };
                self.input_str(s);
            }
            BtnKind::Equals    => self.calculate(),
            BtnKind::Clear     => self.clear(),
            BtnKind::Backspace => self.backspace(),
            BtnKind::Negate    => self.negate(),
            BtnKind::Paren     => self.toggle_paren(),
            BtnKind::Percent   => self.input_str("%"),
            BtnKind::Power     => self.input_str("^"),
            BtnKind::Sqrt      => self.apply_sqrt(),
            BtnKind::History   => self.show_history = !self.show_history,
            BtnKind::Dot       => {
                let last_op  = self.expression.rfind(|c: char| "+-*/%^".contains(c)).unwrap_or(0);
                let last_dot = self.expression.rfind('.').unwrap_or(usize::MAX);
                if last_dot == usize::MAX || last_dot < last_op {
                    self.input_str(".");
                }
            }
        }
    }

}

// ── Layout ────────────────────────────────────────────────────────
// Window size is intentionally fixed (not resizable by user).
// All mouse hit-testing uses these constants directly — no GetClientRect
// needed in event handlers. Only paint() calls GetClientRect as a
// safe fallback, though the values will always match WIN_W/WIN_H.
const WIN_W:    i32 = 320;
const WIN_H:    i32 = 610;
const TITLE_H:  i32 = 32;   // custom title bar height
const DISP_H:   i32 = 110;  // display panel height (below title)
const HEADER_H: i32 = TITLE_H + DISP_H; // total top area
const PAD:      i32 = 10;
const BTN_GAP:  i32 = 7;
const COLS:     i32 = 4;
const ROWS:     i32 = 6;

fn btn_rect(idx: usize, cw: i32, ch: i32) -> RECT {
    let b  = &BUTTONS[idx];
    let aw = cw - PAD * 2;
    let ah = ch - HEADER_H - PAD * 2;
    let bw = (aw - BTN_GAP * (COLS - 1)) / COLS;
    let bh = (ah - BTN_GAP * (ROWS - 1)) / ROWS;
    let x  = PAD + b.col * (bw + BTN_GAP);
    let y  = HEADER_H + PAD + b.row * (bh + BTN_GAP);
    let h  = bh * b.row_span + BTN_GAP * (b.row_span - 1);
    RECT { left: x, top: y, right: x + bw, bottom: y + h }
}

// Pre-compute all button rects once at startup (window size is fixed).
fn precompute_btn_rects() -> Vec<RECT> {
    (0..BUTTONS.len()).map(|i| btn_rect(i, WIN_W, WIN_H)).collect()
}

// Fast hit-test using pre-computed rects — O(n) but no rect calculations
fn hit_test_cached(mx: i32, my: i32, rects: &[RECT]) -> i32 {
    for (i, r) in rects.iter().enumerate() {
        if mx >= r.left && mx < r.right && my >= r.top && my < r.bottom { return i as i32; }
    }
    -1
}

fn close_btn_rect(cw: i32) -> RECT {
    RECT { left: cw-36, top: 6, right: cw-6, bottom: TITLE_H-6 }
}

fn min_btn_rect(cw: i32) -> RECT {
    RECT { left: cw-72, top: 6, right: cw-42, bottom: TITLE_H-6 }
}

fn in_rect(mx: i32, my: i32, r: &RECT) -> bool {
    mx >= r.left && mx < r.right && my >= r.top && my < r.bottom
}

// ── Drawing ───────────────────────────────────────────────────────
unsafe fn draw_rrect(hdc: HDC, r: &RECT, radius: i32, fill: COLORREF) {
    let brush = CreateSolidBrush(fill);
    let ob    = SelectObject(hdc, brush as HGDIOBJ);
    let op    = SelectObject(hdc, GetStockObject(NULL_PEN)); // stock — no alloc, no delete
    RoundRect(hdc, r.left, r.top, r.right, r.bottom, radius, radius);
    SelectObject(hdc, ob);
    SelectObject(hdc, op);
    DeleteObject(brush as HGDIOBJ);
}

unsafe fn draw_text_r(hdc: HDC, text: &str, mut r: RECT, font: HFONT, color: COLORREF, flags: UINT) {
    let of = SelectObject(hdc, font as HGDIOBJ);
    SetTextColor(hdc, color); SetBkMode(hdc, TRANSPARENT);
    let ws = to_wstring(text);
    DrawTextW(hdc, ws.as_ptr(), -1, &mut r, flags);
    SelectObject(hdc, of);
}

// Version using pre-built wstring — no allocation
unsafe fn draw_text_w(hdc: HDC, ws: &[u16], mut r: RECT, font: HFONT, color: COLORREF, flags: UINT) {
    let of = SelectObject(hdc, font as HGDIOBJ);
    SetTextColor(hdc, color); SetBkMode(hdc, TRANSPARENT);
    DrawTextW(hdc, ws.as_ptr(), -1, &mut r, flags);
    SelectObject(hdc, of);
}

unsafe fn draw_history_panel(hdc: HDC, state: &AppState, cw: i32, ch: i32) {
    // Semi-transparent overlay from bottom
    let panel_y = ch - HISTORY_PANEL_H;
    let panel_r = RECT { left: 0, top: panel_y, right: cw, bottom: ch };
    let bg = CreateSolidBrush(0x00181820u32);
    FillRect(hdc, &panel_r, bg);
    DeleteObject(bg as HGDIOBJ);

    // Title (cached)
    let title_r = RECT { left: PAD, top: panel_y+6, right: cw-PAD, bottom: panel_y+26 };
    draw_text_w(hdc, &state.ws.history, title_r, state.fonts.small, 0x00888888,
        DT_CENTER|DT_SINGLELINE|DT_VCENTER);

    // Separator
    let pen = CreatePen(PS_SOLID, 1, 0x00333333u32);
    let op = SelectObject(hdc, pen as HGDIOBJ);
    MoveToEx(hdc, PAD, panel_y+28, ptr::null_mut()); LineTo(hdc, cw-PAD, panel_y+28);
    SelectObject(hdc, op); DeleteObject(pen as HGDIOBJ);

    // Entries — show the most recent, oldest at top
    let entry_h = 18i32;
    let entries_to_show = ((HISTORY_PANEL_H - 36) / entry_h) as usize;
    let skip = if state.history.len() > entries_to_show { state.history.len() - entries_to_show } else { 0 };
    for (i, entry) in state.history.iter().skip(skip).enumerate() {
        let y = panel_y + 32 + i as i32 * entry_h;
        let r = RECT { left: PAD, top: y, right: cw-PAD, bottom: y+entry_h };
        let color = if entry.contains("Ошибка") { 0x006666CCu32 } else { 0x00888888u32 };
        draw_text_r(hdc, entry, r, state.fonts.history, color, DT_RIGHT|DT_SINGLELINE|DT_VCENTER);
    }
}

unsafe fn paint(hwnd: HWND, state: &AppState) {
    let mut ps: PAINTSTRUCT = mem::zeroed();
    let hdc_win = BeginPaint(hwnd, &mut ps);
    let mut cr: RECT = mem::zeroed();
    GetClientRect(hwnd, &mut cr);
    let (cw, ch) = (cr.right, cr.bottom);

    let mem_dc = CreateCompatibleDC(hdc_win);
    let hbm    = CreateCompatibleBitmap(hdc_win, cw, ch);
    let old_bm = SelectObject(mem_dc, hbm);

    // Background
    let bg_br = CreateSolidBrush(BG_COLOR); FillRect(mem_dc, &cr, bg_br); DeleteObject(bg_br as HGDIOBJ);

    // ── Custom title bar ──
    let tb_r = RECT { left:0, top:0, right:cw, bottom:TITLE_H };
    let tb_br = CreateSolidBrush(0x00181818u32); FillRect(mem_dc, &tb_r, tb_br); DeleteObject(tb_br as HGDIOBJ);
    // Title text (cached wstring — no allocation)
    let txt_r = RECT { left:PAD+20, top:0, right:cw-80, bottom:TITLE_H };
    draw_text_w(mem_dc, &state.ws.title, txt_r, state.fonts.small, 0x00666666, DT_CENTER|DT_SINGLELINE|DT_VCENTER);
    // Minimize button
    {
        let r = min_btn_rect(cw);
        let fill = if state.min_pressed { CLOSE_PRESS } else if state.min_hovered { CLOSE_HOVER } else { 0x00181818u32 };
        draw_rrect(mem_dc, &r, 5, fill);
        draw_text_w(mem_dc, &state.ws.minimize, r, state.fonts.small, 0x00888888, DT_CENTER|DT_SINGLELINE|DT_VCENTER);
    }
    // Close button
    {
        let r = close_btn_rect(cw);
        let fill = if state.close_pressed { CLOSE_PRESS } else if state.close_hovered { CLOSE_HOVER } else { 0x00181818u32 };
        draw_rrect(mem_dc, &r, 5, fill);
        draw_text_w(mem_dc, &state.ws.close, r, state.fonts.small, 0x00888888, DT_CENTER|DT_SINGLELINE|DT_VCENTER);
    }

    // ── Display panel ──
    let dp = RECT { left:0, top:TITLE_H, right:cw, bottom:HEADER_H };
    let dp_br = CreateSolidBrush(PANEL_COLOR); FillRect(mem_dc, &dp, dp_br); DeleteObject(dp_br as HGDIOBJ);

    // Sub-expression
    if !state.display_sub.is_empty() {
        let r = RECT { left:PAD, top:TITLE_H+4, right:cw-PAD, bottom:TITLE_H+28 };
        draw_text_r(mem_dc, &state.display_sub, r, state.fonts.sub, TEXT_GRAY, DT_RIGHT|DT_SINGLELINE|DT_VCENTER);
    }
    // Main display
    {
        let font = if state.display_char_count > 12 { state.fonts.sub } else { state.fonts.main };
        let r    = RECT { left:PAD, top:TITLE_H+26, right:cw-PAD, bottom:HEADER_H-5 };
        let col  = if state.error { ERROR_COLOR } else { TEXT_WHITE };
        draw_text_r(mem_dc, &state.display_main, r, font, col, DT_RIGHT|DT_SINGLELINE|DT_VCENTER|DT_END_ELLIPSIS);
    }

    // Separator
    {
        let pen = CreatePen(PS_SOLID, 1, SEPARATOR);
        let op  = SelectObject(mem_dc, pen as HGDIOBJ);
        MoveToEx(mem_dc, 0, HEADER_H, ptr::null_mut()); LineTo(mem_dc, cw, HEADER_H);
        SelectObject(mem_dc, op); DeleteObject(pen as HGDIOBJ);
    }

    // ── Buttons ──
    for i in 0..BUTTONS.len() {
        let btn = &BUTTONS[i];
        let r   = state.btn_rects[i]; // cached — no recalculation
        let hov = state.hovered_btn == i as i32;
        let prs = state.pressed_btn == i as i32;

        let (fill, text_col) = match btn.kind {
            BtnKind::Equals => (
                if prs { EQ_PRESS } else if hov { EQ_HOVER } else { EQ_COLOR },
                EQ_TEXT,
            ),
            BtnKind::Clear => (
                if prs { CLEAR_PRESS } else if hov { CLEAR_HOVER } else { CLEAR_COLOR },
                CLEAR_TEXT,
            ),
            BtnKind::Backspace => (
                if prs { BTN_PRESS } else if hov { BTN_HOVER } else { BACK_COLOR },
                BACK_TEXT,
            ),
            BtnKind::Op => (
                if prs { OP_PRESS } else if hov { OP_HOVER } else { OP_COLOR },
                OP_TEXT,
            ),
            BtnKind::Sqrt | BtnKind::Power | BtnKind::Negate | BtnKind::History => (
                if prs { ADV_PRESS } else if hov { ADV_HOVER } else { ADV_COLOR },
                ADV_TEXT,
            ),
            _ => (
                if prs { BTN_PRESS } else if hov { BTN_HOVER } else { BTN_COLOR },
                TEXT_WHITE,
            ),
        };

        let radius = if btn.row_span > 1 { 14 } else { 10 };
        draw_rrect(mem_dc, &r, radius, fill);

        // History button: draw clock icon (circle + lines) instead of emoji
        if btn.kind == BtnKind::History {
            let cx = (r.left + r.right) / 2;
            let cy = (r.top + r.bottom) / 2;
            let rad = 8i32;
            // Circle
            let pen = CreatePen(PS_SOLID, 1, ADV_TEXT);
            let brush_null = CreateSolidBrush(ADV_COLOR);
            let op = SelectObject(mem_dc, pen as HGDIOBJ);
            let ob = SelectObject(mem_dc, brush_null as HGDIOBJ);
            Ellipse(mem_dc, cx-rad, cy-rad, cx+rad, cy+rad);
            // Clock hands
            MoveToEx(mem_dc, cx, cy, ptr::null_mut()); LineTo(mem_dc, cx, cy-rad+2);
            MoveToEx(mem_dc, cx, cy, ptr::null_mut()); LineTo(mem_dc, cx+rad-3, cy);
            SelectObject(mem_dc, op); SelectObject(mem_dc, ob);
            DeleteObject(pen as HGDIOBJ); DeleteObject(brush_null as HGDIOBJ);
        } else {
            draw_text_w(mem_dc, &state.ws.btn_labels[i], r, state.fonts.btn, text_col, DT_CENTER|DT_SINGLELINE|DT_VCENTER);
        }
    }

    // ── History overlay ──
    if state.show_history {
        draw_history_panel(mem_dc, state, cw, ch);
    }

    BitBlt(hdc_win, 0, 0, cw, ch, mem_dc, 0, 0, SRCCOPY);
    SelectObject(mem_dc, old_bm); DeleteObject(hbm); DeleteDC(mem_dc);
    EndPaint(hwnd, &ps);
}

// ── Clipboard ─────────────────────────────────────────────────────
unsafe fn copy_to_clipboard(hwnd: HWND, text: &str) {
    let ws = to_wstring(text);
    let hmem = GlobalAlloc(GMEM_MOVEABLE, ws.len() * 2);
    if hmem.is_null() { return; }
    let ptr = GlobalLock(hmem) as *mut u16;
    if ptr.is_null() { return; }
    std::ptr::copy_nonoverlapping(ws.as_ptr(), ptr, ws.len());
    GlobalUnlock(hmem);
    OpenClipboard(hwnd); EmptyClipboard(); SetClipboardData(CF_UNICODETEXT, hmem); CloseClipboard();
}

unsafe fn paste_from_clipboard() -> Option<String> {
    if IsClipboardFormatAvailable(CF_UNICODETEXT) == 0 { return None; }
    OpenClipboard(ptr::null_mut());
    let hmem = GetClipboardData(CF_UNICODETEXT);
    if hmem.is_null() { CloseClipboard(); return None; }
    let ptr = GlobalLock(hmem) as *const u16;
    if ptr.is_null() { CloseClipboard(); return None; }
    let s = wstring_to_string(ptr); GlobalUnlock(hmem); CloseClipboard(); Some(s)
}

// ── Fonts ─────────────────────────────────────────────────────────
unsafe fn make_font(size: i32, weight: i32) -> HFONT {
    let face = to_wstring("Segoe UI");
    CreateFontW(size, 0, 0, 0, weight, 0, 0, 0,
        ANSI_CHARSET as DWORD, OUT_TT_PRECIS as DWORD,
        CLIP_DEFAULT_PRECIS as DWORD, CLEARTYPE_QUALITY as DWORD,
        (VARIABLE_PITCH | FF_SWISS) as DWORD, face.as_ptr())
}

// ── Window procedure ──────────────────────────────────────────────
unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: UINT, wp: WPARAM, lp: LPARAM) -> LRESULT {
    let sp = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut AppState;

    match msg {
        WM_CREATE => { 0 }

        WM_NCHITTEST => {
            // Allow dragging the title bar area.
            // Window width is fixed (WIN_W) — no need for GetClientRect syscall here.
            let mut pt = POINT::default();
            GetCursorPos(&mut pt);
            ScreenToClient(hwnd, &mut pt);
            if pt.y >= 0 && pt.y < TITLE_H {
                let close_r = close_btn_rect(WIN_W);
                let min_r   = min_btn_rect(WIN_W);
                if !in_rect(pt.x, pt.y, &close_r) && !in_rect(pt.x, pt.y, &min_r) {
                    return HTCAPTION;
                }
            }
            HTCLIENT
        }

        WM_PAINT => {
            if !sp.is_null() { paint(hwnd, &*sp); }
            else { let mut ps:PAINTSTRUCT=mem::zeroed(); BeginPaint(hwnd,&mut ps); EndPaint(hwnd,&ps); }
            0
        }
        WM_ERASEBKGND => 1,

        WM_MOUSEMOVE => {
            if sp.is_null() { return DefWindowProcW(hwnd,msg,wp,lp); }
            let s  = &mut *sp;
            let mx = (lp & 0xFFFF) as i16 as i32;
            let my = ((lp >> 16) & 0xFFFF) as i16 as i32;

            // Close button hover
            let close_r = close_btn_rect(WIN_W);
            let ch = in_rect(mx, my, &close_r);
            if ch != s.close_hovered { s.close_hovered = ch; InvalidateRect(hwnd, ptr::null(), 0); }

            // Minimize button hover
            let min_r = min_btn_rect(WIN_W);
            let mh = in_rect(mx, my, &min_r);
            if mh != s.min_hovered { s.min_hovered = mh; InvalidateRect(hwnd, ptr::null(), 0); }

            let idx = hit_test_cached(mx, my, &s.btn_rects);
            if s.hovered_btn != idx {
                s.hovered_btn = idx; InvalidateRect(hwnd, ptr::null(), 0);
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
                s.close_hovered = false; s.close_pressed = false;
                s.min_hovered = false; s.min_pressed = false;
                InvalidateRect(hwnd, ptr::null(), 0);
            }
            0
        }
        WM_LBUTTONDOWN => {
            if sp.is_null() { return DefWindowProcW(hwnd,msg,wp,lp); }
            let s  = &mut *sp;
            let mx = (lp & 0xFFFF) as i16 as i32;
            let my = ((lp >> 16) & 0xFFFF) as i16 as i32;

            // Close button?
            let close_r = close_btn_rect(WIN_W);
            if in_rect(mx, my, &close_r) { s.close_pressed = true; InvalidateRect(hwnd, ptr::null(), 0); return 0; }

            // Minimize button?
            let min_r = min_btn_rect(WIN_W);
            if in_rect(mx, my, &min_r) { s.min_pressed = true; InvalidateRect(hwnd, ptr::null(), 0); return 0; }

            let idx = hit_test_cached(mx, my, &s.btn_rects);
            if idx >= 0 { s.pressed_btn = idx; InvalidateRect(hwnd, ptr::null(), 0); }
            SetFocus(hwnd); 0
        }
        WM_LBUTTONUP => {
            if sp.is_null() { return DefWindowProcW(hwnd,msg,wp,lp); }
            let s  = &mut *sp;
            let mx = (lp & 0xFFFF) as i16 as i32;
            let my = ((lp >> 16) & 0xFFFF) as i16 as i32;

            // Close button?
            if s.close_pressed {
                s.close_pressed = false;
                let close_r = close_btn_rect(WIN_W);
                if in_rect(mx, my, &close_r) { DestroyWindow(hwnd); return 0; }
                InvalidateRect(hwnd, ptr::null(), 0); return 0;
            }

            // Minimize button?
            if s.min_pressed {
                s.min_pressed = false;
                let min_r = min_btn_rect(WIN_W);
                if in_rect(mx, my, &min_r) { ShowWindow(hwnd, SW_MINIMIZE); return 0; }
                InvalidateRect(hwnd, ptr::null(), 0); return 0;
            }

            let idx = hit_test_cached(mx, my, &s.btn_rects);
            if idx >= 0 && s.pressed_btn == idx { s.handle_button(idx as usize); }
            s.pressed_btn = -1; InvalidateRect(hwnd, ptr::null(), 0); 0
        }

        WM_KEYDOWN => {
            if sp.is_null() { return DefWindowProcW(hwnd,msg,wp,lp); }
            let s = &mut *sp;
            let ctrl = (GetKeyState(VK_CONTROL as i32) & 0x8000u16 as i16) != 0;
            match wp {
                VK_RETURN => s.calculate(),
                VK_ESCAPE => {
                    if s.show_history { s.show_history = false; }
                    else { s.clear(); }
                }
                VK_BACK   => s.backspace(),
                VK_C if ctrl => copy_to_clipboard(hwnd, &s.display_main),
                VK_V if ctrl => {
                    if let Some(t) = paste_from_clipboard() {
                        let mut buf = [0u8; 4]; // stack buffer — no heap alloc per char
                        for ch in t.chars() {
                            if ch.is_ascii_digit() || ch == '.' {
                                s.input_str(ch.encode_utf8(&mut buf));
                            } else if "+-*/%^".contains(ch) {
                                s.input_str(ch.encode_utf8(&mut buf));
                            } else if ch == '(' { s.input_str("("); }
                            else if ch == ')' { s.input_str(")"); }
                        }
                    }
                }
                _ => return DefWindowProcW(hwnd,msg,wp,lp),
            }
            InvalidateRect(hwnd, ptr::null(), 0); 0
        }
        WM_CHAR => {
            if sp.is_null() { return DefWindowProcW(hwnd,msg,wp,lp); }
            let s = &mut *sp;
            let ctrl = (GetKeyState(VK_CONTROL as i32) & 0x8000u16 as i16) != 0;
            if ctrl { return 0; }
            let ch = char::from_u32(wp as u32).unwrap_or('\0');
            match ch {
                '0'..='9' => {
                    // encode_utf8 writes into a stack buffer — no heap allocation
                    let mut buf = [0u8; 4];
                    s.input_str(ch.encode_utf8(&mut buf));
                }
                '+'  => s.input_str("+"),
                '-'  => s.input_str("-"),
                '*'  => s.input_str("*"),
                '/'  => s.input_str("/"),
                '%'  => s.input_str("%"),
                '^'  => s.input_str("^"),
                '.'|',' => s.input_str("."),
                '('  => s.input_str("("),
                ')'  => s.input_str(")"),
                '='  => s.calculate(),
                '\r' | '\x08' => {}
                _ => return DefWindowProcW(hwnd,msg,wp,lp),
            }
            InvalidateRect(hwnd, ptr::null(), 0); 0
        }

        WM_DESTROY => {
            if !sp.is_null() {
                (*sp).fonts.free();
                let _ = Box::from_raw(sp); // drop + free
            }
            PostQuitMessage(0); 0
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
            cb_cls_extra:    0, cb_wnd_extra: 0,
            h_instance:      hinstance, h_icon: ptr::null_mut(),
            h_cursor:        LoadCursorW(ptr::null_mut(), IDC_ARROW as *const u16),
            hbr_background:  ptr::null_mut(),
            lpsz_menu_name:  ptr::null(), lpsz_class_name: class_name.as_ptr(),
            h_icon_sm:       ptr::null_mut(),
        };
        RegisterClassExW(&wc);

        let sw = GetSystemMetrics(0); let sh = GetSystemMetrics(1);

        // WS_POPUP = fully borderless window, no system chrome at all
        let hwnd = CreateWindowExW(
            0,
            class_name.as_ptr(), title.as_ptr(),
            WS_POPUP | WS_CLIPCHILDREN,
            (sw - WIN_W) / 2, (sh - WIN_H) / 2, WIN_W, WIN_H,
            ptr::null_mut(), ptr::null_mut(), hinstance, ptr::null_mut(),
        );

        SetWindowLongPtrW(hwnd, GWLP_USERDATA, sp as isize);
        (*sp).fonts = Fonts::load(); // load fonts after window exists (needs HWND)

        // Load application icon from file and set on window
        // This makes it appear in Alt+Tab, taskbar preview, and title bar
        let icon_path = to_wstring("calculator.ico");
        let hicon = LoadImageW(
            ptr::null_mut(), icon_path.as_ptr(),
            IMAGE_ICON, 0, 0,
            LR_LOADFROMFILE | LR_DEFAULTSIZE,
        );
        if !hicon.is_null() {
            SendMessageW(hwnd, WM_SETICON, ICON_BIG,   hicon as LPARAM);
            SendMessageW(hwnd, WM_SETICON, ICON_SMALL, hicon as LPARAM);
            // Note: icon is owned by the window; DestroyIcon not needed until app exit
        }

        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);

        let mut msg: MSG = mem::zeroed();
        while GetMessageW(&mut msg, ptr::null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}
