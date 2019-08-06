use gherkin::cuke;

pub use self::convert::{FromDataTableBodyRow, FromDataTableRow};
pub use self::iter::{FromDataTableBodyRowIter, FromDataTableRowIter};
use self::iter::DataTableIter;
pub use self::row::{BodyRow, BodyRowRef, Row, RowRef};

mod row;
mod convert;
mod iter;

/// A data table that is attached to a step text.
///
/// The lifetime parameter `'s` refers to the lifetime of the step.
/// It cannot escape the step function.
///
/// # Examples
///
/// With the scenario:
/// ```gherkin
/// Given the previous entries:
///   | first | second | operation |
///   | 1     | 1      | +         |
///   | 2     | 1      | +         |
/// ```
/// and the glue code:
/// ```rust
/// # #![feature(custom_attribute, param_attrs)]
/// #
/// # pub struct Calc;
/// # impl Calc {
/// #     pub fn push(&mut self, s: &str) {}
/// # }
/// use cuke_runner_glue::step::argument::{DataTable, FromDataTableBodyRow, BodyRowRef};
///
/// struct Entry<'dt> {
///     first: &'dt str,
///     second: &'dt str,
///     operation: &'dt str,
/// }
///
/// impl<'dt> FromDataTableBodyRow<'dt> for Entry<'dt> {
///     fn from(body_row: BodyRowRef<'_, 'dt>) -> Self {
///         Entry {
///             first: body_row["first"],
///             second: body_row["second"],
///             operation: body_row["operation"],
///         }
///     }
/// }
///
/// #[given("the previous entries:")]
/// pub fn previous_entries(#[scenario] calc: &mut Calc, data_table: &DataTable) {
///     for entry in data_table.body_rows::<Entry>() {
///         calc.push(entry.first);
///         calc.push(entry.second);
///         calc.push(entry.operation);
///     }
/// }
/// ```
/// The data table will contain all cells in the table.
/// The values can be extract via one of the `Iterator` returning functions.
#[derive(Debug, Clone)]
pub struct DataTable<'s> {
    cuke_table: &'s cuke::Table<'s>,
}

#[doc(hidden)]
impl<'s> From<&'s cuke::Table<'s>> for DataTable<'s> {
    fn from(cuke_table: &'s cuke::Table<'s>) -> Self {
        DataTable {
            cuke_table,
        }
    }
}

impl<'s> DataTable<'s> {
    /// Returns an table row `Iterator` returning an row columns `Iterator`
    /// to get all cell values of the table.
    ///
    /// This should provide maximum flexibility when needed.
    ///
    /// A more idiomatic way to get the data out of the table is provided
    /// with the [`rows`] and [`body_rows`] functions.
    ///
    /// [`rows`]: ./struct.DataTable.html#method.rows
    /// [`body_rows`]: ./struct.DataTable.html#method.body_rows
    pub fn iter(&'s self)
        -> impl Iterator<Item=impl Iterator<Item=&'s str>>
    {
        DataTableIter::new(self)
    }

    /// Returns an `Iterator` implementation that automatically converts **all** rows
    /// using [`FromDataTableRow`] to a custom type.
    ///
    /// # Examples
    ///
    /// Scenario:
    /// ```gherkin
    /// Given the previous entries:
    ///   | 1     | 1      | +         |
    ///   | 2     | 1      | +         |
    /// ```
    /// Glue code:
    /// ```rust
    /// # #![feature(custom_attribute, param_attrs)]
    /// #
    /// # pub struct Calc;
    /// # impl Calc {
    /// #     pub fn push(&mut self, s: &str) {}
    /// # }
    /// use cuke_runner_glue::step::argument::{DataTable, FromDataTableRow, RowRef};
    ///
    /// struct Entry<'dt> {
    ///     first: &'dt str,
    ///     second: &'dt str,
    ///     operation: &'dt str,
    /// }
    ///
    /// impl<'dt> FromDataTableRow<'dt> for Entry<'dt> {
    ///     fn from(row: RowRef<'_, 'dt>) -> Self {
    ///         Entry {
    ///             first: row[0],
    ///             second: row[1],
    ///             operation: row[2],
    ///         }
    ///     }
    /// }
    ///
    /// #[given("the previous entries:")]
    /// pub fn previous_entries(#[scenario] calc: &mut Calc, data_table: &DataTable) {
    ///     for entry in data_table.rows::<Entry>() {
    ///         calc.push(entry.first);
    ///         calc.push(entry.second);
    ///         calc.push(entry.operation);
    ///     }
    /// }
    /// ```
    ///
    /// [`FromDataTableRow`]: ./trait.FromDataTableRow.html
    pub fn rows<T: FromDataTableRow<'s>>(&'s self)
        -> FromDataTableRowIter<'s, T>
    {
        FromDataTableRowIter::new(self)
    }

