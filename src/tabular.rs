pub struct Table {
    headers: Vec<String>,
    widths: Vec<usize>,
    length: usize,
}

impl Table {
    pub fn new(headers: &[&str]) -> Self {
        let widths: Vec<usize> = headers.iter().map(|s| s.len()).collect();

        Table::new_fixed(headers, &widths)
    }

    pub fn new_fixed(headers: &[&str], widths: &[usize]) -> Self {
        assert_eq!(headers.len(), widths.len());
        let length = headers.len();

        Table {
            headers: headers.into_iter().map(|s| s.to_string()).collect(),
            widths: widths.to_vec(),
            length,
        }
    }

    pub fn update_widths<R: AsRef<Vec<String>>>(&mut self, rows: &Vec<R>) {
        for row in rows {
            let row: &Vec<String> = row.as_ref();
            assert_eq!(row.len(), self.length);
            row.iter()
                .enumerate()
                .for_each(|(i, cell)| self.widths[i] = usize::max(self.widths[i], cell.len()));
        }
    }

    pub fn print<R: AsRef<Vec<String>>>(&self, rows: &Vec<R>) {
        self.print_headers();
        for row in rows {
            self.print_row(row);
        }
    }

    fn print_headers(&self) {
        println!(
            "| {} |",
            self.headers
                .iter()
                .enumerate()
                .map(|(i, v)| format!("{:<1$}", v, self.widths[i]))
                .collect::<Vec<_>>()
                .join(" | ")
        );
        println!(
            "| {} |",
            self.widths
                .iter()
                .map(|w| "-".repeat(*w))
                .collect::<Vec<_>>()
                .join(" | ")
        );
    }

    fn print_row<R: AsRef<Vec<String>>>(&self, row: &R) {
        let row: &Vec<String> = row.as_ref();
        assert_eq!(row.len(), self.length);

        println!(
            "| {} |",
            row.iter()
                .enumerate()
                .map(|(i, v)| format!("{:<1$}", v, self.widths[i]))
                .collect::<Vec<_>>()
                .join(" | ")
        );
    }
}
