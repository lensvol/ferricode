pub mod memory {
    use std::collections::HashMap;
    use std::ops::Range;

    pub(crate) type ComputerMemory = HashMap<usize, i32>;

    pub trait RangeAddressable {
        fn read_range(&self, range: Range<usize>) -> Vec<i32>;
        fn write_range(&mut self, range: Range<usize>, data: Vec<i32>);
    }

    impl RangeAddressable for ComputerMemory {
        fn read_range(&self, range: Range<usize>) -> Vec<i32> {
            range
                .into_iter()
                .map(|addr| *self.get(&addr).unwrap_or(&0))
                .collect()
        }

        fn write_range(&mut self, range: Range<usize>, data: Vec<i32>) {
            for addr in range {
                self.insert(addr, data[addr]);
            }
        }
    }
}