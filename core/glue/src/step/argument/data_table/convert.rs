use super::{Row, RowRef, BodyRow, BodyRowRef};

/// Converts a row of the [`DataTable`] to `Self`.
///
/// This function will **also** be called for the first row in the data table.
/// If you want to ignore the first row or want to extract values based on the first (header) row,
/// use [`FromDataTableBodyRow`] with the [`body_rows`] function instead.
///
/// The lifetime parameter `'dt` refers to the lifetime of the DataTable.
/// It cannot escape the step function.
///
/// [`DataTable`]: ./struct.DataTable.html
/// [`FromDataTableBodyRow`]: ./trait.FromDataTableBodyRow.html
/// [`body_rows`]: ./struct.DataTable.html#method.body_rows
pub trait FromDataTableRow<'dt>: Sized {
    /// Performs the conversion of a single table row reference to `Self`.
    ///
    /// The first anonymous lifetime parameter `'_` refers to the lifetime of the row reference.
    /// It cannot escape this conversation function.
    ///
    /// The lifetime parameter `'dt` refers to the lifetime of the DataTable.
    /// It cannot escape the step function.
    fn from(row: RowRef<'_, 'dt>) -> Self;
}

/// Converts a body row of the [`DataTable`] to `Self`.
///
/// This function will **not** be called for the first (header) row in the data table,
/// but will provide those details in the [`BodyRowRef`] struct.
/// If you want to map **all** rows use either [`FromDataTableRow`] with the [`rows`] function
/// or if this is not sufficient the lower level [`iter`] function.
///
/// The lifetime parameter `'dt` refers to the lifetime of the DataTable.
/// It cannot escape the step function.
///
/// [`DataTable`]: ./struct.DataTable.html
/// [`BodyRowRef`]: ./struct.BodyRowRef.html
/// [`FromDataTableRow`]: ./trait.FromDataTableRow.html
/// [`rows`]: ./struct.DataTable.html#method.rows
/// [`iter`]: ./struct.DataTable.html#method.iter
pub trait FromDataTableBodyRow<'dt>: Sized {
    /// Performs the conversion of a single table body row reference to `Self`.
    ///
    /// The first anonymous lifetime parameter `'_` refers to the lifetime of the row reference.
    /// It cannot escape this conversation function.
    ///
    /// The lifetime parameter `'dt` refers to the lifetime of the DataTable.
    /// It cannot escape the step function.
    fn from(body_row: BodyRowRef<'_, 'dt>) -> Self;
}


impl<'r, 'dt: 'r> FromDataTableRow<'dt> for Row {
    fn from(row: RowRef<'_, 'dt>) -> Self {
        row.to_owned()
    }
}

impl<'r, 'dt: 'r> FromDataTableBodyRow<'dt> for BodyRow {
    fn from(body_row: BodyRowRef<'_, 'dt>) -> Self {
        body_row.to_owned()
    }
}
