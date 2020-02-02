extern crate csv;
use csv::*;

struct DataFrame {
     colum: Vec<String>,
     value: Vec<Vec<String>>,
}

impl DataFrame {
    fn head(&self,num:usize) -> DataFrame {
        let mut count = 0;
        let mut _value: Vec<Vec<String>> = Vec::new();
        for value_vector in &self.value {
            _value.push(value_vector.to_vec());
            count += 1;
            if count == num {break};
        }
        let _colum = &self.colum;
        DataFrame {
            colum: _colum.to_vec(),
            value: _value
        }
    }

    fn shape(&self) -> (usize,usize)  {
        (self.colum.len(),self.value.len())
    }

    fn col_select(&self,_col: &str) -> DataFrame {
        let col = _col.to_string();
        let mut _value = Vec::new();
        for value_vector in &self.value {
            for (i,value) in value_vector.iter().enumerate() {
                if &col == &self.colum[i] {
                    _value.push(vec![value.to_owned()]);
                }
            }
        }
        DataFrame {
            colum: vec![_col.to_string()],
            value: _value
        }
    }

}


fn read_csv(path:&str) -> DataFrame {
    // Build the CSV reader and iterate over each record.
    let mut value_table = csv::Reader::from_path(path).unwrap();
    let mut columns :Vec<String> = Vec::new();
    for colum in value_table.headers().unwrap().iter() {
        columns.push(colum.to_string());
    }
    let mut values: Vec<Vec<String>> = Vec::new();
    for value_vector in value_table.records() {
        let mut tmp_value_vector:Vec<String> = Vec::new();
        for value in value_vector.unwrap().iter() {
            tmp_value_vector.push(value.to_string());
        }
        values.push(tmp_value_vector);
    }
    DataFrame{
        colum: columns,
        value: values
    }
}

fn print_data_frame(data: &DataFrame) {
    for colum in &data.colum {
        print!("{},",colum);
    }
    println!("");
    for value_vector in &data.value {
        for value in value_vector {
            print!("{},",value);
        }
        println!("");
    }
}

/*
//ここからテストです
*/
fn df_test(x: &DataFrame,y: &DataFrame) {
    assert_eq!(x.colum,y.colum);
    assert_eq!(x.value,y.value);
}
#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn shape_work() {
        let size = read_csv("test_csv/test_data.csv").shape();
        let ans  = (6, 150);
        assert_eq!(size,ans);
    }

    #[test]
    fn head_work() {
        let ans  = read_csv("test_csv/test_data.csv").head(4);
        let head = read_csv("test_csv/head_test.csv");
        df_test(&ans,&head);
    }
    
    #[test]
    fn selected_colum_work() {
        let ans  = read_csv("test_csv/test_data.csv").col_select("sepal_length");
        let test = read_csv("test_csv/selected_col_test.csv").col_select("sepal_length"); 
        
        df_test(&ans,&test);
    }
    
}

