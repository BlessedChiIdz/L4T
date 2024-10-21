use std::vec;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Matrix {
    i:i32,
    j:i32,
    data:Vec<Vec<i32>>
}

impl Matrix{
    fn constructor(data:Vec<Vec<i32>>,i:i32,j:i32) -> Matrix{
        Matrix{
            data,
            i,
            j
        }
    }

    fn Plus(data1:Matrix,data2:Matrix) -> Result<Matrix,String> {
        if data1.i != data2.i || data1.j != data2.j{
            return Err(String::from("not same size"));
        }
        println!("{:?}", data1);
        println!("{:?}", data2);
        let i = data1.i;
        let j = data1.j;
        let mut matrixRet:Vec<Vec<i32>> = vec![];
        for q in 0..(data1.data.len() as usize){
            let mut arr:Vec<i32> = vec![];
            for w in 0..(data1.data[0].len() as usize){
                let sum = data1.data[q][w] + data2.data[q][w];
                arr.push(sum);
            }
            matrixRet.push(arr);
        }
        return Ok(Matrix::constructor(matrixRet, i, j));
    }

    fn Minus(data1:Matrix,data2:Matrix) -> Result<Matrix,String> {
        if data1.i != data2.i || data1.j != data2.j{
            return Err(String::from("not same size"));
        }
        let i = data1.i;
        let j = data1.j;
        let mut matrixRet:Vec<Vec<i32>> = vec![];
        for q in 0..(data1.data.len() as usize){
            let mut arr:Vec<i32> = vec![];
            for w in 0..(data1.data[0].len() as usize){
                let sum = data1.data[q][w] - data2.data[q][w];
                arr.push(sum);
            }
            matrixRet.push(arr);
        }
        return Ok(Matrix::constructor(matrixRet, i, j));
    }

    fn Multiply(data1:Matrix,data2:Matrix) -> Result<Matrix,String> {
        if data1.i != data2.i || data1.j != data2.j{
            return Err(String::from("not same size"));
        }
        let i = data1.i;
        let j = data1.j;
        let mut matrixRet:Vec<Vec<i32>> = vec![];
        for q in 0..(data1.data.len() as usize){
            let mut arr:Vec<i32> = vec![];
            for w in 0..(data1.data[0].len() as usize){
                let sum = data1.data[q][w] * data2.data[q][w];
                arr.push(sum);
            }
            matrixRet.push(arr);
        }
        return Ok(Matrix::constructor(matrixRet, i, j));
    }

    fn Eq(data1:Matrix,data2:Matrix) -> Result<bool,String> {
        let mut flag = true;
        if data1.i != data2.i || data1.j != data2.j{
            return Err(String::from("not same size"));
        }
        let i = data1.i;
        let j = data1.j;
        for q in 0..(data1.data.len() as usize){
            for w in 0..(data1.data[0].len() as usize){
                if data1.data[q][w] !=data2.data[q][w] {
                    flag = false;
                }
            }
        }
        return Ok(flag);
    }

    fn Trans(&self) -> Matrix {
        let sI = self.j as usize;
        let sJ = self.i as usize;
        let mut vecM:Vec<Vec<i32>> = vec![vec![0;0];0];
        for i in 0..sI{
            let mut temp = vec![];
            for j in 0..sJ{
                temp.push(self.data[j][i]);
            }
            vecM.push(temp);
        }
        let matrix = Matrix::constructor(vecM, self.j, self.i);
        return matrix;
    }

    fn Min(&self) -> i32 {
        let mut min = 9999;
        for i in 0..(self.i as usize){
            for j in 0..(self.j as usize){
                if min > self.data[i][j]{
                    min = self.data[i][j];
                }
            }
        }
        min
    }

    fn toStr(&self) -> String {
        let mut str = String::new();
        for i in 0..(self.i as usize){
            str.push('{');
            for j in 0..(self.j as usize){
                let temp = format!("{}", self.data[i][j]);
                str.push_str(&temp);
            }
            str.push('}');
        }
        return str;
    }

    fn getEl(&self,i:i32,j:i32) -> i32 {
       self.data[i as usize][j as usize]
    }

    fn getI(&self) -> i32 {
        self.i
    }

