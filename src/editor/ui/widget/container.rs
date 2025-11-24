use crate::editor::ui::{frame::Cell, style::Style, widget::Widget};

/// The alignment strategy of a container.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    /// Aligns all children to the left.
    #[default]
    Left,
    /// Aligns all children to the right.
    Right,
    /// Aligns all children in the middle.
    Center,
    /// Aligns all children spaced evenly.
    SpaceEvenly,
}

/// A container that can hold objects.
#[derive(Debug, Default, Clone)]
pub struct Container<T: Widget> {
    /// The width of the container. If `None`, the container will be flexible.
    pub width: Option<usize>,
    // TODO: Make this `Vec<Box<dyn Widget>>
    /// The children of the container.
    pub children: Vec<T>,
    /// The style of the container.
    pub style: Style,
    /// How the container aligns it's children.
    pub alignment: Alignment,
}

impl<T: Widget> Container<T> {
    pub fn new(children: Vec<T>) -> Self {
        Self {
            width: None,
            children,
            style: Style::default(),
            alignment: Alignment::default(),
        }
    }

    /// Adds a new child to the container.
    pub fn with_child(mut self, child: T) -> Self {
        self.children.push(child);
        self
    }

    /// Sets the alignment of the containeer.
    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Calculates and returns the widths of each child. This only returns `Some` if alignment is set to
    /// [`Alignment::SpaceEvenly`] and the container has children.
    fn calculate_child_widths(&self) -> Vec<Option<usize>> {
        let n = self.children.len();
        if self.alignment == Alignment::SpaceEvenly {
            if n == 0 {
                return vec![None; n];
            }

            if let Some(width) = self.width {
                let slot_base = width / n;
                let extra = width % n;
                (0..n)
                    .map(|i| Some(slot_base + usize::from(i < extra)))
                    .collect()
            } else {
                return vec![None; n];
            }
        } else {
            return vec![None; n];
        }
    }
}

impl<T: Widget> Widget for Container<T> {
    fn into_cells(self) -> Vec<Cell> {
        let child_widths = self.calculate_child_widths();
        let content: Vec<_> = self
            .children
            .into_iter()
            .zip(child_widths)
            .flat_map(|(child, width)| {
                let width = width.or_else(|| Some(child.width()));
                child.with_width(width).with_style(self.style).into_cells()
            })
            .collect();

        let pad_cell = Cell::default().with_style(self.style);
        let width = self.width.unwrap_or(content.len());
        if width == 0 {
            return Vec::new();
        }

        match self.alignment {
            Alignment::Left => {
                let padding = width.saturating_sub(content.len());
                let mut out = Vec::with_capacity(width);
                out.extend(content);
                out.extend(std::iter::repeat(pad_cell).take(padding));
                out.truncate(width);
                out
            }
            Alignment::Right => {
                let padding = width.saturating_sub(content.len());
                let mut out = Vec::with_capacity(width);
                out.extend(std::iter::repeat(pad_cell).take(padding));
                out.extend(content);
                out.truncate(width);
                out
            }
            Alignment::Center => {
                let padding = width.saturating_sub(content.len());
                let left = padding / 2;
                let right = padding - left;

                let mut out = Vec::with_capacity(width);
                out.extend(std::iter::repeat(pad_cell.clone()).take(left));
                out.extend(content);
                out.extend(std::iter::repeat(pad_cell).take(right));
                out.truncate(width);
                out
            }
            Alignment::SpaceEvenly => content,
        }
    }

    fn width(&self) -> usize {
        self.children.iter().map(|child| child.width()).sum()
    }

    fn with_width(mut self, width: Option<usize>) -> Self {
        self.width = width;
        self
    }

    fn with_style(mut self, style: Style) -> Self {
        self.style.apply(style);
        self
    }
}
