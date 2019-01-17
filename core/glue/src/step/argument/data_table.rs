use gherkin::pickle::PickleTable;

/// The lifetime parameter `'s` refers to the lifetime of the step.
/// It cannot escape the step function.
#[derive(Debug, Clone)]
pub struct DataTable<'s> {
    pickle_table: &'s PickleTable,
}

impl<'s> From<&'s PickleTable> for DataTable<'s> {
    fn from(pickle_table: &'s PickleTable) -> Self {
        DataTable {
            pickle_table,
        }
    }
}

impl<'s> DataTable<'s> {
    pub fn to_vec<T: FromDataTableRow<'s>>(&'s self) -> super::FromStepArgumentResult<Vec<T>> {
        self.pickle_table.rows.iter()
            .skip(1)
            .map(|row| T::from_data_table_row(&row.cells))
            .collect()
    }
}

/// Converts a row of the `DataTable` to `Self`.
///
/// The lifetime parameter `'r` refers to the lifetime of the DataTable row,
/// which is same as the DataTable itself. It cannot escape the step function.
pub trait FromDataTableRow<'r>: Sized {
    fn from_data_table_row<S: AsRef<str>>(row: &'r [S]) -> super::FromStepArgumentResult<Self>;
}