    /// Returns an `Iterator` implementation that automatically converts **only body rows**
    /// (all rows except the first one) using [`FromDataTableBodyRow`] to a custom type.
    ///
    /// # Examples
    ///
    /// Scenario:
    /// ```gherkin
    /// Given the previous entries:
    ///   | first | second | operation |
    ///   | 1     | 1      | +         |
    ///   | 2     | 1      | +         |
    /// ```
    /// Glue code:
    /// ```rust
    /// # #![feature(custom_attribute, param_attrs)]
    /// #
    /// # pub struct Calc;
    /// # impl Calc {
    /// #     pub fn push(&mut self, s: &str) {}
    /// # }
    /// use cuke_runner_glue::step::argument::{DataTable, FromDataTableBodyRow, BodyRowRef};
    ///
    /// struct Entry<'dt> {
    ///     first: &'dt str,
    ///     second: &'dt str,
    ///     operation: &'dt str,
    /// }
    ///
    /// impl<'dt> FromDataTableBodyRow<'dt> for Entry<'dt> {
    ///     fn from(body_row: BodyRowRef<'_, 'dt>) -> Self {
    ///         Entry {
    ///             first: body_row["first"],
    ///             second: body_row["second"],
    ///             operation: body_row["operation"],
    ///         }
    ///     }
    /// }
    ///
    /// #[given("the previous entries:")]
    /// pub fn previous_entries(#[scenario] calc: &mut Calc, data_table: &DataTable) {
    ///     for entry in data_table.body_rows::<Entry>() {
    ///         calc.push(entry.first);
    ///         calc.push(entry.second);
    ///         calc.push(entry.operation);
    ///     }
    /// }
    /// ```
    ///
    /// [`FromDataTableBodyRow`]: ./trait.FromDataTableBodyRow.html
    pub fn body_rows<T: FromDataTableBodyRow<'s>>(&'s self)
        -> FromDataTableBodyRowIter<'s, T>
    {
        FromDataTableBodyRowIter::new(self)
    }

    /// Returns the number of rows in this table (height).
    pub fn rows_len(&self) -> usize {
        self.cuke_table.rows.len()
    }

    /// Returns the number of columns in this table (width).
    pub fn columns_len(&self) -> usize {
        self.cuke_table.rows.first()
            .map(|first_row| first_row.cells.len())
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use std::iter::Iterator;

    use super::*;

    const EMPTY_TABLE: &[&[&str]] = &[];
    const DEFAULT_NON_EMPTY_TABLE: &[&[&str]] = &[
        &["first", "second", "operation"],
        &["1", "4", "+"],
        &["2", "5", "*"],
        &["7", "6", "-"],
    ];

    #[test]
    fn rows_len_0() {
        let cuke_table = create_cuke_table(EMPTY_TABLE);
        let data_table = DataTable::from(&cuke_table);

        assert_eq!(data_table.rows_len(), 0);
    }

    #[test]
    fn rows_len_4() {
        let cuke_table = create_cuke_table(DEFAULT_NON_EMPTY_TABLE);
        let data_table = DataTable::from(&cuke_table);

        assert_eq!(data_table.rows_len(), 4);
    }

    #[test]
    fn columns_len_0() {
        let cuke_table = create_cuke_table(EMPTY_TABLE);
        let data_table = DataTable::from(&cuke_table);

        assert_eq!(data_table.columns_len(), 0);
    }

    #[test]
    fn columns_len_3() {
        let cuke_table = create_cuke_table(DEFAULT_NON_EMPTY_TABLE);
        let data_table = DataTable::from(&cuke_table);

        assert_eq!(data_table.columns_len(), 3);
    }

    #[test]
    fn iter() {
        let cuke_table = create_cuke_table(DEFAULT_NON_EMPTY_TABLE);
        let data_table = DataTable::from(&cuke_table);

        assert_eq!(data_table.iter().next().unwrap().next().unwrap(), "first");
        assert_eq!(data_table.iter().last().unwrap().last().unwrap(), "-");
    }

    #[test]
    fn rows() {
        let cuke_table = create_cuke_table(DEFAULT_NON_EMPTY_TABLE);
        let data_table = DataTable::from(&cuke_table);

        assert_eq!(data_table.rows::<Row>().count(), 4);
        assert_eq!(&data_table.rows::<Row>().next().unwrap()[1], "second");
    }

    #[test]
    fn custom_rows() {
        let cuke_table = create_cuke_table(DEFAULT_NON_EMPTY_TABLE);
        let data_table = DataTable::from(&cuke_table);

        assert_eq!(data_table.rows::<TestRow>().count(), 4);

        let mut test_row_iter = data_table.rows::<TestRow>();

        let header_test_row = test_row_iter.next().unwrap();
        assert_eq!(header_test_row.first, "first");
        assert_eq!(header_test_row.second, "second");
        assert_eq!(header_test_row.operation, "operation");
        let body_test_row = test_row_iter.next().unwrap();
        assert_eq!(body_test_row.first, "1");
        assert_eq!(body_test_row.second, "4");
        assert_eq!(body_test_row.operation, "+");

        let _collect_works = test_row_iter.collect::<Vec<_>>();
    }

    #[test]
    fn body_rows() {
        let cuke_table = create_cuke_table(DEFAULT_NON_EMPTY_TABLE);
        let data_table = DataTable::from(&cuke_table);

        assert_eq!(data_table.body_rows::<BodyRow>().count(), 3);
        assert_eq!(&data_table.body_rows::<BodyRow>().next().unwrap()["operation"], "+");
    }

    #[test]
    fn custom_body_rows() {
        let cuke_table = create_cuke_table(DEFAULT_NON_EMPTY_TABLE);
        let data_table = DataTable::from(&cuke_table);

        assert_eq!(data_table.body_rows::<BodyTestRowIndexedStr>().count(), 3);

        let mut test_row_iter = data_table.body_rows::<BodyTestRowIndexedStr>();
        let first_row = test_row_iter.next().unwrap();
        assert_eq!(first_row.first, "1");
        assert_eq!(first_row.second, "4");
        assert_eq!(first_row.operation, "+");
        let second_row = test_row_iter.next().unwrap();
        assert_eq!(second_row.first, "2");
        assert_eq!(second_row.second, "5");
        assert_eq!(second_row.operation, "*");

        let mut test_row_iter = data_table.body_rows::<BodyTestRowIndexedUsize>();
        let first_row = test_row_iter.next().unwrap();
        assert_eq!(first_row.first, "1");
        assert_eq!(first_row.second, "4");
        assert_eq!(first_row.operation, "+");
        let second_row = test_row_iter.next().unwrap();
        assert_eq!(second_row.first, "2");
        assert_eq!(second_row.second, "5");
        assert_eq!(second_row.operation, "*");

        let mut test_row_iter = data_table.body_rows::<BodyTestRowByHeaderName>();
        let first_row = test_row_iter.next().unwrap();
        assert_eq!(first_row.first, "1");
        assert_eq!(first_row.second, "4");
        assert_eq!(first_row.operation, "+");
        let second_row = test_row_iter.next().unwrap();
        assert_eq!(second_row.first, "2");
        assert_eq!(second_row.second, "5");
        assert_eq!(second_row.operation, "*");

        let mut test_row_iter = data_table.body_rows::<BodyTestRowByValueAtIndex>();
        let first_row = test_row_iter.next().unwrap();
        assert_eq!(first_row.first, "1");
        assert_eq!(first_row.second, "4");
        assert_eq!(first_row.operation, "+");
        let second_row = test_row_iter.next().unwrap();
        assert_eq!(second_row.first, "2");
        assert_eq!(second_row.second, "5");
        assert_eq!(second_row.operation, "*");

        let _collect_works = test_row_iter.collect::<Vec<_>>();
    }

    struct TestRow<'dt> {
        first: &'dt str,
        second: &'dt str,
        operation: &'dt str,
    }

    impl<'dt> FromDataTableRow<'dt> for TestRow<'dt> {
        fn from(row: RowRef<'_, 'dt>) -> Self {
            TestRow {
                first: row[0],
                second: row[1],
                operation: row[2],
            }
        }
    }

    struct BodyTestRowIndexedStr<'dt> {
        first: &'dt str,
        second: &'dt str,
        operation: &'dt str,
    }

    impl<'dt> FromDataTableBodyRow<'dt> for BodyTestRowIndexedStr<'dt> {
        fn from(row: BodyRowRef<'_, 'dt>) -> Self {
            BodyTestRowIndexedStr {
                first: row["first"],
                second: row["second"],
                operation: row["operation"],
            }
        }
    }

    struct BodyTestRowIndexedUsize<'dt> {
        first: &'dt str,
        second: &'dt str,
        operation: &'dt str,
    }

    impl<'dt> FromDataTableBodyRow<'dt> for BodyTestRowIndexedUsize<'dt> {
        fn from(row: BodyRowRef<'_, 'dt>) -> Self {
            BodyTestRowIndexedUsize {
                first: row[0],
                second: row[1],
                operation: row[2],
            }
        }
    }

    struct BodyTestRowByHeaderName<'dt> {
        first: &'dt str,
        second: &'dt str,
        operation: &'dt str,
    }

    impl<'dt> FromDataTableBodyRow<'dt> for BodyTestRowByHeaderName<'dt> {
        fn from(row: BodyRowRef<'_, 'dt>) -> Self {
            BodyTestRowByHeaderName {
                first: row.value("first").unwrap(),
                second: row.value("second").unwrap(),
                operation: row.value("operation").unwrap(),
            }
        }
    }

    struct BodyTestRowByValueAtIndex<'dt> {
        first: &'dt str,
        second: &'dt str,
        operation: &'dt str,
    }

    impl<'dt> FromDataTableBodyRow<'dt> for BodyTestRowByValueAtIndex<'dt> {
        fn from(row: BodyRowRef<'_, 'dt>) -> Self {
            BodyTestRowByValueAtIndex {
                first: row.value_at_index(0).unwrap(),
                second: row.value_at_index(1).unwrap(),
                operation: row.value_at_index(2).unwrap(),
            }
        }
    }

    fn create_cuke_table<'a>(table: &'a [&'a [&'a str]]) -> cuke::Table<'a> {
        cuke::Table {
            rows: table.iter()
                .map(|row| {
                    cuke::Row {
                        cells: row.iter()
                            .map(|cell| {
                                cuke::Cell {
                                    location: cuke::Location { line: 0, column: 0 },
                                    value: std::borrow::Cow::Borrowed(cell),
                                }
                            })
                            .collect()
                    }
                })
                .collect()
        }
    }
}
