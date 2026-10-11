//! The corpus itself: hand-written identifiers, each in the form its
//! orthography takes. Invisible characters are written as escapes so the file
//! can be reviewed; everything else is the literal text.
//!
//! Nothing here was chosen to pass. An entry refused by the shipped validator
//! is a finding, and the README forbids removing it.

use super::{Entry, Pair, Script};

const fn ar(text: &'static str, exercises: &'static str) -> Entry {
    Entry {
        script: Script::Arabic,
        text,
        exercises,
    }
}

const fn he(text: &'static str, exercises: &'static str) -> Entry {
    Entry {
        script: Script::Hebrew,
        text,
        exercises,
    }
}

const fn fa(text: &'static str, exercises: &'static str) -> Entry {
    Entry {
        script: Script::Persian,
        text,
        exercises,
    }
}

/// Every identifier the corpus check tests, as an event type and as a tag.
pub const CORPUS: &[Entry] = &[
    // Arabic.
    ar(
        "طلب_تم_إنشاؤه",
        "letters with hamza forms, ASCII underscore",
    ),
    ar("الطلب.تم_الشحن", "definite article, dotted namespace"),
    ar("مُحَمَّد", "harakat: damma, fatha, shadda (Mn)"),
    ar("مـحـمـد", "tatweel U+0640 (Lm)"),
    ar("فاتورة-٢٠٢٦-٠٤٢", "Arabic-Indic digits U+0660-U+0669"),
    ar("عميل:أحمد", "key:value, both halves Arabic"),
    ar("سؤال؟", "Arabic question mark U+061F"),
    ar("نسبة:٥٠٪", "Arabic percent sign U+066A"),
    ar("قيمة:٣٫١٤", "Arabic decimal separator U+066B"),
    ar("أ،ب،ج", "Arabic comma U+060C"),
    ar(
        "رصيد:\u{061C}-١٢٣",
        "Arabic Letter Mark U+061C (Cf) before a minus sign",
    ),
    ar(
        "\u{0600}١٢٣",
        "Arabic number sign U+0600 (Cf, prepended concatenation mark)",
    ),
    ar("آية\u{06DD}١٢", "Arabic end of ayah U+06DD (Cf)"),
    ar(
        "\u{FEFB}_يوجد",
        "presentation form: lam-alef ligature U+FEFB",
    ),
    ar("ﺍﻟﻌﺮﺑﻴﺔ", "presentation forms B, contextual shapes"),
    ar("customer:أحمد", "mixed: LTR key, RTL value"),
    ar("OrderPlaced.طلب", "mixed: LTR namespace, RTL leaf"),
    ar(
        "Invoice-2026/فاتورة-42",
        "mixed: ASCII digits either side of RTL",
    ),
    ar(
        "المنتج:\u{200E}iPhone-15",
        "mixed: LRM U+200E before an LTR value",
    ),
    ar("مستخدم:Ali\u{200F}", "mixed: RLM U+200F after an LTR value"),
    ar("course:مقدمة-في-Rust", "mixed: LTR inside an RTL value"),
    // Hebrew.
    he("הזמנה_נוצרה", "letters, ASCII underscore"),
    he("לקוח:דוד", "key:value, both halves Hebrew"),
    he("שָׁלוֹם", "niqqud: qamats, shin dot, holam (Mn)"),
    he("בית־ספר", "maqaf U+05BE"),
    he("צה״ל", "gershayim U+05F4"),
    he("ג׳ירפה", "geresh U+05F3"),
    he("מלך:שלום", "final kaf and final mem"),
    he("\u{FB2A}בת", "presentation form: shin with shin dot U+FB2A"),
    he("ﭏ", "presentation form: alef-lamed ligature U+FB4F"),
    he("ייִדיש", "hiriq under yod, Yiddish spelling"),
    he("customer:דוד", "mixed: LTR key, RTL value"),
    he("OrderShipped.משלוח", "mixed: LTR namespace, RTL leaf"),
    he("חשבונית-2026-042", "mixed: ASCII digits after RTL"),
    he("קורס:מבוא-ל-Rust", "mixed: LTR inside an RTL value"),
    he(
        "מוצר:\u{200E}iPhone-15",
        "mixed: LRM U+200E before an LTR value",
    ),
    he("משתמש:Dana\u{200F}", "mixed: RLM U+200F after an LTR value"),
    // Persian.
    fa("سفارش\u{200C}ها", "ZWNJ U+200C before the plural suffix"),
    fa("می\u{200C}خواهم", "ZWNJ U+200C after the verbal prefix"),
    fa("ثبت\u{200C}نام_شد", "ZWNJ U+200C inside a compound"),
    fa("کاربر:پژمان", "Persian letters keheh, peh, zheh"),
    fa("گزارش_۱۴۰۵", "Extended Arabic-Indic digits U+06F0-U+06F9"),
    fa("فاکتور-۱۴۰۵-۰۴۲", "Persian digits with hyphens"),
    fa("یادداشت\u{200C}های_روزانه", "farsi yeh U+06CC, ZWNJ"),
    fa(
        "ه\u{200D}.ش",
        "ZWJ U+200D forcing a joining form in an abbreviation",
    ),
    fa("customer:مهسا", "mixed: LTR key, RTL value"),
    fa(
        "Order.سفارش\u{200C}ها",
        "mixed: LTR namespace, RTL leaf with ZWNJ",
    ),
    fa("کد-2026-چ", "mixed: ASCII digits inside RTL"),
    fa(
        "کتاب\u{200C}خانه:Library-1",
        "mixed: RTL key with ZWNJ, LTR value",
    ),
    fa(
        "کاربر:\u{200F}زهرا",
        "RLM U+200F at the start of an RTL value",
    ),
    fa(
        "محصول:\u{200E}Galaxy-S25",
        "mixed: LRM U+200E before an LTR value",
    ),
];

/// `key:value` pairs, tested through `Tag::key_value`.
pub const PAIRS: &[Pair] = &[
    Pair {
        script: Script::Arabic,
        key: "customer",
        value: "أحمد",
    },
    Pair {
        script: Script::Arabic,
        key: "عميل",
        value: "Ahmed",
    },
    Pair {
        script: Script::Arabic,
        key: "فاتورة",
        value: "٢٠٢٦-٠٤٢",
    },
    Pair {
        script: Script::Arabic,
        key: "course",
        value: "مقدمة-في-Rust",
    },
    Pair {
        script: Script::Hebrew,
        key: "customer",
        value: "דוד",
    },
    Pair {
        script: Script::Hebrew,
        key: "לקוח",
        value: "Dana",
    },
    Pair {
        script: Script::Hebrew,
        key: "מוצר",
        value: "\u{200E}iPhone-15",
    },
    Pair {
        script: Script::Persian,
        key: "کاربر",
        value: "Pezhman",
    },
    Pair {
        script: Script::Persian,
        key: "customer",
        value: "سفارش\u{200C}ها",
    },
    Pair {
        script: Script::Persian,
        key: "کتاب\u{200C}خانه",
        value: "۱۴۰۵",
    },
];
