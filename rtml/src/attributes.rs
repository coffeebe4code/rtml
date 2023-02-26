use crate::*;

pub trait GlobalAttribute {}
pub trait EventAttribute {}

pub trait Attribute: std::fmt::Display + 'static {}

#[macro_export]
macro_rules! allattrs {
    ($attr:ident) => {
        impl ACompat for $attr {}
        impl AbbrCompat for $attr {}
        impl AddressCompat for $attr {}
        impl AreaCompat for $attr {}
        impl ArticleCompat for $attr {}
        impl AsideCompat for $attr {}
        impl AudioCompat for $attr {}
        impl BCompat for $attr {}
        impl BaseCompat for $attr {}
        impl BdiCompat for $attr {}
        impl BdoCompat for $attr {}
        impl BlockquoteCompat for $attr {}
        impl BodyCompat for $attr {}
        impl BrCompat for $attr {}
        impl ButtonCompat for $attr {}
        impl CanvasCompat for $attr {}
        impl CaptionCompat for $attr {}
        impl CiteCompat for $attr {}
        impl CodeCompat for $attr {}
        impl ColCompat for $attr {}
        impl ColgroupCompat for $attr {}
        impl DataCompat for $attr {}
        impl DatalistCompat for $attr {}
        impl DdCompat for $attr {}
        impl DelCompat for $attr {}
        impl DetailsCompat for $attr {}
        impl DfnCompat for $attr {}
        impl DialogCompat for $attr {}
        impl DivCompat for $attr {}
        impl DlCompat for $attr {}
        impl DtCompat for $attr {}
        impl EmCompat for $attr {}
        impl EmbedCompat for $attr {}
        impl FieldsetCompat for $attr {}
        impl FigcaptionCompat for $attr {}
        impl FigureCompat for $attr {}
        impl FooterCompat for $attr {}
        impl FormCompat for $attr {}
        impl H1Compat for $attr {}
        impl H2Compat for $attr {}
        impl H3Compat for $attr {}
        impl H4Compat for $attr {}
        impl H5Compat for $attr {}
        impl H6Compat for $attr {}
        impl HeadCompat for $attr {}
        impl HeaderCompat for $attr {}
        impl HrCompat for $attr {}
        impl HtmlCompat for $attr {}
        impl ICompat for $attr {}
        impl IframeCompat for $attr {}
        impl ImgCompat for $attr {}
        impl InputCompat for $attr {}
        impl InsCompat for $attr {}
        impl KbdCompat for $attr {}
        impl LabelCompat for $attr {}
        impl LegendCompat for $attr {}
        impl LiCompat for $attr {}
        impl LinkCompat for $attr {}
        impl MainCompat for $attr {}
        impl MapCompat for $attr {}
        impl MarkCompat for $attr {}
        impl MetaCompat for $attr {}
        impl MeterCompat for $attr {}
        impl NavCompat for $attr {}
        impl NoscriptCompat for $attr {}
        impl ObjectCompat for $attr {}
        impl OlCompat for $attr {}
        impl OptgroupCompat for $attr {}
        impl OptionCompat for $attr {}
        impl OutputCompat for $attr {}
        impl PCompat for $attr {}
        impl PictureCompat for $attr {}
        impl PreCompat for $attr {}
        impl ProgressCompat for $attr {}
        impl QCompat for $attr {}
        impl RpCompat for $attr {}
        impl RtCompat for $attr {}
        impl RubyCompat for $attr {}
        impl SCompat for $attr {}
        impl ScriptCompat for $attr {}
        impl SectionCompat for $attr {}
        impl SelectCompat for $attr {}
        impl SmallCompat for $attr {}
        impl SourceCompat for $attr {}
        impl SpanCompat for $attr {}
        impl StrongCompat for $attr {}
        impl StyleCompat for $attr {}
        impl SubCompat for $attr {}
        impl SummaryCompat for $attr {}
        impl SupCompat for $attr {}
        impl TableCompat for $attr {}
        impl TbodyCompat for $attr {}
        impl TdCompat for $attr {}
        impl TemplateCompat for $attr {}
        impl TextareaCompat for $attr {}
        impl TfootCompat for $attr {}
        impl ThCompat for $attr {}
        impl TheadCompat for $attr {}
        impl TimeCompat for $attr {}
        impl TitleCompat for $attr {}
        impl TrCompat for $attr {}
        impl TrackCompat for $attr {}
        impl UCompat for $attr {}
        impl UlCompat for $attr {}
        impl VarCompat for $attr {}
        impl VideoCompat for $attr {}
        impl WbrCompat for $attr {}

        impl GlobalAttribute for $attr {}
    };
}

