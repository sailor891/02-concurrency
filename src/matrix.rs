use crate::vector::Vector;
use anyhow::Result;
use std::{
    fmt::{self, Debug},
    iter::Sum,
    ops::{Add, AddAssign, Mul},
    sync::mpsc,
    thread,
};

const NUM_THREADS: usize = 4;
pub struct Matrix<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>,
}
pub struct MsgInput<T> {
    pub idx: usize,
    pub row: Vector<T>,
    pub col: Vector<T>,
}
pub struct MsgOutput<T> {
    pub idx: usize,
    pub value: T,
}
pub struct Msg<T> {
    pub inputs: MsgInput<T>,
    pub sender: oneshot::Sender<MsgOutput<T>>,
}

impl<T: Debug> Matrix<T> {
    pub fn new(rows: usize, cols: usize, data: impl Into<Vec<T>>) -> Matrix<T>
    where
        T: fmt::Debug,
    {
        Matrix {
            rows,
            cols,
            data: data.into(),
        }
    }
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: fmt::Debug
        + Default
        + Add<Output = T>
        + AddAssign
        + Mul<Output = T>
        + Copy
        + Sum<T>
        + Send
        + 'static,
{
    if a.cols != b.rows {
        return Err(anyhow::anyhow!("Matrix dimensions do not match"));
    }

    let senders = (0..NUM_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let value = msg.inputs.row.dot_product(&msg.inputs.col)?;
                    if let Err(e) = msg.sender.send(MsgOutput {
                        idx: msg.inputs.idx,
                        value,
                    }) {
                        eprint!("{:?}", e);
                    };
                }
                Ok::<_, anyhow::Error>(())
            });
            tx
        })
        .collect::<Vec<_>>();
    let matrix_len = a.rows * b.cols;
    let bt = b.transposition(); // b的转置
    let mut data = vec![T::default(); matrix_len];
    let mut receivers = Vec::with_capacity(matrix_len);
    for i in 0..a.rows {
        for j in 0..bt.rows {
            let row = Vector::new(&a.data[i * a.cols..(i + 1) * a.cols]);
            let col = Vector::new(&bt.data[j * bt.cols..(j + 1) * bt.cols]);
            let inputs = MsgInput::new(i * b.cols + j, row, col);
            let (tx, rx) = oneshot::channel();
            let msg = Msg::new(inputs, tx);
            if let Err(e) = senders[i % NUM_THREADS].send(msg) {
                eprint!("{:?}", e);
            }
            receivers.push(rx);
        }
    }
    for rx in receivers {
        let msg = rx.recv().unwrap();
        data[msg.idx] = msg.value;
    }
    let result = Matrix::new(a.rows, b.cols, data);
    Ok(result)
}
impl<T> fmt::Display for Matrix<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.rows {
            write!(f, "{{")?;
            for j in 0..self.cols {
                write!(f, "{}", self.data[i * self.cols + j])?;
                if j != self.cols - 1 {
                    write!(f, " ")?;
                } else {
                    write!(f, "}}")?;
                }
            }
            if i != self.rows - 1 {
                writeln!(f)?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}
impl<T> MsgInput<T> {
    pub fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        Self { idx, row, col }
    }
}
impl<T> MsgOutput<T> {
    pub fn new(idx: usize, value: T) -> Self {
        Self { idx, value }
    }
}
impl<T> Msg<T> {
    pub fn new(inputs: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { inputs, sender }
    }
}
impl<T> fmt::Debug for Matrix<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Matrix {{ rows: {}, cols: {}, data: {:?} }}",
            self.rows, self.cols, self.data
        )
    }
}
impl<T> Matrix<T>
where
    T: fmt::Debug + Default + Add<Output = T> + AddAssign + Mul<Output = T> + Copy,
{
    pub fn transposition(&self) -> Matrix<T> {
        let mut data = vec![T::default(); self.cols * self.rows];
        for i in 0..self.rows {
            for j in 0..self.cols {
                data[j * self.rows + i] = self.data[i * self.cols + j];
            }
        }
        Matrix::new(self.cols, self.rows, data)
    }
}
impl<T> Mul for Matrix<T>
where
    T: fmt::Debug
        + Default
        + Add<Output = T>
        + AddAssign
        + Mul<Output = T>
        + Copy
        + Sum<T>
        + Send
        + 'static,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        multiply(&self, &rhs).expect("Matrix multiplication failed")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_multiply() {
        let a = Matrix::new(2, 3, [1, 2, 3, 4, 5, 6]);
        let b = Matrix::new(3, 2, [5, 6, 7, 8, 9, 10]);
        let result = a * b;
        assert_eq!(result.rows, 2);
        assert_eq!(result.cols, 2);
        // assert_eq!(result, format!("{:?}",result.data))
        println!("{}", result);
    }
    #[test]
    #[should_panic]
    fn test_multiply_panic() {
        let a = Matrix::new(2, 3, [1, 2, 3, 4, 5, 6]);
        let b = Matrix::new(3, 4, [5, 6, 7, 8, 9, 10]);
        let _result = a * b;
    }
}
