//! The markdown tables the portfolio check reads, and the cell forms in them.

/// One markdown table: the requested columns of each body row.
pub(super) struct Table {
    pub(super) rows: Vec<Row>,
}

/// A table row, its cells reordered to the columns [`read_table`] was asked for.
pub(super) struct Row {
    /// One-based line in the specification.
    pub(super) line: usize,
    cells: Vec<String>,
}

impl Row {
    /// The `i`th requested column, or empty when the row is short.
    pub(super) fn cell(&self, i: usize) -> &str {
        self.cells.get(i).map_or("", String::as_str)
    }
}

/// The first table after the heading line starting with `heading`, projected
/// onto `columns` by header name. `None` when the heading, the table or any
/// column is missing — a table reached past another heading is not this one.
pub(super) fn read_table(lines: &[&str], heading: &str, columns: &[&str]) -> Option<Table> {
    let start = lines.iter().position(|l| l.starts_with(heading))? + 1;
    let (header_at, header) = lines
        .iter()
        .enumerate()
        .skip(start)
        .take_while(|(_, l)| !l.starts_with('#'))
        .find(|(_, l)| l.starts_with('|'))?;
    let header = cells(header);
    let picks: Vec<usize> = columns
        .iter()
        .map(|c| header.iter().position(|h| h == c))
        .collect::<Option<_>>()?;
    let rows = lines
        .iter()
        .enumerate()
        .skip(header_at + 1)
        .take_while(|(_, l)| l.starts_with('|'))
        .filter(|(_, l)| !is_separator(l))
        .map(|(i, l)| {
            let mut all = cells(l);
            // A short row's missing cell reads as empty, which `Row::cell`
            // documents; every check then reports it as the wrong value.
            let cells = picks
                .iter()
                .map(|&p| all.get_mut(p).map(std::mem::take).unwrap_or_default())
                .collect();
            Row { line: i + 1, cells }
        })
        .collect();
    Some(Table { rows })
}

/// A row's cells, trimmed. A `|` inside a code span is text, not a border.
fn cells(line: &str) -> Vec<String> {
    let inner = line.trim().trim_start_matches('|');
    let inner = inner.strip_suffix('|').unwrap_or(inner);
    let mut out = Vec::new();
    let mut current = String::new();
    let mut in_code = false;
    for ch in inner.chars() {
        match ch {
            '`' => {
                in_code = !in_code;
                current.push(ch);
            }
            '|' if !in_code => out.push(std::mem::take(&mut current).trim().to_owned()),
            _ => current.push(ch),
        }
    }
    out.push(current.trim().to_owned());
    out
}

/// `|---|:--:|` and the like.
fn is_separator(line: &str) -> bool {
    line.chars().all(|c| matches!(c, '|' | '-' | ':' | ' '))
}

/// The text of a leading `**…**` run, if the cell opens with one.
pub(super) fn bold_lead(cell: &str) -> Option<&str> {
    let rest = cell.strip_prefix("**")?;
    rest.split_once("**").map(|(lead, _)| lead.trim())
}

/// An axis or port name as a cell writes it: `**Batch shape** (`…`)` is
/// `Batch shape`, and `` **`EventStore`** `` is `EventStore`.
pub(super) fn name_in(cell: &str) -> &str {
    let cell = cell.trim();
    bold_lead(cell)
        .unwrap_or(cell)
        .trim_matches(['*', '`'])
        .trim()
}