     fn getJ(&self) -> i32 {
        self.j
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testConstructor() {
        let i:i32 = 2;
        let j:i32 = 5;
        let data:Vec<Vec<i32>> = vec![vec![1;i as usize];j as usize];
        let result = Matrix::constructor(data, i, j);
        let expected:Matrix = Matrix {
            data: vec![vec![1;i as usize];j as usize],
            i,
            j
        };
        println!("{:?}", expected);
        assert_eq!(result, expected);
    }

    #[test]
    fn testPlus() {
        let i:i32 = 2;
        let j:i32 = 3;
        let data:Vec<Vec<i32>> = vec![vec![1;i as usize];j as usize];
        let matrix1 = Matrix::constructor(data.clone(), i.clone(), j.clone());
        let matrix2 = Matrix::constructor(data, i, j);
        let result = Matrix::Plus(matrix1, matrix2);
        let expected = Matrix {
            data: vec![vec![2;i as usize];j as usize],
            i,
            j
        };
        assert_eq!(result.unwrap(),expected);
    }

    #[test]
    fn testMinus() {
        let i:i32 = 2;
        let j:i32 = 3;
        let data:Vec<Vec<i32>> = vec![vec![1;i as usize];j as usize];
        let matrix1 = Matrix::constructor(data.clone(), i.clone(), j.clone());
        let matrix2 = Matrix::constructor(data, i, j);
        let result = Matrix::Minus(matrix1, matrix2);
        let expected = Matrix {
            data: vec![vec![0;i as usize];j as usize],
            i,
            j
        };
        assert_eq!(result.unwrap(),expected);
    }

    #[test]
    fn testMultiply() {
        let i:i32 = 2;
        let j:i32 = 3;
        let data:Vec<Vec<i32>> = vec![vec![2;i as usize];j as usize];
        let matrix1 = Matrix::constructor(data.clone(), i.clone(), j.clone());
        let matrix2 = Matrix::constructor(data, i, j);
        let result = Matrix::Multiply(matrix1, matrix2);
        let expected = Matrix {
            data: vec![vec![4;i as usize];j as usize],
            i,
            j
        };
        assert_eq!(result.unwrap(),expected);
    }

    #[test]
    fn testEq() {
        let i:i32 = 2;
        let j:i32 = 3;
        let data:Vec<Vec<i32>> = vec![vec![1;i as usize];j as usize];
        let matrix1 = Matrix::constructor(data.clone(), i.clone(), j.clone());
        let matrix2 = Matrix::constructor(data, i, j);
        let result = Matrix::Eq(matrix1, matrix2);
        let expected = true;
        assert_eq!(result.unwrap(),expected);
    }


    #[test]
    fn testTrans() {
        let mut data = vec![];
        data.push(vec![0,1,2]);
        data.push(vec![3,4,5]);
        let i = 2;
        let j = 3;
        let mut test: Matrix = Matrix::constructor(data, i, j);
        let result = Matrix::Trans(&test);
        let mut expectedData = vec![];
        expectedData.push(vec![0,3]);
        expectedData.push(vec![1,4]);
        expectedData.push(vec![2,5]);
        let expected = Matrix {
            data: expectedData,
            i:j,
            j:i
        };
        assert_eq!(result,expected);
    }

    #[test]
    fn testMin() {
        let mut data = vec![];
        data.push(vec![0,1,2]);
        data.push(vec![3,4,5]);
        let i = 2;
        let j = 3;
        let mut test: Matrix = Matrix::constructor(data, i, j);
        let result = Matrix::Min(&test);
        let expected = 0;
        assert_eq!(result,expected);
    }

    #[test]
    fn testToStr() {
        let mut data = vec![];
        data.push(vec![0,1,2]);
        data.push(vec![3,4,5]);
        let i = 2;
        let j = 3;
        let mut test: Matrix = Matrix::constructor(data, i, j);
        let result = Matrix::toStr(&test);
        let expected = "{012}{345}";
        assert_eq!(result,expected);
    }

    #[test]
    fn testGetEl() {
        let mut data = vec![];
        data.push(vec![0,1,2]);
        data.push(vec![3,4,5]);
        let i = 2;
        let j = 3;
        let mut test: Matrix = Matrix::constructor(data, i, j);
        let result = Matrix::getEl(&test,1,2);
        let expected = 5;
        assert_eq!(result,expected);
    }


    #[test]
    fn testGetI() {
        let mut data = vec![];
        data.push(vec![0,1,2]);
        data.push(vec![3,4,5]);
        let i = 2;
        let j = 3;
        let mut test: Matrix = Matrix::constructor(data, i, j);
        let result = Matrix::getI(&test);
        let expected = 2;
        assert_eq!(result,expected);
    }

    #[test]
    fn testGetJ() {
        let mut data = vec![];
        data.push(vec![0,1,2]);
        data.push(vec![3,4,5]);
        let i = 2;
        let j = 3;
        let mut test: Matrix = Matrix::constructor(data, i, j);
        let result = Matrix::getJ(&test);
        let expected = 3;
        assert_eq!(result,expected);
    }
}


fn main() {
    let mut data1 = vec![];
    data1.push(vec![0,1,2]);
    data1.push(vec![3,4,5]);
    let mut data2 = vec![];
    data2.push(vec![0,1,2]);
    data2.push(vec![3,4,5]);
    let i = 2;
    let j = 3;
    let mut test1: Matrix = Matrix::constructor(data1, i, j);
    let test2: Matrix = Matrix::constructor(data2, i, j);
    let min = test2.getJ();
    test1.Trans();
    println!("{:?}", min);
}
