use anyhow::Result;

use crate::app::Mode;

/// 0 based, only handles the cursor on the file
pub struct Cursor {
    /// 0 is the first line of the file, etc
    pub file_y: usize,
    pub file_x: usize,
}

impl Cursor {
    pub fn new(file_x: usize, file_y: usize) -> Self {
        Self {
            file_y: file_x,
            file_x: file_y,
        }
    }

    /// returns (x, y, file_y, mem_x)
    /// file_data is to determine length of each line
    /// Mode is to add extre length for insert
    pub fn move_and_set_coords(
        &mut self,
        x_off: isize,
        y_off: isize,
        file_data: &Vec<String>,
        mode: Mode,
    ) -> Result<()> {
        // clamp (y+y_off) to be between 0 and the length of the file
        let max_y = (file_data.len() as isize - 1).max(0);
        let y = (y_off + self.file_y as isize).clamp(0, max_y) as usize;
        //
        // similarly, clamp (x+x_off) to be between 0 and the length of the line
        let max_x = if file_data.len() == 0 {
            0
        } else {
            file_data[y].len()
        } as isize;
        let x = (x_off + self.file_x as isize).clamp(0, max_x) as usize;

        self.file_y = y;
        self.file_x = x;
        Ok(())
    }
}
