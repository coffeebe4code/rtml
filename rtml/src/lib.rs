pub mod attributes;
pub mod macros;
pub mod tags;

use attributes::Attribute;
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
attributeit! {accesskey, "accesskey"}
attributeit! {class, "class"}
attributeit! {contenteditable, "contenteditable"}
attributeit! {dir, "dir"}
attributeit! {draggable, "draggable"}
attributeit! {hidden, "hidden"}
attributeit! {id, "id"}
attributeit! {lang, "lang"}
attributeit! {spellcheck, "spellcheck"}
attributeit! {style, "style"}
attributeit! {tabindex, "tabindex"}
attributeit! {title, "title"}
attributeit! {translate, "translate"}

// Specific Attributes
attributeit! {href, "href"}

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
tagit! {ObjectTag, "object"}
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
