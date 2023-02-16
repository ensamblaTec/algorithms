#[derive(Debug)]
pub struct Algorithm {
    pub data: Vec<usize>,
}

impl Algorithm {
    pub fn new(data: Vec<usize>) -> Algorithm {
        Algorithm { data }
    }

    pub fn bubble_sort(&mut self) {
        let len = self.data.len();
        let mut cnt: usize = 1;
        let mut flag: u8 = 1;
        'ext: loop {
            while cnt < len-cnt {
                if self.data[cnt] < self.data[cnt-1] {
                    flag = 0;
                    let tmp = self.data.get(cnt).unwrap().clone();
                    self.data[cnt] = *self.data.get(cnt-1).unwrap();
                    self.data[cnt-1] = tmp;
                }
                cnt+=1;
            }
            if flag == 1 {
                break 'ext;
            }
            flag = 1;
        }
    }

    pub fn selection_sort(&mut self) {
        let len: usize = self.data.len()-1;
        let mut cnt: usize = 0;
        let mut pivot: usize;
        'ext: loop {
            let current: usize = self.data.get(cnt).unwrap().clone();
            pivot = cnt;
            for i in cnt..len {
                if current >= self.data.get(i+1).unwrap().clone() {
                    pivot = i+1;
                }
            }
            if cnt == len {
                break 'ext;
            }
            let tmp = self.data[pivot].clone();
            self.data[cnt] = tmp;
            self.data[pivot] = current; 
            println!("{:?}\n===\tchange\t===", self.data);
            cnt += 1;
        }
    }

    pub fn insertion_sort() {
        let len: usize = self.data.len()-1;
        let flag: bool = true;
        while flag {

        }
    }
}
