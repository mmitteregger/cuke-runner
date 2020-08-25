use gherkin::cuke;
use indexmap::IndexMap;

use super::*;

/// `Iterator` implementation that converts cuke `Row`s into `DataTableCellIter`
/// to finally get all cells as `&str`.
#[derive(Debug)]
pub(crate) struct DataTableIter<'dt> {
    /// Iterator over all data table rows with their cells.
    row_iter: std::slice::Iter<'dt, cuke::Row<'dt>>,
}

impl<'dt> DataTableIter<'dt> {
    pub(crate) fn new(data_table: &'dt DataTable<'_>) -> DataTableIter<'dt> {
        DataTableIter {
            row_iter: data_table.cuke_table.rows.iter(),
        }
    }
}

impl<'dt> Iterator for DataTableIter<'dt> {
    type Item = DataTableCellIter<'dt>;

    fn next(&mut self) -> Option<Self::Item> {
        self.row_iter.next()
            .map(|row| DataTableCellIter::new(row))
    }
}

/// `Iterator` implementation that converts cuke `Cell`s into `&str`.
#[derive(Debug)]
pub(crate) struct DataTableCellIter<'dt> {
    /// Iterator over all data table cells in a row.
    cell_iter: std::slice::Iter<'dt, cuke::Cell<'dt>>,
}

impl<'dt> DataTableCellIter<'dt> {
    pub(crate) fn new(row: &'dt cuke::Row<'_>) -> DataTableCellIter<'dt> {
        DataTableCellIter {
            cell_iter: row.cells.iter(),
        }
    }
}

impl<'dt> Iterator for DataTableCellIter<'dt> {
    type Item = &'dt str;

    fn next(&mut self) -> Option<Self::Item> {
        self.cell_iter.next()
            .map(|cell| cell.value.as_ref())
    }
}

/// `Iterator` implementation that automatically converts all [`DataTable`] rows
/// using [`FromDataTableRow`] to a custom type.
///
/// [`DataTable`]: ./struct.DataTable.html
/// [`FromDataTableRow`]: ./trait.FromDataTableRow.html
#[derive(Debug)]
pub struct FromDataTableRowIter<'dt, T: FromDataTableRow<'dt>> {
    /// Iterator over all data table rows.
    rows_iter: std::slice::Iter<'dt, cuke::Row<'dt>>,
    /// User supplied result type for a single row.
    result: std::marker::PhantomData<T>,
}

impl<'dt, T: FromDataTableRow<'dt>> FromDataTableRowIter<'dt, T> {
    pub(crate) fn new(data_table: &'dt DataTable<'_>) -> FromDataTableRowIter<'dt, T> {
        FromDataTableRowIter {
            rows_iter: data_table.cuke_table.rows.iter(),
            result: std::marker::PhantomData,
        }
    }
}

impl<'dt, T: FromDataTableRow<'dt>> Iterator for FromDataTableRowIter<'dt, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.rows_iter
            .next()
            .map(|row| {
                let cells = row.cells
                    .iter()
                    .map(|cell| cell.value.as_ref())
                    .collect::<Vec<&str>>();

                let row_ref = RowRef::new(&cells);
                FromDataTableRow::from(row_ref)
            })
    }
}

/// `Iterator` implementation that automatically converts all [`DataTable`] **body** rows
/// using [`FromDataTableBodyRow`] to a custom type.
///
/// [`DataTable`]: ./struct.DataTable.html
/// [`FromDataTableBodyRow`]: ./trait.FromDataTableBodyRow.html
#[derive(Debug)]
pub struct FromDataTableBodyRowIter<'dt, T: FromDataTableBodyRow<'dt>> {
    /// Header name to body value mapping to reuse a single allocation.
    cells: IndexMap<&'dt str, &'dt str>,
    /// Iterator over all body rows (all data table rows except the first one).
    body_rows_iter: std::slice::Iter<'dt, cuke::Row<'dt>>,
    /// User supplied result type for a single row.
    phantom: std::marker::PhantomData<T>,
}

impl<'dt, T: FromDataTableBodyRow<'dt>> FromDataTableBodyRowIter<'dt, T> {
    pub(crate) fn new(data_table: &'dt DataTable<'_>) -> FromDataTableBodyRowIter<'dt, T> {
        let mut iter = data_table.cuke_table.rows.iter();

        let mut cells = IndexMap::with_capacity(data_table.columns_len());

        match iter.next() {
            Some(header_row) => {
                header_row.cells
                    .iter()
                    .map(|header_cell| header_cell.as_ref())
                    .for_each(|header| {
                        cells.insert(header, "");
                    });
            },
            None => panic!("Cannot extract headers from empty data table"),
        }

        FromDataTableBodyRowIter {
            cells,
            body_rows_iter: iter,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'dt, T: FromDataTableBodyRow<'dt>> Iterator for FromDataTableBodyRowIter<'dt, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.body_rows_iter
            .next()
            .map(|row| {
                assert_eq!(self.cells.len(), row.cells.len(),
                    "Header row and body row should have the same length");

                self.cells
                    .iter_mut()
                    .enumerate()
                    .for_each(|(index, (_header, value))| {
                        *value = row.cells[index].value.as_ref();
                    });

                let row_ref = BodyRowRef::new(&self.cells);
                FromDataTableBodyRow::from(row_ref)
            })
    }
}
