use std::vec;

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
        let i = data1.i;
        let j = data1.j;
        let mut matrixRet:Vec<Vec<i32>> = vec![];
        for q in 0..(i as usize){
            let mut arr:Vec<i32> = vec![];
            for w in 0..(j as usize){
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
        for q in 0..(i as usize){
            let mut arr:Vec<i32> = vec![];
            for w in 0..(j as usize){
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
        for q in 0..(i as usize){
            let mut arr:Vec<i32> = vec![];
            for w in 0..(j as usize){
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
        for q in 0..(i as usize){
            for w in 0..(j as usize){
                if data1.data[q][w] !=data2.data[q][w] {
                    flag = false;
                }
            }
        }
        return Ok(flag);
    }

    fn Trans(&self) -> Result<Matrix,String> {
        let i = self.i;
        let j = self.j;
        let mut temp = vec![];
        for q in 0..(i as usize){
            for w in 0..(j as usize){
                temp.push(self.data[q][w]);
            }
        }
        let mut arrToMatrix = vec![];
        let mut count = 0;
        let mut w = 0;
        let mut arr= vec![];
        println!("{:?}", temp);
        for q in 0..(temp.len() as usize){
            arr.push(self.data[w][count]);
            count+=1;
            if count>=j.try_into().unwrap() {
                count = 0;
                w+=1;
                arrToMatrix.push(arr);
                arr = vec![];
                println!("{:?}", arrToMatrix);
            }
        }
        let matrix = Matrix::constructor(arrToMatrix, i, j);
        return Ok(matrix);
    }
}



fn main() {
    let mut data1 = vec![];
    data1.push(vec![0,1,2]);
    data1.push(vec![3,4,5]);
    let mut data2 = vec![];
    data2.push(vec![0,1,2]);
    data2.push(vec![3,4,5]);
    let i = data1.len().try_into().unwrap();
    let j = data1[0].len().try_into().unwrap();
    let test1: Matrix = Matrix::constructor(data1, i, j);
    let test2: Matrix = Matrix::constructor(data2, i, j);
    let out = test1.Trans();
    println!("{:?}", out);
}
