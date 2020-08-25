use indexmap::IndexMap;

/// Reference to a data table row with its cells.
///
/// This struct is used in [`FromDataTableRow`],
/// which is called for **every row** in the table.
///
/// The lifetime parameter `'r` refers to the lifetime of the row in the [`Iterator`].
/// The lifetime parameter `'dt` refers to the lifetime of the DataTable.
/// This struct cannot escape the step function.
///
/// [`FromDataTableRow`]: ./trait.FromDataTableRow.html
/// [`Iterator`]: ./struct.FromDataTableRowIter.html
#[derive(Debug)]
pub struct RowRef<'r, 'dt> {
    cells: &'r [&'dt str],
}

impl<'r, 'dt: 'r> RowRef<'r, 'dt> {
    pub(crate) fn new(cells: &'r [&'dt str]) -> RowRef<'r, 'dt> {
        RowRef { cells }
    }

    /// Returns the number of cells in this row.
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    /// Returns the column value at a specific index.
    ///
    /// For a panicking variant the [`Index`] trait is also implemented for this type.
    ///
    /// [`Index`]: ./struct.RowRef.html#impl-Index<usize>
    pub fn value_at_index(&self, index: usize) -> Option<&'r str> {
        self.cells.get(index)
            .map(|value| *value)
    }

    /// Returns an `Iterator` over all columns in this row.
    pub fn iter(&'r self) -> impl Iterator<Item=&'r str> {
        self.cells.iter()
            .map(|value| *value)
    }

    /// Detaches the data in this row.
    ///
    /// This needs to allocate every column in this row, use it only as last resort.
    pub fn to_owned(&self) -> Row {
        Row::new(self)
    }
}

impl<'r, 'dt: 'r> std::ops::Index<usize> for RowRef<'r, 'dt> {
    type Output = &'dt str;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cells[index]
    }
}

/// Data table row with its cells.
///
/// This is the owned variant of [`RowRef`] and thus can escape the step function.
/// Due to the additional allocations to detach from the [`DataTable`]
/// it is advised to use it only as last resort.
///
/// [`RowRef`]: ./struct.RowRef.html
/// [`DataTable`]: ./struct.DataTable.html
#[derive(Debug)]
pub struct Row {
    cells: Vec<String>,
}

impl<'r, 'dt: 'r> Row {
    fn new(row: &RowRef<'r, 'dt>) -> Row {
        let cells = row.cells.iter()
            .map(|cell| cell.to_string())
            .collect();

        Row {
            cells,
        }
    }
}

impl Row {
    /// Returns the number of cells in this row.
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    /// Returns the cell value at a specific index.
    ///
    /// For a panicking variant the [`Index`] trait is also implemented for this type.
    ///
    /// [`Index`]: ./struct.Row.html#impl-Index<usize>
    pub fn value_at_index(&self, index: usize) -> Option<&str> {
        self.cells.get(index)
            .map(|value| value.as_str())
    }

    /// Returns an `Iterator` over all columns in this row.
    pub fn iter(&self) -> impl Iterator<Item=&str> {
        self.cells.iter()
            .map(|value| value.as_str())
    }
}

impl std::ops::Index<usize> for Row {
    type Output = str;

    fn index(&self, index: usize) -> &Self::Output {
        self.cells[index].as_str()
    }
}

/// Reference to a data table row with its cells and their corresponding headers from the first row.
///
/// This struct is used in [`FromDataTableBodyRow`],
/// which is called for every row in the table **after the first one** .
///
/// The lifetime parameter `'r` refers to the lifetime of the row in the [`Iterator`].
/// The lifetime parameter `'dt` refers to the lifetime of the DataTable.
/// This struct cannot escape the step function.
///
/// [`FromDataTableBodyRow`]: ./trait.FromDataTableBodyRow.html
/// [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
#[derive(Debug)]
pub struct BodyRowRef<'r, 'dt> {
    cells: &'r IndexMap<&'dt str, &'dt str>,
}

