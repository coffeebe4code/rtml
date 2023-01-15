pub mod attributes;
pub mod macros;
pub mod tags;

use attributes::*;
use std::fmt;
use tags::Tag;

pub trait Render: ToString {
    fn render(&self) -> String {
        self.to_string()
    }
}

impl Render for fmt::Arguments<'_> {
    fn render(&self) -> String {
        format!("{}", self)
    }
}

macro_rules! globalattributeit {
    ($attr:ident, $val:expr) => {
        #[allow(non_camel_case_types)]
        pub struct $attr;
        impl fmt::Display for $attr {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                return write!(f, "{}=\"", $val);
            }
        }
        impl Attribute for $attr {}
        impl GlobalAttribute for $attr {}
    };
}

macro_rules! attributeit {
    ($attr:ident, $val:expr) => {
        #[allow(non_camel_case_types)]
        pub struct $attr;
        impl fmt::Display for $attr {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                return write!(f, "{}=\"", $val);
            }
        }
        impl Attribute for $attr {}
    };
}

macro_rules! tagit {
    ($tag:ident, $val:expr) => {
        pub struct $tag;
        impl fmt::Display for $tag {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                return write!(f, "{}", $val);
            }
        }
        impl Tag for $tag {}
    };
}

// need a special dataAttr as its attribute name is dynamic
#[allow(non_camel_case_types)]
struct data(String);
impl fmt::Display for data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}-{}=\"", "data", self.0);
    }
}
impl Attribute for data {}

// Global Attributes
globalattributeit! {accesskey, "accesskey"}
globalattributeit! {class, "class"}
globalattributeit! {contenteditable, "contenteditable"}
globalattributeit! {dir, "dir"}
globalattributeit! {draggable, "draggable"}
globalattributeit! {hidden, "hidden"}
globalattributeit! {id, "id"}
globalattributeit! {lang, "lang"}
globalattributeit! {spellcheck, "spellcheck"}
globalattributeit! {style, "style"}
globalattributeit! {tabindex, "tabindex"}
globalattributeit! {title, "title"}
globalattributeit! {translate, "translate"}

// type unimplemented for now
// loop unimplemented for now
// accept-charset unimplemented for now
// for unimplemented for now
// http-equiv unimplemented for now
// async unimplemented for now
// kind unimplemented for now

// Specific Attributes
// a
attributeit! {href, "href"}
attributeit! {src, "src"}
attributeit! {download, "download"}
attributeit! {media, "media"}
attributeit! {ping, "ping"}
attributeit! {referrerpolicy, "referrerpolicy"}
attributeit! {rel, "rel"}
attributeit! {hreflang, "hreflang"}
attributeit! {target, "target"}
// area
attributeit! {alt, "alt"}
attributeit! {coords, "coords"}
attributeit! {shape, "shape"}
// audio
attributeit! {autoplay, "autoplay"}
attributeit! {controls, "controls"}
attributeit! {muted, "muted"}
attributeit! {preload, "preload"}
attributeit! {cite, "cite"}
// button
attributeit! {autofocus, "autofocus"}
attributeit! {disabled, "disabled"}
attributeit! {form, "form"}
attributeit! {formaction, "formaction"}
attributeit! {formenctype, "formenctype"}
attributeit! {formmethod, "formmethod"}
attributeit! {formnovalidate, "formnovalidate"}
attributeit! {formtarget, "formtarget"}
attributeit! {name, "name"}
attributeit! {value, "value"}
// canvas
attributeit! {height, "height"}
attributeit! {width, "width"}
// col
attributeit! {span, "span"}
// del
attributeit! {datetime, "datetime"}
// details
attributeit! {open, "open"}
// form
attributeit! {action, "action"}
attributeit! {autocomplete, "autocomplete"}
attributeit! {enctype, "enctype"}
attributeit! {method, "method"}
attributeit! {novalidate, "novalidate"}
// html
attributeit! {xmlns, "xmlns"}
// iframe
attributeit! {allow, "allow"}
attributeit! {allowfullscreen, "allowfullscreen"}
attributeit! {allowpaymentrequest, "allowpaymentrequest"}
attributeit! {loading, "loading"}
attributeit! {sandbox, "sandbox"}
attributeit! {srcdoc, "srcdoc"}
// img
attributeit! {crossorigin, "crossorigin"}
attributeit! {ismap, "ismap"}
attributeit! {longdesc, "longdesc"}
attributeit! {sizes, "sizes"}
attributeit! {usemap, "usemap"}
// input
attributeit! {accept, "accept"}
attributeit! {checked, "checked"}
attributeit! {dirname, "dirname"}
attributeit! {list, "list"}
attributeit! {max, "max"}
attributeit! {maxlength, "maxlength"}
attributeit! {min, "min"}
attributeit! {minlength, "minlength"}
attributeit! {multiple, "multiple"}
attributeit! {pattern, "pattern"}
attributeit! {placeholder, "placeholder"}
attributeit! {readonly, "readonly"}
attributeit! {required, "required"}
attributeit! {step, "step"}
// meta
attributeit! {content, "content"}
// meter
attributeit! {high, "high"}
attributeit! {low, "low"}
attributeit! {optimum, "optimum"}
// ol
attributeit! {reversed, "reversed"}
attributeit! {start, "start"}
// optgroup
attributeit! {label, "label"}
// option
attributeit! {selected, "selected"}
// script
attributeit! {defer, "defer"}
attributeit! {integrity, "integrity"}
attributeit! {nomodule, "nomodule"}
// select
attributeit! {size, "size"}
// td
attributeit! {rowspan, "rowspan"}
// textarea
attributeit! {wrap, "wrap"}
// th
attributeit! {headers, "headers"}
attributeit! {scope, "scope"}

