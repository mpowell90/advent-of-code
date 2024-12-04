fn main() {
    let input = include_str!("./input.txt");

    println!("Part 1: {}", WordSearch::parse(input).search_all());
}

#[derive(Debug)]
pub struct WordSearch {
    row_count: usize,
    col_count: usize,
    chars: Vec<Vec<char>>,
}

impl WordSearch {
    pub fn parse(input: &str) -> Self {
        let lines = input.lines();
        let row_count = lines.clone().count();
        let col_count = lines.clone().next().unwrap().chars().count();

        Self {
            row_count,
            col_count,
            chars: lines.map(|line| line.chars().collect()).collect(),
        }
    }

    pub fn matcher(&self, ch: char, current_count: &mut usize, found: &mut usize) {
        match ch {
            'X' => {
                *current_count = 1;
            }
            'M' if *current_count == 1 => {
                *current_count = 2;
            }
            'A' if *current_count == 2 => {
                *current_count = 3;
            }
            'S' if *current_count == 3 => {
                *found += 1;
                *current_count = 0;
            }
            _ => {
                *current_count = 0;
            }
        }
    }

    pub fn search_all(&self) -> usize {
        let mut found = 0;

        found += self.search_east();
        found += self.search_west();
        found += self.search_south();
        found += self.search_north();
        found += self.search_south_east();
        found += self.search_south_west();
        found += self.search_north_east();
        found += self.search_north_west();

        found
    }

    pub fn search_east(&self) -> usize {
        let mut current_count = 0;
        let mut found = 0;

        for row in 0..self.row_count {
            for col in 0..self.col_count {
                self.matcher(self.chars[row][col], &mut current_count, &mut found);
            }
            current_count = 0;
        }

        println!("search_east found: {}", found);

        found
    }

    pub fn search_west(&self) -> usize {
        let mut current_count = 0;
        let mut found = 0;

        for row in 0..self.row_count {
            for col in (0..self.col_count).rev() {
                self.matcher(self.chars[row][col], &mut current_count, &mut found);
            }
            current_count = 0;
        }

        println!("search_west found: {}", found);

        found
    }

    pub fn search_south(&self) -> usize {
        let mut current_count = 0;
        let mut found = 0;

        for col in 0..self.col_count {
            for row in 0..self.row_count {
                self.matcher(self.chars[row][col], &mut current_count, &mut found);
            }
            current_count = 0;
        }

        println!("search_south found: {}", found);

        found
    }

    pub fn search_north(&self) -> usize {
        let mut current_count = 0;
        let mut found = 0;

        for col in (0..self.col_count).rev() {
            for row in (0..self.row_count).rev() {
                self.matcher(self.chars[row][col], &mut current_count, &mut found);
            }
            current_count = 0;
        }

        println!("search_north found: {}", found);

        found
    }

    pub fn search_north_west(&self) -> usize {
        let mut current_count = 0;
        let mut found = 0;

        let mut last_start_row = self.row_count as isize - 1;
        let mut last_start_col = 0_isize;

        let mut row_offset = self.row_count as isize - 1;
        let mut col_offset = 0_isize;

        while row_offset >= 0 && col_offset < self.col_count as isize {
            // println!(
            //     "row {}, col {}, ch {}",
            //     row_offset, col_offset, self.chars[row_offset as usize][col_offset as usize]
            // );
            self.matcher(
                self.chars[row_offset as usize][col_offset as usize],
                &mut current_count,
                &mut found,
            );

            if col_offset == 0 || row_offset == 0 {
                if last_start_col < self.col_count as isize - 1 {
                    last_start_col += 1;
                    row_offset = self.row_count as isize - 1;
                    col_offset = last_start_col;
                } else {
                    last_start_row -= 1;
                    row_offset = last_start_row;
                    col_offset = last_start_col;
                }

                current_count = 0;

                continue;
            }

            row_offset -= 1;
            col_offset -= 1;
        }

        println!("search_north_west found: {}", found);

        found
    }

    pub fn search_south_east(&self) -> usize {
        let mut current_count = 0;
        let mut found = 0;

        let mut last_start_row = self.row_count as isize - 1;
        let mut last_start_col = 0_isize;

        let mut row_offset = self.row_count as isize - 1;
        let mut col_offset = 0_isize;

        while row_offset >= 0 && col_offset < self.col_count as isize {
            // println!(
            //     "row {}, col {}, ch {}",
            //     row_offset, col_offset, self.chars[row_offset as usize][col_offset as usize]
            // );

            self.matcher(
                self.chars[row_offset as usize][col_offset as usize],
                &mut current_count,
                &mut found,
            );

            if col_offset == self.col_count as isize - 1
                || row_offset == self.row_count as isize - 1
            {
                if last_start_row > 0 {
                    last_start_row -= 1;
                    col_offset = 0;
                    row_offset = last_start_row;
                } else {
                    last_start_col += 1;
                    row_offset = last_start_row;
                    col_offset = last_start_col;
                }

                current_count = 0;

                continue;
            }

            row_offset += 1;
            col_offset += 1;
        }

        println!("search_south_east found: {}", found);

        found
    }