impl<'r, 'dt: 'r> BodyRowRef<'r, 'dt> {
    pub(crate) fn new(cells: &'r IndexMap<&'dt str, &'dt str>) -> BodyRowRef<'r, 'dt> {
        BodyRowRef { cells }
    }

    /// Returns the number of cells in this row.
    ///
    /// Note that the length of the referenced cells in this row
    /// and the number of headers in the table is the same,
    /// so this function can be used to check both.
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    /// Returns a tuple with the header and value at the specified index.
    pub fn header_and_value_at_index(&self, index: usize) -> Option<(&'dt str, &'dt str)> {
        self.cells.get_index(index)
            .map(|(header, value)| (*header, *value))
    }

    /// Returns the header at the specified index.
    pub fn header_at_index(&self, index: usize) -> Option<&'dt str> {
        self.cells.get_index(index)
            .map(|(header, _value)| *header)
    }

    /// Returns the cell value at a specific index.
    ///
    /// For a panicking variant the [`Index`] trait is also implemented for this type.
    ///
    /// [`Index`]: ./struct.BodyRowRef.html#impl-Index<usize>
    pub fn value_at_index(&self, index: usize) -> Option<&'dt str> {
        self.cells.get_index(index)
            .map(|(_header, value)| *value)
    }

    /// Returns the cell value from the same column as the given header name.
    ///
    /// For a panicking variant the [`Index`] trait is also implemented for this type.
    ///
    /// [`Index`]: ./struct.BodyRowRef.html#impl-Index%3C%26%27_%20str%3E
    pub fn value(&self, header: &str) -> Option<&'dt str> {
        self.cells.get(header).map(|value| *value)
    }

    /// Checks whether the table contains a header with the given name.
    pub fn contains_header(&self, header: &str) -> bool {
        self.cells.contains_key(header)
    }

    /// Returns an `Iterator` over all columns in this row.
    pub fn iter(&'dt self) -> impl Iterator<Item=(&'dt str, &'dt str)> {
        self.cells.iter()
            .map(|(header, value)| (*header, *value))
    }

    /// Detaches the data in this row.
    ///
    /// This needs to allocate every header column and every column in this row,
    /// use it only as last resort.
    pub fn to_owned(&self) -> BodyRow {
        BodyRow::new(self)
    }
}

impl<'r, 'dt: 'r> std::ops::Index<&str> for BodyRowRef<'r, 'dt> {
    type Output = &'dt str;

    fn index(&self, index: &str) -> &Self::Output {
        self.cells.get(index)
            .unwrap()
    }
}

impl<'r, 'dt: 'r> std::ops::Index<usize> for BodyRowRef<'r, 'dt> {
    type Output = &'dt str;

    fn index(&self, index: usize) -> &Self::Output {
        self.cells.get_index(index)
            .map(|(_header, value)| value)
            .unwrap()
    }
}

/// Data table row with its cells and their corresponding headers from the first row.
///
/// This is the owned variant of [`BodyRowRef`] and thus can escape the step function.
/// Due to the additional allocations to detach from the [`DataTable`]
/// it is advised to use it only as last resort.
///
/// [`BodyRowRef`]: ./struct.BodyRowRef.html
/// [`DataTable`]: ./struct.DataTable.html
#[derive(Debug)]
pub struct BodyRow {
    cells: IndexMap<String, String>,
}

impl<'r, 'dt: 'r> BodyRow {
    fn new(body_row: &BodyRowRef<'r, 'dt>) -> BodyRow {
        let cells = body_row.cells.iter()
            .map(|(header, value)| (header.to_string(), value.to_string()))
            .collect::<IndexMap<String, String>>();

        BodyRow {
            cells,
        }
    }
}

impl BodyRow {
    /// Returns the number of cells in this row.
    ///
    /// Note that the length of the referenced cells in this row
    /// and the number of headers in the table is the same,
    /// so this function can be used to check both.
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    /// Returns a tuple with the header and value at the specified index.
    pub fn header_and_value_at_index(&self, index: usize) -> Option<(&str, &str)> {
        self.cells.get_index(index)
            .map(|(header, value)| (header.as_str(), value.as_str()))
    }

    /// Returns the header at the specified index.
    pub fn header_at_index(&self, index: usize) -> Option<&str> {
        self.cells.get_index(index)
            .map(|(header, _value)| header.as_str())
    }

    /// Returns the cell value at a specific index.
    ///
    /// For a panicking variant the [`Index`] trait is also implemented for this type.
    ///
    /// [`Index`]: ./struct.BodyRow.html#impl-Index<usize>
    pub fn value_at_index(&self, index: usize) -> Option<&str> {
        self.cells.get_index(index)
            .map(|(_header, value)| value.as_str())
    }

    /// Returns the cell value from the same column as the given header name.
    ///
    /// For a panicking variant the [`Index`] trait is also implemented for this type.
    ///
    /// [`Index`]: ./struct.BodyRow.html#impl-Index%3C%26%27_%20str%3E
    pub fn value(&self, header: &str) -> Option<&str> {
        self.cells.get(header).map(|value| value.as_str())
    }

    /// Checks whether the table contains a header with the given name.
    pub fn contains_header(&self, header: &str) -> bool {
        self.cells.contains_key(header)
    }

    /// Returns an `Iterator` over all columns in this row.
    pub fn iter(&self) -> impl Iterator<Item=(&str, &str)> {
        self.cells.iter()
            .map(|(header, value)| (header.as_str(), value.as_str()))
    }
}

impl std::ops::Index<&str> for BodyRow {
    type Output = str;

    fn index(&self, index: &str) -> &Self::Output {
        self.cells.get(index)
            .unwrap()
    }
}

impl std::ops::Index<usize> for BodyRow {
    type Output = str;

    fn index(&self, index: usize) -> &Self::Output {
        self.cells.get_index(index)
            .map(|(_header, value)| value.as_str())
            .unwrap()
    }
}