#[macro_export]
macro_rules! globalattributeit {
    ($attr:ident) => {
        allattrs!($attr);
    };
}
//#[macro_export]
//macro_rules! globalattributeit {
//    ($attr:ident, $val:expr) => {
//        paste::paste! {
//        #[allow(non_camel_case_types)]
//        #[derive(Clone)]
//        pub struct [<$attr _>];
//        impl std::fmt::Display for [<$attr _>] {
//            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//                return write!(f, "{}", $val);
//            }
//        }
//        allattrs!([< $attr _>]);
//        }
//    };
//}

#[macro_export]
macro_rules! make_data {
    ($attr:ident) => {
        paste::paste! {
        #[allow(non_camel_case_types)]
        #[derive(Clone)]
        pub struct [<data _ $attr>];
        impl std::fmt::Display for [<data _ $attr>] {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                return write!(f, "{}", str::replace(stringify!([<data _ $attr >]), "_", "-"));
            }
        }
        allattrs!([< data _ $attr>]);
        }
    };
}

//macro_rules! attributeit {
//    ($attr:ident, $val:expr) => {
//        paste::paste! {
//        #[allow(non_camel_case_types)]
//        #[derive(Clone)]
//        pub struct [< $attr _ >];
//        impl fmt::Display for [<$attr _>] {
//            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//                return write!(f, "{}", $val);
//            }
//        }
//        impl Attribute for [<$attr _>] {}
//        }
//    };
//}

macro_rules! attributeit {
    ($attr:ident) => {
        impl Attribute for $attr {}
    };
}
// Global Attributes
globalattributeit! {accesskey}
globalattributeit! {class}
globalattributeit! {contenteditable}
globalattributeit! {dir}
globalattributeit! {draggable}
globalattributeit! {hidden}
globalattributeit! {id}
globalattributeit! {lang}
globalattributeit! {spellcheck}
globalattributeit! {style}
globalattributeit! {tabindex}
globalattributeit! {title}
globalattributeit! {translate}

// Special
attributeit! {_type}
attributeit! {_loop}
attributeit! {_for}
attributeit! {http_equiv}
attributeit! {accept_charset}
attributeit! {_as}
attributeit! {_async}
attributeit! {_kind}

// Event
// Window
attributeit! {onafterprint}
attributeit! {onbeforeprint}
attributeit! {onbeforeunload}
attributeit! {onerror}
attributeit! {onhashchange}
attributeit! {onload}
attributeit! {onmessage}
attributeit! {onoffline}
attributeit! {ononline}
attributeit! {onpagehide}
attributeit! {onpageshow}
attributeit! {onpopstate}
attributeit! {onresize}
attributeit! {onunload}

// Form
attributeit! {onblur}
attributeit! {onchange}
attributeit! {oncontextmenu}
attributeit! {onfocus}
attributeit! {oninput}
attributeit! {onreset}
attributeit! {onsearch}
attributeit! {onselect}
attributeit! {onsubmit}

// Keyboard
attributeit! {onkeydown}
attributeit! {onkeypress}
attributeit! {onkeyup}

// Mouse
attributeit! {onclick}
attributeit! {ondblclick}
attributeit! {onmousedown}
attributeit! {onmousemove}
attributeit! {onmouseout}
attributeit! {onmouseover}
attributeit! {onmouseup}
attributeit! {onwheel}

