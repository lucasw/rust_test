/*
 * Encode a struct of floats/ints of different lengths, also a string and turn it into a vector of
 * bytes, then send that to be deserialized- maybe actually send it over the network?
 *
 */

use std::convert::TryInto;

#[derive(Debug)]
struct DataStuff {
    val1: f64,
    val2: f32,
    val3: i64,
    val4: u8,
    text: String,
    val5: u32,
}

impl DataStuff {

    fn to_bytes(&self) -> Vec<u8> {
        let mut data_bytes: Vec<u8> = Vec::new();

        println!("{:?}", data_bytes);

        println!("{:?}", self.val1.to_be_bytes());
        for byte in self.val1.to_be_bytes().iter() {
            data_bytes.push(*byte);
        }
        for byte in self.val2.to_be_bytes().iter() {
            data_bytes.push(*byte);
        }
        for byte in self.val3.to_be_bytes().iter() {
            data_bytes.push(*byte);
        }
        for byte in self.val4.to_be_bytes().iter() {
            data_bytes.push(*byte);
        }

        // let string_bytes = self.text.clone().into_bytes();
        // let text_len: usize = string_bytes.len();
        let text_len = self.text.as_bytes().len();
        print!("text length {}, {}..", text_len, data_bytes.len());
        for byte in text_len.to_be_bytes().iter() {
            data_bytes.push(*byte);
        }
        println!("{}", data_bytes.len());
        for byte in self.text.as_bytes().iter() {
            data_bytes.push(*byte);
        }
        // for byte in string_bytes.iter() {
        //    data_bytes.push(*byte);
        // }

        for byte in self.val5.to_be_bytes().iter() {
            data_bytes.push(*byte);
        }
        println!("encoded {} bytes", data_bytes.len());

        data_bytes
    }

    fn from_bytes(data_bytes: Vec<u8>) -> DataStuff {
        println!("decoding {} bytes", data_bytes.len());
        let mut ind1 = 0;
        let mut ind2 = 0;

        let mut val1: f64 = 0.0;
        ind2 += val1.to_be_bytes().len();
        val1 = f64::from_be_bytes(data_bytes[ind1..ind2].to_vec().try_into().unwrap());
        println!("val1 {}", val1);
        ind1 = ind2;

        let mut val2: f32 = 0.0;
        ind2 += val2.to_be_bytes().len();
        val2 = f32::from_be_bytes(data_bytes[ind1..ind2].to_vec().try_into().unwrap());
        println!("val2 {}", val2);
        ind1 = ind2;

        let mut val3: i64 = 0;
        ind2 += val3.to_be_bytes().len();
        val3 = i64::from_be_bytes(data_bytes[ind1..ind2].to_vec().try_into().unwrap());
        println!("val3 {}", val3);
        ind1 = ind2;

        let mut val4: u8 = 0;
        ind2 += val4.to_be_bytes().len();
        val4 = u8::from_be_bytes(data_bytes[ind1..ind2].to_vec().try_into().unwrap());
        println!("val4 {}, {}..{}", val4, ind1, ind2);
        ind1 = ind2;

        let mut text_len: usize = 0;
        ind2 += text_len.to_be_bytes().len();
        text_len = usize::from_be_bytes(data_bytes[ind1..ind2].to_vec().try_into().unwrap());
        println!("text length {}, {}..{}", text_len, ind1, ind2);
        ind1 = ind2;

        let mut text: String;
        ind2 += text_len;
        let text_vec = data_bytes[ind1..ind2].to_vec().try_into().unwrap();
        text = String::from_utf8(text_vec).unwrap();
        ind1 = ind2;

        let mut val5: u32 = 0;
        ind2 += val5.to_be_bytes().len();
        val5 = u32::from_be_bytes(data_bytes[ind1..ind2].to_vec().try_into().unwrap());

        let data = DataStuff {
            val1,
            val2,
            val3,
            val4,
            text,
            val5,
        };

        data
    }

}

fn main() {
    let data0 = DataStuff{
        val1: 234.24213,
        val2: 0.001419,
        val3: -9432235,
        val4: 201,
        text: String::from("test message blah blah"),
        val5: 24323,
    };

    println!("{:?}", data0);
    let data_bytes = data0.to_bytes();
    println!("{:?}", data_bytes);

    let data1 = DataStuff::from_bytes(data_bytes);
    println!("{:?}", data1);
}
