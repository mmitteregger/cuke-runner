use gherkin::pickle::PickleTable;

#[derive(Debug, Clone)]
pub struct DataTable {
    pub(crate) values: Vec<Vec<String>>,
}

impl From<&PickleTable> for DataTable {
    fn from(pickle_table: &PickleTable) -> Self {
        let mut data_table = Vec::with_capacity(pickle_table.rows.len());

        for row in &pickle_table.rows {
            let mut data_table_cells = Vec::with_capacity(row.cells.len());

            for cell in &row.cells {
                data_table_cells.push(cell.value.clone());
            }

            data_table.push(data_table_cells)
        }

        DataTable {
            values: data_table,
        }
    }
}

impl<'a> DataTable {
    pub fn to_vec<T: FromDataTableRow<'a>>(&'a self) -> super::FromStepArgumentResult<Vec<T>> {
        self.values.iter()
            .skip(1)
            .map(|row| T::from_data_table_row(row))
            .collect()
    }
}

/// Converts a row of the `DataTable` to `Self`.
///
/// The lifetime `'a` is the same as the lifetime of the DataTable.
pub trait FromDataTableRow<'a>: Sized {
    fn from_data_table_row<S: AsRef<str>>(row: &'a [S]) -> super::FromStepArgumentResult<Self>;
}
