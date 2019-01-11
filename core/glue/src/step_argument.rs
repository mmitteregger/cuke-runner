use gherkin::pickle::*;

#[derive(Debug, Clone)]
pub enum StepArgument {
    Expression(ExpressionArgument),
    DocString(DocStringArgument),
    DataTable(DataTableArgument),
}

impl StepArgument {
    pub fn get_value(&self) -> &str {
        use self::StepArgument::*;

        match self {
            Expression(expression_argument) => &expression_argument.value,
            DocString(doc_string_argument) => &doc_string_argument.value,
            DataTable(data_table_argument) => panic!("cannot get str value from DataTable"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExpressionArgument {
    value: String,
    start: usize,
    end: usize,
}

impl ExpressionArgument {
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn start(&self) -> usize {
        self.start
    }
    pub fn end(&self) -> usize {
        self.end
    }
}

impl<'t> From<regex::Match<'t>> for ExpressionArgument {
    fn from(mat: regex::Match<'t>) -> Self {
        ExpressionArgument {
            value: mat.as_str().to_owned(),
            start: mat.start(),
            end: mat.end(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DocStringArgument {
    pub(crate) value: String,
}

impl From<&PickleString> for DocStringArgument {
    fn from(pickle_string: &PickleString) -> Self {
        DocStringArgument {
            value: pickle_string.content.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DataTable {
    pub(crate) value: Vec<Vec<String>>,
}

impl From<Vec<Vec<String>>> for DataTable {
    fn from(data_table: Vec<Vec<String>>) -> Self {
        DataTable {
            value: data_table,
        }
    }
}

impl DataTable {
    pub fn to_vec<T: FromDataTableRow>(&self) -> Vec<T> {
        self.value.iter()
            .skip(1)
            .map(|row| T::from_data_table_row(row))
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct DataTableArgument {
    pub(crate) value: DataTable,
}

impl From<&PickleTable> for DataTableArgument {
    fn from(pickle_table: &PickleTable) -> Self {
        let mut data_table = Vec::with_capacity(pickle_table.rows.len());

        for row in &pickle_table.rows {
            let mut data_table_cells = Vec::with_capacity(row.cells.len());

            for cell in &row.cells {
                data_table_cells.push(cell.value.clone());
            }

            data_table.push(data_table_cells)
        }

        DataTableArgument {
            value: DataTable::from(data_table),
        }
    }
}

pub trait FromDataTableRow: Sized {
    fn from_data_table_row(row: &[String]) -> Self;
}
