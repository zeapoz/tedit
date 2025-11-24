use crate::editor::ui::{frame::Cell, style::Style};

// TODO: Implement a container struct to manager children positions withhin the container. It
// should hold a type T: IntoCells.

/// A line of text containing multiple secitons.
#[derive(Debug, Clone)]
pub struct Line<'a> {
    /// The number of columns in the line.
    pub width: usize,
    /// The sections of the line.
    pub sections: Vec<Section<'a>>,
    /// The style of the line. This is sets the default style for all spans. Spans with their own
    /// style will override this.
    pub style: Style,
}

impl<'a> Line<'a> {
    pub fn new(width: usize) -> Self {
        Self {
            width,
            sections: Vec::new(),
            style: Style::default(),
        }
    }

    /// Sets the style of the line.
    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Adds a new section to the line.
    pub fn with_section(mut self, section: Section<'a>) -> Self {
        self.sections.push(section);
        self
    }

    /// Returns the line as a vector of cells.
    pub fn as_cells(&mut self) -> Vec<Cell> {
        self.compute_section_widths();
        self.sections
            .iter()
            .flat_map(|s| s.as_cells())
            .map(|mut c| {
                c.style.apply(self.style);
                c
            })
            .collect()
    }

    /// Computes the widths of the sections based on the number of sections and the total width.
    fn compute_section_widths(&mut self) {
        let n = self.sections.len();
        let base = self.width / n;
        let extra = self.width % n;
        for (i, section) in self.sections.iter_mut().enumerate() {
            section.width = base + usize::from(i < extra);
        }
    }
}

/// The alignment strategy of a section.
#[derive(Debug, Default, Clone, Copy)]
pub enum Alignment {
    #[default]
    Left,
    Right,
    Center,
}

/// A section of spans..
#[derive(Debug, Clone)]
pub struct Section<'a> {
    /// The width of the section. Should be set by the container.
    pub width: usize,
    /// The spans of the section.
    pub spans: Vec<Span<'a>>,
    /// The style of the section. This is sets the default style for all spans.
    pub style: Style,
    /// The alignment of the section inside its container.
    pub alignment: Alignment,
}

impl<'a> Section<'a> {
    pub fn new(spans: Vec<Span<'a>>) -> Self {
        Self {
            width: 0,
            spans,
            style: Style::default(),
            alignment: Alignment::default(),
        }
    }

    /// Sets the style of the section.
    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Sets the alignment of the section.
    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Inserts a given separator between each span.
    pub fn with_separator(mut self, separator: &'a str) -> Self {
        if self.spans.is_empty() {
            return self;
        }

        let mut new_spans = Vec::new();

        let mut span_iter = self.spans.iter_mut();
        let prev_span = span_iter.next().expect("spans was unexpectedly empty");
        new_spans.push(*prev_span);

        for span in span_iter {
            let searator_span = Span::new(separator).with_style(self.style);
            new_spans.push(searator_span);
            new_spans.push(*span);
        }

        self.spans = new_spans;
        self
    }

    /// Inserts a whitespace separator between each span.
    pub fn with_whitespace_separator(self) -> Self {
        self.with_separator(" ")
    }

    /// Returns the line as a vector of cells padded and aligned to `self.width`.
    pub fn as_cells(&self) -> Vec<Cell> {
        let content: Vec<Cell> = self
            .spans
            .as_slice()
            .iter()
            .flat_map(|&(mut s)| {
                s.style.apply(self.style);
                s.as_cells()
            })
            .collect();
        let pad_cell = Cell::default().with_style(self.style);
        pad_and_align_content(content, self.width, self.alignment, pad_cell)
    }
}

/// A string with a particular style.
#[derive(Debug, Clone, Copy)]
pub struct Span<'a> {
    /// The width of the span.
    pub width: usize,
    /// The string of the span.
    pub str: &'a str,
    /// The style of the span.
    pub style: Style,
    /// The alignment of the span.
    pub alignment: Alignment,
}

impl<'a> Span<'a> {
    pub fn new(str: &'a str) -> Self {
        Self {
            width: str.len(),
            str,
            style: Style::default(),
            alignment: Alignment::default(),
        }
    }

    /// Sets the width of the span used for padding and alignment.
    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    /// Sets the style of the span.
    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Sets the alignment of the span.
    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Returns the span as a vector of cells.
    pub fn as_cells(&self) -> Vec<Cell> {
        let content: Vec<_> = self
            .str
            .chars()
            .map(|c| Cell::new(c).with_style(self.style))
            .collect();
        let pad_cell = Cell::default().with_style(self.style);
        pad_and_align_content(content, self.width, self.alignment, pad_cell)
    }
}

/// Returns the given content padded with `pad_cell` and aligned to the given width and alignment.
fn pad_and_align_content(
    content: Vec<Cell>,
    width: usize,
    alignment: Alignment,
    pad_cell: Cell,
) -> Vec<Cell> {
    let padding = width - content.len();
    match alignment {
        Alignment::Left => {
            let mut left = content;
            left.extend(std::iter::repeat_n(pad_cell, padding));
            left
        }
        Alignment::Right => {
            let mut right = (0..padding).map(|_| pad_cell).collect::<Vec<_>>();
            right.extend(content);
            right
        }
        Alignment::Center => {
            let left = padding / 2;
            let right = padding - left;

            let mut center = Vec::with_capacity(width);
            center.extend(std::iter::repeat_n(pad_cell, left));
            center.extend(content);
            center.extend(std::iter::repeat_n(pad_cell, right));
            center
        }
    }
}