tagit! {ATag, "a"}
tagit! {AbbrTag, "abbr"}
tagit! {AddressTag, "address"}
tagit! {AreaTag, "area"}
tagit! {ArticleTag, "article"}
tagit! {AsideTag, "aside"}
tagit! {AudioTag, "audio"}
tagit! {BTag, "b"}
tagit! {BaseTag, "base"}
tagit! {BdiTag, "bdi"}
tagit! {BdoTag, "bdo"}
tagit! {BlockquoteTag, "blockquote"}
tagit! {BodyTag, "body"}
tagit! {BrTag, "br"}
tagit! {ButtonTag, "button"}
tagit! {CanvasTag, "canvas"}
tagit! {CaptionTag, "caption"}
tagit! {CiteTag, "cite"}
tagit! {CodeTag, "code"}
tagit! {ColTag, "col"}
tagit! {ColgroupTag, "colgroup"}
tagit! {DataTag, "data"}
tagit! {DatalistTag, "datalist"}
tagit! {DdTag, "dd"}
tagit! {DelTag, "del"}
tagit! {DetailsTag, "details"}
tagit! {DfnTag, "dfn"}
tagit! {DialogTag, "dialog"}
tagit! {DivTag, "div"}
tagit! {DlTag, "dl"}
tagit! {DtTag, "dt"}
tagit! {EmTag, "em"}
tagit! {EmbedTag, "embed"}
tagit! {FieldsetTag, "fieldset"}
tagit! {FigcaptionTag, "figcaption"}
tagit! {FigureTag, "figure"}
tagit! {FooterTag, "footer"}
tagit! {FormTag, "form"}
tagit! {H1Tag, "h1"}
tagit! {H2Tag, "h2"}
tagit! {H3Tag, "h3"}
tagit! {H4Tag, "h4"}
tagit! {H5Tag, "h5"}
tagit! {H6Tag, "h6"}
tagit! {HeadTag, "head"}
tagit! {HeaderTag, "header"}
tagit! {HrTag, "hr"}
tagit! {HtmlTag, "html"}
tagit! {ITag, "i"}
tagit! {IframeTag, "iframe"}
tagit! {ImgTag, "img"}
tagit! {InputTag, "input"}
tagit! {InsTag, "ins"}
tagit! {KbdTag, "kbd"}
tagit! {LabelTag, "label"}
tagit! {LegendTag, "legend"}
tagit! {LiTag, "li"}
tagit! {LinkTag, "link"}
tagit! {MainTag, "main"}
tagit! {MapTag, "map"}
tagit! {MarkTag, "mark"}
tagit! {MenuTag, "menu"}
tagit! {MenuitemTag, "menuitem"}
tagit! {MetaTag, "meta"}
tagit! {MeterTag, "meter"}
tagit! {NavTag, "nav"}
tagit! {NoscriptTag, "noscript"}
tagit! {OlTag, "ol"}
tagit! {OptgroupTag, "optgroup"}
tagit! {OptionTag, "option"}
tagit! {OutputTag, "output"}
tagit! {PTag, "p"}
tagit! {ParamTag, "param"}
tagit! {PictureTag, "picture"}
tagit! {PreTag, "pre"}
tagit! {ProgressTag, "progress"}
tagit! {QTag, "q"}
tagit! {RTag, "rp"}
tagit! {RtTag, "rt"}
tagit! {RubyTag, "ruby"}
tagit! {STag, "s"}
tagit! {SampTag, "samp"}
tagit! {ScriptTag, "script"}
tagit! {SectionTag, "section"}
tagit! {SelectTag, "select"}
tagit! {SmallTag, "small"}
tagit! {SourceTag, "source"}
tagit! {SpanTag, "span"}
tagit! {StrongTag, "strong"}
tagit! {StyleTag, "style"}
tagit! {SubTag, "sub"}
tagit! {SummaryTag, "summary"}
tagit! {SupTag, "sup"}
tagit! {TableTag, "table"}
tagit! {TbodyTag, "tbody"}
tagit! {TdTag, "td"}
tagit! {TemplateTag, "template"}
tagit! {TextareaTag, "textarea"}
tagit! {TfootTag, "tfoot"}
tagit! {ThTag, "th"}
tagit! {TheadTag, "thead"}
tagit! {TimeTag, "time"}
tagit! {TitleTag, "title"}
tagit! {TrTag, "tr"}
tagit! {TrackTag, "track"}
tagit! {UTag, "u"}
tagit! {UlTag, "ul"}
tagit! {VarTag, "var"}
tagit! {VideoTag, "video"}
tagit! {WbrTag, "wbr"}