// Drag
attributeit! {ondrag}
attributeit! {ondragend}
attributeit! {ondragenter}
attributeit! {ondragleave}
attributeit! {ondragover}
attributeit! {ondragstart}
attributeit! {ondrop}
attributeit! {onscroll}

// Clipboard
attributeit! {oncopy}
attributeit! {oncut}
attributeit! {onpaste}

// Media
attributeit! {onabort}
attributeit! {oncanplay}
attributeit! {oncanplaythrough}
attributeit! {oncuechange}
attributeit! {ondurationchange}
attributeit! {onemptied}
attributeit! {onended}
attributeit! {onloadeddata}
attributeit! {onloadedmetadata}
attributeit! {onloadstart}
attributeit! {onpause}
attributeit! {onplay}
attributeit! {onplaying}
attributeit! {onprogress}
attributeit! {onratechange}
attributeit! {onseeked}
attributeit! {onseeking}
attributeit! {onstalled}
attributeit! {onsuspend}
attributeit! {ontimeupdate}
attributeit! {onvolumechange}
attributeit! {onwaiting}

// Misc
attributeit! {ontoggle}

// Specific Attributes
// a
attributeit! {href}
attributeit! {src}
attributeit! {download}
attributeit! {media}
attributeit! {ping}
attributeit! {referrerpolicy}
attributeit! {rel}
attributeit! {hreflang}
attributeit! {target}
// area
attributeit! {alt}
attributeit! {coords}
attributeit! {shape}
// audio
attributeit! {autoplay}
attributeit! {controls}
attributeit! {muted}
attributeit! {preload}
attributeit! {cite}
// button
attributeit! {autofocus}
attributeit! {disabled}
attributeit! {form}
attributeit! {formaction}
attributeit! {formenctype}
attributeit! {formmethod}
attributeit! {formnovalidate}
attributeit! {formtarget}
attributeit! {name}
attributeit! {value}
// canvas
attributeit! {height}
attributeit! {width}
// col
attributeit! {span}
// del
attributeit! {datetime}
// details
attributeit! {open}
// form
attributeit! {action}
attributeit! {autocomplete}
attributeit! {enctype}
attributeit! {method}
attributeit! {novalidate}
// html
attributeit! {xmlns}
// iframe
attributeit! {allow}
attributeit! {allowfullscreen}
attributeit! {allowpaymentrequest}
attributeit! {loading}
attributeit! {sandbox}
attributeit! {srcdoc}
// img
attributeit! {crossorigin}
attributeit! {decoding}
attributeit! {ismap}
attributeit! {longdesc}
attributeit! {srcset}
attributeit! {sizes}
attributeit! {usemap}
// input
attributeit! {accept}
attributeit! {checked}
attributeit! {dirname}
attributeit! {list}
attributeit! {max}
attributeit! {maxlength}
attributeit! {min}
attributeit! {minlength}
attributeit! {multiple}
attributeit! {pattern}
attributeit! {placeholder}
attributeit! {readonly}
attributeit! {required}
attributeit! {step}
// meta
attributeit! {content}
attributeit! {charset}
// meter
attributeit! {high}
attributeit! {low}
attributeit! {optimum}
// object
attributeit! {data}
// ol
attributeit! {reversed}
attributeit! {start}
// optgroup
attributeit! {label}
// option
attributeit! {selected}
// script
attributeit! {defer}
attributeit! {integrity}
attributeit! {nomodule}
// select
attributeit! {size}
// style
attributeit! {nonce}
// table
attributeit! {colspan}
// textarea
attributeit! {cols}
attributeit! {rows}
// td
attributeit! {rowspan}
// textarea
attributeit! {wrap}
// track
attributeit! {default}
attributeit! {srclang}
// th
attributeit! {headers}
attributeit! {scope}
// video
attributeit! {playsinline}
attributeit! {poster}
