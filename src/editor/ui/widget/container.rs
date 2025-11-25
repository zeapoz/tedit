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
#[derive(Default)]
pub struct Container {
    /// The width of the container. If `None`, the container will be flexible.
    pub width: Option<usize>,
    /// The children of the container.
    pub children: Vec<Box<dyn Widget + 'static>>,
    /// The style of the container.
    pub style: Style,
    /// How the container aligns it's children.
    pub alignment: Alignment,
}

impl Container {
    /// Adds a new child to the container.
    pub fn with_child(mut self, child: impl Widget + 'static) -> Self {
        self.children.push(Box::new(child));
        self
    }

    /// Adds multiple children to the container.
    pub fn with_children(
        mut self,
        children: impl IntoIterator<Item = Box<dyn Widget + 'static>>,
    ) -> Self {
        self.children.extend(children);
        self
    }

    /// Sets the alignment of the container.
    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Sets the width of the container. If `None`, the container will be flexible.
    pub fn with_width(mut self, width: Option<usize>) -> Self {
        self.width = width;
        self
    }

    /// Sets the style of the container.
    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
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

impl Widget for Container {
    fn as_cells(&mut self) -> Vec<Cell> {
        let child_widths = self.calculate_child_widths();
        let mut cells = Vec::new();
        for (i, child) in self.children.iter_mut().enumerate() {
            let width = child_widths[i].or_else(|| Some(child.width()));
            child.set_width(width);
            child.set_style(self.style);
            cells.extend(child.as_cells());
        }

        let pad_cell = Cell::default().with_style(self.style);
        let width = self.width.unwrap_or(cells.len());
        if width == 0 {
            return Vec::new();
        }

        match self.alignment {
            Alignment::Left => {
                let padding = width.saturating_sub(cells.len());
                let mut out = Vec::with_capacity(width);
                out.extend(cells);
                out.extend(std::iter::repeat(pad_cell).take(padding));
                out.truncate(width);
                out
            }
            Alignment::Right => {
                let padding = width.saturating_sub(cells.len());
                let mut out = Vec::with_capacity(width);
                out.extend(std::iter::repeat(pad_cell).take(padding));
                out.extend(cells);
                out.truncate(width);
                out
            }
            Alignment::Center => {
                let padding = width.saturating_sub(cells.len());
                let left = padding / 2;
                let right = padding - left;

                let mut out = Vec::with_capacity(width);
                out.extend(std::iter::repeat(pad_cell.clone()).take(left));
                out.extend(cells);
                out.extend(std::iter::repeat(pad_cell).take(right));
                out.truncate(width);
                out
            }
            Alignment::SpaceEvenly => cells,
        }
    }

    fn width(&self) -> usize {
        self.children.iter().map(|child| child.width()).sum()
    }

    fn set_width(&mut self, width: Option<usize>) {
        self.width = width;
    }

    fn set_style(&mut self, style: Style) {
        self.style.apply(style);
    }
}
