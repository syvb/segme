use unicode_segmentation::UnicodeSegmentation;
use wasm_bindgen::prelude::*;

pub fn entity_escape_char(khar: &char) -> String {
    format!("&#x{:X};", (*khar) as u32)
}

/// Escapes a string to allow it to be used in HTML as:
/// - an element value
/// - an attribute value
///
/// This works by replacing every character with its HTML entity, except for letters, numbers,
/// and spaces. This *does* mean that we sometimes "over-escape", but it's worth it for more
/// security.
pub fn html_escape(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    for khar in text.chars() {
        match khar {
            c @ '0'..='9' => result.push(c),
            c @ 'A'..='z' => result.push(c),
            ' ' => result.push(' '),
            c => result.push_str(&entity_escape_char(&c)),
        }
    }
    result
}

#[wasm_bindgen]
pub fn segment_table(t: String) -> String {
    let mut rows = String::from("<tr><th>Graphemes</th>");
    for grapheme in t.graphemes(true) {
        rows.push_str(&format!(
            r#"<td colspan="{}"><span class="char">{}</span></td>"#,
            grapheme.len(),
            grapheme
        ));
    }
    rows.push_str("</tr><tr><th>Characters</th>");
    for khar in t.chars() {
        rows.push_str(&format!(
            r#"<td colspan="{}"><span class="char">{}</span><div class="name">{}</div></td>"#,
            khar.len_utf8(),
            khar,
            if let Some(x) = unicode_names2::name(khar) {
                format!("{}", x)
            } else {
                String::from("?")
            }
        ));
    }
    rows.push_str(r#"</tr><tr class="bytes-row"><th>Bytes</th>"#);
    for byte in t.bytes() {
        rows.push_str(&format!(r#"<td>{:x}</td>"#, byte));
    }
    rows.push_str("</tr>");
    format!(r#"<table class="segmented">{}</table>"#, rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segment_table_works() {
        assert_eq!(segment_table("👨‍👩‍👦".to_string()), "<table class=\"segmented\"><tr><th>Graphemes</th><td colspan=\"18\">👨\u{200d}👩\u{200d}👦</tr></tr><tr><th>Characters</th><td colspan=\"4\">👨</tr><td colspan=\"3\">\u{200d}</tr><td colspan=\"4\">👩</tr><td colspan=\"3\">\u{200d}</tr><td colspan=\"4\">👦</tr></tr><tr><th>Bytes</th><td>f0</tr><td>9f</tr><td>91</tr><td>a8</tr><td>e2</tr><td>80</tr><td>8d</tr><td>f0</tr><td>9f</tr><td>91</tr><td>a9</tr><td>e2</tr><td>80</tr><td>8d</tr><td>f0</tr><td>9f</tr><td>91</tr><td>a6</tr></tr></table>".to_string());
    }
}
