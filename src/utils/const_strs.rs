use super::string_pool::NameID;

macro_rules! const_strs {
    (@count () {$id: expr}) => {};
    (@count ($name_head: ident, $val_head: expr, $($name_tail: ident, $val_tail: expr,)*) {$id: expr}) => {
        #[doc = concat!("The ID of the interned string `", stringify!($val_head), "`.")]
        pub(crate) const $name_head: NameID = NameID($id);
        const_strs!(@count ($($name_tail, $val_tail,)*) {$id + 1});
    };
    
    ($($name: ident = $val: expr,)*) => {
        pub static CONST_STRS: &'static [&'static str] = &[
            $($val,)*
        ];
        
        /// Constants for ids of names which are always pooled.
        pub mod str_ids {
            use super::NameID;
            const_strs!{@count ($($name, $val,)*) {0}}
        }
    }
}

const_strs!(
    ANONYMOUS = "<anonymous>",
    _DOCTYPE = "!DOCTYPE",
    _0 = "_0",
    _1 = "_1",
    A = "a",
    ADD = "add",
    ADDRESS = "address",
    AREA = "area",
    ARTICLE = "article",
    ASIDE = "aside",
    BASE = "base",
    BIND = "bind",
    BLOCKQUOTE = "blockquote",
    BODY = "body",
    BR = "br",
    CANVAS = "canvas",
    CAPTION = "caption",
    CLASS = "class",
    CODE = "code",
    CODE_BLOCK = "code_block",
    COL = "col",
    COLGROUP = "colgroup",
    COMMAND = "command",
    DATA_LINE_NO = "data_line_no",
    DATA_PAREN_NO = "data_paren_no",
    DD = "dd",
    DETAILS = "details",
    DIV = "div",
    DL = "dl",
    DT = "dt",
    EMBED = "embed",
    ESCAPE_HTML = "escape_html",
    FIELDSET = "fieldset",
    FIGCAPTION = "figcaption",
    FIGURE = "figure",
    FILTER = "filter",
    FIRST_LINE_NO = "first_line_no",
    FOOTER = "footer",
    FORM = "form",
    FUNCTION = "function",
    H1 = "h1",
    H2 = "h2",
    H3 = "h3",
    H4 = "h4",
    H5 = "h5",
    H6 = "h6",
    HEAD = "head",
    HEADER = "header",
    HGROUP = "hgroup",
    HR = "hr",
    HREF = "href",
    HTML = "html",
    IMG = "img",
    IMPORT = "import",
    INCLUDE = "include",
    INPUT = "input",
    INT = "int",
    IS_EMPTY = "is_empty",
    JOIN = "join",
    KEY = "key",
    KEYGEN = "keygen",
    KWARGS = "kwargs",
    LANGUAGE = "language",
    LEN = "len",
    LI = "li",
    LINK = "link",
    LIST = "list",
    LIST_FILES = "list_files",
    MAIN = "main",
    MAP = "map",
    MAX_LENGTH = "max_length",
    MENU = "menu",
    MENUITEM = "menuitem",
    META = "meta",
    NAME = "name",
    NAV = "nav",
    NEGATE = "negate",
    OL = "ol",
    P = "p",
    PARAM = "param",
    PRE = "pre",
    RAISE = "raise",
    REVERSE = "reverse",
    SCRIPT = "script",
    SECTION = "section",
    SLICE = "slice",
    SORTED = "sorted",
    SOURCE = "source",
    SPAN = "span",
    STR = "str",
    TABLE = "table",
    TAG_NAME = "tag_name",
    TBODY = "tbody",
    TD = "td",
    TFOOT = "tfoot",
    TH = "th",
    THEAD = "thead",
    TR = "tr",
    TRACK = "track",
    UL = "ul",
    UNIQUE_ID = "unique_id",
    VIDEO = "video",
    WBR = "wbr",
    WRITE_FILE = "write_file",
);
