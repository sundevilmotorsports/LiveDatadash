use serialport::SerialPortInfo;


pub fn get_ports() -> Vec<SerialPortInfo> {
    serialport::available_ports().expect("No ports found!")
}

// pub fn clone_thread(){
  
// }

// fn decode(buf : Vec<u8>) -> Vec<i32>{
//     let mut data: Vec<i32> = vec![];
  
//     //tells program to increment index if i is a comma or space
//     let mut new_index = true;
//     let mut index = 0;
     
//     for i in buf{
//       if i as char == '\n' {
//         println!("final vec {:?}", data); 
//         return data;
//       } else if i as char == ' '{
//         continue;
//       } else if i as char == ','{
//         new_index = true;
//         index += 1;
//       }else if i.is_ascii_digit() {
//         if new_index{
//           //If previous character was a comma, this is a new number and needs to be pushed
//           data.push((i - 48).into());
//           new_index = false;
//         } else {
//           //Increases previous number by a factor of ten and adds current value to it
//           data[index] = data[index] * 10 + <u8 as Into<i32>>::into(i - 48);
//         }
  
//       }
//     }
//     return data;
//   }