    pub fn search_north_east(&self) -> usize {
        let mut current_count = 0;
        let mut found = 0;

        let mut last_start_row = self.row_count as isize - 1;
        let mut last_start_col = self.col_count as isize - 1;

        let mut row_offset = self.row_count as isize - 1;
        let mut col_offset = self.col_count as isize - 1;

        while row_offset >= 0 && col_offset >= 0 {
            // println!(
            //     "row {}, col {}, ch {}",
            //     row_offset, col_offset, self.chars[row_offset as usize][col_offset as usize]
            // );

            self.matcher(
                self.chars[row_offset as usize][col_offset as usize],
                &mut current_count,
                &mut found,
            );

            if col_offset == self.col_count as isize - 1
                || row_offset == 0
            {
                if last_start_col > 0 {
                    last_start_col -= 1;
                    row_offset = self.row_count as isize - 1;
                    col_offset = last_start_col;
                } else {
                    last_start_row -= 1;
                    row_offset = last_start_row;
                    col_offset = last_start_col;
                }

                current_count = 0;

                continue;
            }

            row_offset -= 1;
            col_offset += 1;
        }

        println!("search_north_east found: {}", found);

        found
    }

    pub fn search_south_west(&self) -> usize {
        let mut current_count = 0;
        let mut found = 0;

        let mut last_start_row = 0;
        let mut last_start_col = 0;

        let mut row_offset = 0;
        let mut col_offset = 0;

        while row_offset < self.row_count as isize && col_offset < self.col_count as isize {
            // println!(
            //     "row {}, col {}, ch {}",
            //     row_offset, col_offset, self.chars[row_offset as usize][col_offset as usize]
            // );

            self.matcher(
                self.chars[row_offset as usize][col_offset as usize],
                &mut current_count,
                &mut found,
            );

            if col_offset == 0
                || row_offset == self.row_count as isize - 1
            {
                if last_start_col < self.col_count as isize - 1 {
                    last_start_col += 1;
                    row_offset = 0;
                    col_offset = last_start_col;
                } else {
                    last_start_row += 1;
                    row_offset = last_start_row;
                    col_offset = last_start_col;
                }

                current_count = 0;

                continue;
            }

            row_offset += 1;
            col_offset -= 1;
        }

        println!("search_south_west found: {}", found);

        found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_east_search() {
        const EXAMPLE: &str = "XMAS.\n\
                               .XMAS";
        assert_eq!(WordSearch::parse(EXAMPLE).search_east(), 2);
    }

    #[test]
    fn should_west_search() {
        const EXAMPLE: &str = "SAMX.\n\
                               .SAMX";
        assert_eq!(WordSearch::parse(EXAMPLE).search_west(), 2);
    }

    #[test]
    fn should_south_search() {
        let example = ".X\n\
                             XM\n\
                             MA\n\
                             AS\n\
                             S.";
        assert_eq!(WordSearch::parse(example).search_south(), 2);
    }

    #[test]
    fn should_north_search() {
        let example = ".S\n\
                             SA\n\
                             AM\n\
                             MX\n\
                             X.";
        assert_eq!(WordSearch::parse(example).search_north(), 2);
    }

    #[test]
    fn should_south_east_search() {
        let example = "X...\n\
                             XM..\n\
                             .MA.\n\
                             ..AS\n\
                             ...S";
        assert_eq!(WordSearch::parse(example).search_south_east(), 2);
    }

    #[test]
    fn should_south_west_search() {
        let example = "...X\n\
                             ..MX\n\
                             .AM.\n\
                             SA..\n\
                             S...";
        assert_eq!(WordSearch::parse(example).search_south_west(), 2);
    }

    #[test]
    fn should_north_east_search() {
        let example = "...S\n\
                             ..AS\n\
                             .MA.\n\
                             XM..\n\
                             X...";
        assert_eq!(WordSearch::parse(example).search_north_east(), 2);
    }

    #[test]
    fn should_north_west_search() {
        let example = "S...\n\
                             SA..\n\
                             .AM.\n\
                             ..MX\n\
                             ...X";
        assert_eq!(WordSearch::parse(example).search_north_west(), 2);
    }

    #[test]
    fn should_find_all() {
        const EXAMPLE_1: &str = "....XXMAS.\n\
                                 .SAMXMS...\n\
                                 ...S..A...\n\
                                 ..A.A.MS.X\n\
                                 XMASAMX.MM\n\
                                 X.....XA.A\n\
                                 S.S.S.S.SS\n\
                                 .A.A.A.A.A\n\
                                 ..M.M.M.MM\n\
                                 .X.X.XMASX";

        assert_eq!(WordSearch::parse(EXAMPLE_1).search_south_east(), 1);
        assert_eq!(WordSearch::parse(EXAMPLE_1).search_north_west(), 4);
        assert_eq!(WordSearch::parse(EXAMPLE_1).search_north_east(), 4);
        assert_eq!(WordSearch::parse(EXAMPLE_1).search_south_west(), 1);

        assert_eq!(WordSearch::parse(EXAMPLE_1).search_all(), 18);
    }
}
