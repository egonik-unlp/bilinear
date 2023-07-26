use std::error::Error;
use std::fs;
use ndarray::{Array, Array2};


#[derive(Debug)]
pub struct EEMMatrix{
    pub excitation : Vec<i32>,
    pub emission : Vec<i32>,
    pub data : Array2<Vec<Vec<f64>>>
}



impl EEMMatrix {
pub fn load_eem(filename : &str) -> Result<Self, Box<dyn Error>> {
    // let url : &str = "http://cowlet.org/2016/08/23/linear-regression-in-rust.html";
    let filestring = fs::read_to_string(filename).unwrap();
    let mut fs_it = filestring.lines().into_iter();
    let headers: Vec<i32> = fs_it
        .next()
        .unwrap()
        .split('\t')
        .into_iter()
        .skip(1)
        .map(|num| num.parse::<i32>().unwrap())
        .collect();
    let mut body: Vec<Vec<f64>> = vec![];
    let mut col = vec![];
    for line in fs_it {
        let mut line_split_it = line.split('\t').into_iter();
        let future_first_col = line_split_it.next().unwrap();
        col.push(future_first_col.parse::<i32>().unwrap());
        body.push(line_split_it.map(|x| x.parse::<f64>().unwrap()).collect())
    }
    let array_body = Array::from_elem((body.len(), body[0].len()), body);
    println!("lambda ex {:?}", headers.len());
    println!("lambda em {:?}", col.len());
    println!("matriz itself => {:?}", array_body.shape());
    let return_matrix = EEMMatrix {
        excitation : headers,
        emission : col,
        data : array_body
    };
    Ok(return_matrix)
}
}